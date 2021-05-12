+++
title    = "DW1000 Driver in Rust"
template = "article.html"
date     = "2020-03-31"

[extra]
subtitle = """
The DW1000 is a radio transceiver chip that is capable of IEEE 802.15.4 communication and position tracking. This article introduces the chip and its Rust driver.
"""
+++

### Introduction

The [Decawave DW1000 Radio IC][DW1000] is an ultra-wideband wireless transceiver chip, supporting wireless communication based on the [IEEE 802.15.4] standard. It is low-cost, low-power, has a high data rate, and, according to the manufacturer, a range of up to 290 meters.

But arguably the main feature is its ability to measure the distance between two DW1000 nodes, by precisely measuring the in-flight time of the wireless packages sent between them. This makes it possible to build highly accurate (within 10 cm, according to Decawave) position tracking solutions based on this chip.

I worked on building Rust support for the DW1000 and some related hardware (namely the [DWM1001 Module] and [DWM1001 Development Board]), which resulted in the release of the [`dw1000`] and [`dwm1001`] crates, as well as the more broadly applicable [`ieee802154`] and [`embedded-timeout-macros`] crates.

The initial work, starting in 2018, was sponsored by [Ferrous Systems]. Since then, I've taken care of the ongoing maintenance myself, with lots of help from [James Munns] and other members of the community.

The goal of this project was to create a solid foundation for everyone who wants to build something with the DW1000 using the [Rust programming language][Rust]. While the libraries are not complete and will likely see more development going forward, I believe I succeeded in that goal.


### DW1000, DWM1001, DWM1001-Dev

Before we proceed to take a look at the `dw1000`/`dwm1001` crates, let's get a good overview of the hardware first.

Let's start with the [DW1000]. As noted above, it's a radio chip and the core of the whole thing. It's not very useful by itself, but if you want to create your own board and have full control over the design, the DW1000 is what you'd use.

The [DWM1001 Module] contains the DW1000, a [Nordic nRF52832] microcontroller (which can communicate via Bluetooth), a 3-axis accelerometer, and other things like antennas. Again, this is not very useful by itself, but you can use it as part of your design.

Finally, the [DWM1001 Development Board] is what you'd use for development, playing around, or maybe as a complete solution for your own product. It contains a DWM1001 Module, plus everything needed to make it accessible: An on-board programmer, pins that you can solder standard pin headers onto, LEDs, buttons, and more.

If you're interested in the DW1000 but are not sure what to get, I recommend you buy a handful of DWM1001 Development Boards, as that's the easiest way to get started.


### The `dw1000` Driver

Now that we know about the hardware, let's start talking about the software. Let's start with the [`dw1000`] driver crate.

In the embedded world, the word "driver" is a bit overloaded, but in our embedded Rust corner, it usually means something specific: A crate (Rust parlance for "library") that interfaces with an external sensor or actuator (in this case the DW1000).

Drivers usually depend on the [`embedded-hal`] API to abstract over the hardware that they run on. This means the driver is completely portable, as long as there is an implementation of `embedded-hal` for the target hardware, which is often the case.

Let's take look at the driver itself. As of this writing, it consists of 4 modules:

