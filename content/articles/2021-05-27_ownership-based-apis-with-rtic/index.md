+++
title = "Using ownership-based APIs with RTIC"
date  = "2021-05-27"
slug  = "ownership-based-apis-with-rtic"

[extra]
subtitle = """
Many APIs in the Embedded Rust ecosystem utilize Rust's ownership system to encode state at compile time, consuming an old instance of a type and returning a new one when changing state. How can such an API be used with RTIC, when RTIC only gives us mutable references to resources?
"""
+++

Let's say we have a peripheral API that encodes in its type whether it is enabled or not.

``` rust
struct Peripheral<State> {
    // ...
}

impl Peripheral<Disabled> {
    pub fn enable(self) -> Peripheral<Enabled> {
        // ...
    }
}

impl Peripheral<Enabled> {
    pub fn use_peripheral(&mut self) {
        // ...
    }

    pub fn disable(self) -> Peripheral<Disabled> {
        // ...
    }
}

// `Enabled` and `Disabled` are just types that we use as markers. They could be
// defined like this, for example:
pub struct Disabled;
pub struct Enabled;
```

This pattern is often called "type state". The `use_peripheral` method is only available, if the peripheral is enabled, and this is checked at compile-time. This pattern can be very useful, to prevent accidental misuse of the API.

Another example of using ownership in a similar way are DMA APIs, which often involve a `Transfer` type that consumes the peripheral, a buffer, and whatever else it needs to do its job, giving all of it back once the DMA transfer has finished.

If we use this API in an RTIC task, it could look like this (using RTIC 0.5.x):

``` rust
// Most of the RTIC application omitted for brevity.

struct Resources {
    peripheral: Peripheral<Enabled>,
}

#[init]
fn init(cx: init::Context) -> init::LateResources {
    // Let's say whatever HAL we're using is giving us the peripheral in the
    // `Disabled` state.
    let peripheral: Peripheral<Disabled> = cx.device.peripheral;

    init::LateResources {
        peripheral: peripheral.enable(),
    }
}

#[task(resources = [peripheral])]
fn task(cx: task::Context) {
    // `use_peripheral` requires `&mut self` and we got a `&mut Peripheral`
    // here. Everything's great!
    cx.resources.peripheral.use_peripheral();
}
```

All is well: the peripheral is enabled on initialization, and then used during runtime from the task. A more complete version of this example is {{ ext_link(link="https://github.com/braun-embedded/braun-embedded.com/tree/main/rust/example-rtic-type-state-init", text="available on GitHub") }}.

But what if we only want to enable the peripheral while it is used in the task?

``` rust
struct Resources {
    peripheral: Peripheral<Disabled>,
}

#[task(resources = [peripheral])]
fn task(cx: task::Context) {
    // This won't compile! `enable` and `disable` take `self`, meaning they
    // consume a whole `Peripheral`. We only have a `&mut Peripheral` here.
    cx.resources.peripheral.enable();
    cx.resources.peripheral.use_peripheral();
    cx.resources.peripheral.disable();
}
```

We need to move the peripheral out of the resources somehow, but doing this is not straight-forward.

``` rust
#[task(resources = [peripheral])]
fn task(cx: task::Context) {
    // This won't compile either! We can't just move out of a `&mut Peripheral`,
    // as we can't leave the memory it references unoccupied.
    let peripheral = cx.resources.peripheral.enable();
    peripheral.use_peripheral();
    cx.resources.peripheral = peripheral.disable();
}
```

So what to do? Fortunately, there are a few solutions.


### Solution 1: `Option<Peripheral>`

We might not be able to move out of a `&mut Peripheral`, but we certainly can move out of an `Option<Peripheral>` using {{ ext_link(link="https://doc.rust-lang.org/core/option/enum.Option.html#method.take", text="`Option::take`") }}.

``` rust
struct Resources {
    peripheral: Option<Peripheral<Disabled>>,
}

#[task(resources = [peripheral])]
fn task(cx: task::Context) {
    // `take` moves the peripheral out of `&mut Option<Peripheral>`,
    // leaving `None` in its place. Since there could have been a `None` in
    // the first place, `take` returns an `Option<Peripheral>` which we
    // `unwrap` here.
    let mut peripheral = cx.resources.peripheral.take().unwrap().enable();
    peripheral.use_peripheral();
    *cx.resources.peripheral = Some(peripheral.disable());
}
```

As long as we make sure we start out with a `Some` and are careful to always put the peripheral back after using it, `unwrap` will never panic. Check out the more complete {{ ext_link(link="https://github.com/braun-embedded/braun-embedded.com/tree/main/rust/example-rtic-type-state-option", text="example on GitHub") }}.

This solution is usually fine, but the additional `Option<...>` is a bit ugly. And it's not zero-overhead, as we unnecessarily keep track of whether there currently is a `Some` or a `None` in the resource.


### Solution 2: replace_with

The previous solution worked around the limitation that we can't move out of a mutable reference. But is that really true? There surely is no harm done, if we make sure to move something of the same type back, before anyone else has a chance to notice, right?

That's indeed right, and there's even a function in the core library to do it: {{ ext_link(link="https://doc.rust-lang.org/core/mem/fn.replace.html", text="`mem::replace`") }}. That doesn't really help us though, as we'd need a replacement right away, and we won't have one until after we used and re-disabled the peripheral.

We could cook something up using `unsafe`, but fortunately there's already a library that does just that: {{ ext_link(link="https://crates.io/crates/replace_with", text="replace_with") }}

Using replace_with, we can replicate what we did in solution 1, but without the additional overhead of the `Option`.

``` rust
struct Resources {
    peripheral: Peripheral<Disabled>,
}

#[task(resources = [peripheral])]
fn task(cx: task::Context) {
    // Here we move the peripheral out of the mutable reference and put it
    // back after using it.
    replace_with(
        cx.resources.peripheral,
        || {
            // The closure below uses the peripheral. If there's a panic
            // while we do this, `replace_with` can't put a `Peripheral`
            // back, and calls this closure to get a default instance of
            // `Peripheral<Disabled>`.
            //
            // We'll just panic here instead. Please note that this can
            // result in a double-panic, which could be problematic, and the
            // `replace_with` documentation explicitly recommends not to
            // panic here.
            //
            // In a real application, you might want to do something better,
            // like logging an error and restarting the application in a
            // controlled manner.
            panic!("Panic while using peripheral")
        },
        |peripheral| {
            let mut peripheral = peripheral.enable();
            peripheral.use_peripheral();
            peripheral.disable()
        },
    );
}
```

This is a nice and elegant solution to the problem, but it requires an external dependency. There's also the danger of a panic causing a hard-to-debug problem, but that's always the case with embedded code.

There's a more complete example {{ ext_link(link="https://github.com/braun-embedded/braun-embedded.com/tree/main/rust/example-rtic-type-state-replace-with", text="available on GitHub") }}.



### Going beyond

The previous solutions work fine, if all we need is to keep the peripheral enabled within a single task. But what if we need to enable the peripheral based on some external circumstance, keep it enabled for a while, then disable it later? This would require the peripheral to be enabled, disabled, and used in different tasks.

This is possible by combining the previous two solutions: Using an enum to track the peripheral state at runtime (except, instead of an option, we use a custom enum with `Enabled`/`Disabled` variants) and use replace_with for the state transitions.

That's a topic for a different article though. Please {{ email(text="let me know") }} if you want to read that, so I can prioritize writing it accordingly.
