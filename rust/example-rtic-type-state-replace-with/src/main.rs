#![no_main]
#![no_std]

use core::marker::PhantomData;

use replace_with::replace_with;

use panic_halt as _;

pub struct Peripheral<State> {
    _state: PhantomData<State>
}

impl Peripheral<Disabled> {
    pub fn enable(self) -> Peripheral<Enabled> {
        Peripheral {
            _state: PhantomData,
        }
    }
}

impl Peripheral<Enabled> {
    pub fn use_peripheral(&mut self) {
        // ...
    }

    pub fn disable(self) -> Peripheral<Disabled> {
        Peripheral {
            _state: PhantomData,
        }
    }
}

pub struct Enabled;
pub struct Disabled;

#[rtic::app(device = lpc8xx_hal::pac)]
const APP: () = {
    struct Resources {
        peripheral: Peripheral<Disabled>,
    }

    #[init]
    fn init(_: init::Context) -> init::LateResources {
        // In a real scenario, we'd get an instance of the peripheral from the
        // HAL via the `init::Context` argument. But since this is just an
        // example, let's just create an instance of our example peripheral
        // here.
        let peripheral: Peripheral<Disabled> = Peripheral {
            _state: PhantomData
        };

        init::LateResources {
            peripheral,
        }
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
                // `replace_with` documentation explicitly recommends not
                // panicking here.
                //
                // In a real application, you might want to do something better
                // here, like logging an error and restarting the application in
                // a controlled manner.
                panic!("Panic while using peripheral")
            },
            |peripheral| {
                let mut peripheral = peripheral.enable();
                peripheral.use_peripheral();
                peripheral.disable()
            },
        );
    }

    extern "C" {
        fn USART0();
    }
};