- The [`ll`](https://docs.rs/dw1000/0.4.0/dw1000/ll/index.html) module provides a low-level, register-based interface. It is not very convenient to use, but it serves as the base for the higher-level interface that the driver also provides, as well as a fallback, in case the higher-level interface doesn't cover your use case. The low-level interface isn't complete yet, but adding more registers is quite easy.
- The [`hl`](https://docs.rs/dw1000/0.4.0/dw1000/hl/index.html) provides a high-level interface. It is the recommended way of using the driver, however, it does not nearly support all the features of the chip.
- The [`ranging`](https://docs.rs/dw1000/0.4.0/dw1000/ranging/index.html) module provides an implementation of a ranging algorithm on top of the high-level interface. It can be used by an application to measure the distance between nodes.
- The [`time`](https://docs.rs/dw1000/0.4.0/dw1000/time/index.html) modules contains some types to deal with the DW1000's system time.

Here's a piece of code that uses the high-level API to listen for messages and returning them to the sender:

``` rust
loop {
    // Start receiving a message
    let mut receiving = dw1000.receive(RxConfig::default())?;

    // Wait for the message to be received
    let mut buffer = [0; 1024];
    let message = block!(receiving.wait(&mut buffer))?;

    // Receiving data put `dw1000` into the receiving state, moving out of our
    // original variable. Now that we're done, return it to the "ready" state
    // and move it back.
    dw1000 = receiving.finish_receiving()
        .map_err(|(_, err)| err)?;

    // Send the message back to where it came from
    let mut sending = dw1000.send(
        message.frame.payload,
        message.frame.header.source,
        None,
    )?;

    // Wait for the message to be sent
    block!(sending.wait())?;

    // Get `dw1000` ready to receive again in the next loop iteration.
    dw1000 = sending.finish_sending()
        .map_err(|(_, err)| err)?;
}
```

There are many more examples [available in the repository][examples].


### The `dwm1001` Board Support Crate

The [`dwm1001`] crate is designed to support applications that use the DWM1001 module or development board. This kind of library is often called a board support package (BSP), or as Rust people like to call it, a board support crate (BSC).

Like the DWM1001 boards contain various pieces of hardware (like the DW1000, the nRF52832, and more), the `dwm1001` crate contains the various libraries required to access that hardware (like `dw1000`, `nrf52832-hal`, and more). The configuration of the software reflects the design of the hardware: Since on the DWM1001 module, the nRF52832 is hooked up to the DW1000 via SPI, the `dwm1001` crate hooks up the `nrf52832-hal` and `dw1000` crates in the same way.

The design of `dwm1001` is rather simple: The [`DWM1001` struct] provides access to all the hardware. Here's a piece of code using that:

``` rust
// Get an instance of the `DWM1001` struct by calling its `take` method. Since
// we know we're only doing this once in our program, we can safely `unwrap`
// and be sure this will never panic.
let mut dwm1001 = DWM1001::take().unwrap();

// Reset the DW1000, to make sure it's in a known state.
dwm1001.DW_RST.reset_dw1000(&mut delay);

// Initialize the DW1000 and store the initialized driver in this local
// variable.
let mut dw1000 = dwm1001.DW1000.init()?;

// Insert code using the DW1000 here.
```

[`DWM1001` struct]: https://docs.rs/dwm1001/0.4.0/dwm1001/struct.DWM1001.html


### An Example: Ranging

As mentioned above, arguably the main feature of the DW1000 is its ability to measure the distance between two nodes with up to 10 cm accuracy. There are various ways to realize that, and the `dw1000` crate implements one such scheme.

For this scheme, the nodes (all of which have a DW1000) are divided into two groups: Tags, whose position we want to determine, and anchors, that are placed in known positions. Anchors send out regular ping messages that tags listen for.

Each ping message contains the timestamp from when it was sent. Once a tag receives a ping, it replies with a ranging request message that includes the following information:

- The same timestamp that the ping included.
- A timestamp from when the ranging request was sent.
- The time that passed between the ping being received and the ranging request being sent.

Once the anchor receives the ranging request, it has all the information it needs to compute the round-trip-time for the ping (the time from the ping being sent to the ranging request being received). It then sends a ranging reply, which includes the following information:

- The ping round-trip-time it just computed.
- The time it took the tag to reply to the ping (received with the ranging request).
- The timestamp from when the ranging request was sent (received with the ranging request).
- The time passed between the ranging request being received and the ranging reply being sent.

Finally, the tag receives the ranging reply and can now calculate the round-trip-time of the ranging request (the time from the ranging request being sent to the ranging reply being received).

The tag now has all the information it needs to calculate the distance:

- The round-trip-time of the ping (`Tround1`).
- The rount-trip-time of the ranging request (`Tround2`).
- The time passed between the ping being received and the ranging request being sent (`Treply1`).
- The time passed between the ranging request being received and the ranging response being sent (`Treply2`).

From this, the estimated time-of-flight (`Tprop`) can be calculated according to the following formula:
```
Tprop = (Tround1 * Tround2 - Treply1 * Treply2) / (Tround1 + Tround2 + Treply1 + Treply2)
```

Since we know the speed of light (299,792,458 m/s), we can calculate the distance from this.

Some notes:

- As mentioned, there's more than one way to do it. This one is called double-sided two-way ranging using three messages. See DW1000 user manual, section 12.3.2.
- Please note that no clock synchronization is required. Yes, nodes send local timestamps to other nodes, but this is only done to make the protocol stateless, thereby keeping the implementation simple. Each node only makes calculations based on its own timestamps, and time intervals measured by other nodes.
- This alone will not get you fully accurate measurements. You also need to apply a range bias to the distance measurement. Support for this doesn't exist yet in the driver, see ([#105](https://github.com/braun-embedded/rust-dw1000/issues/105)).


### Conclusion

In this article, I introduced the DW1000 chip, some of the hardware ecosystem around it, as well as the Rust libraries that can be used to access it. I've also presented a high-level overview of a ranging algorithm that can be used to measure the distance between two nodes.

If you're interested in using the Rust libraries presented here, the following resources might be useful to you:

- If you want to learn more about Rust and how it can be used for embedded development, the official documentation ([general](https://www.rust-lang.org/learn), [embedded-specific](https://docs.rust-embedded.org/)) is a great place to start.
- As already mentioned, there are [examples] in the repository that demonstrate basic usage.
- If you're looking for more usage examples, the [Internet of Streams] project might be of interest to you. It consists of a code repository, as well as a number of live streams.

If you'd like to use the `dw1000` driver (or any of the other libraries) but it's lacking some feature you need, don't worry, everything's open source! Feel free to open a pull request with your changes in the [repository]. Or, if you don't have the time to do that, you can {{ email_address(text="hire me") }} to do it for you.

Do you have any questions or comments about this article? Please {{ email_address(text="contact me") }} and let me know!


[Rust]: https://www.rust-lang.org/
[DW1000]: https://www.decawave.com/product/dw1000-radio-ic/
[IEEE 802.15.4]: https://en.wikipedia.org/wiki/IEEE_802.15.4
[DWM1001 Module]: https://www.decawave.com/product/dwm1001-module/
[DWM1001 Development Board]: https://www.decawave.com/product/dwm1001-development-board/
[Nordic nRF52832]: https://www.nordicsemi.com/Products/Low-power-short-range-wireless/nRF52832

[`dw1000`]: https://crates.io/crates/dw1000
[`dwm1001`]: https://crates.io/crates/dwm1001
[`ieee802154`]: https://crates.io/crates/ieee802154
[`embedded-timeout-macros`]: https://crates.io/crates/embedded-timeout-macros
[`embedded-hal`]: https://crates.io/crates/embedded-hal

[examples]: https://github.com/braun-embedded/rust-dw1000/tree/master/dwm1001/examples
[Internet of Streams]: https://github.com/ferrous-systems/internet-of-streams
[repository]: https://github.com/braun-embedded/rust-dw1000

[Ferrous Systems]: https://ferrous-systems.com/
[James Munns]: https://jamesmunns.com/
