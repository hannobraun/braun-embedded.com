#![no_main]
#![no_std]

use core::marker::PhantomData;

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
        peripheral: Option<Peripheral<Disabled>>,
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
            peripheral: Some(peripheral),
        }
    }

    #[task(resources = [peripheral])]
    fn task(cx: task::Context) {
        // `take` moves the peripheral out of `&mut Option<Peripheral>`, leaving
        // `None` in its place. Since there could have been a `None` in the
        // first place, `take` returns an `Option<Peripheral>` which we `unwrap`
        // here.
        let mut peripheral = cx.resources.peripheral.take().unwrap().enable();
        peripheral.use_peripheral();
        *cx.resources.peripheral = Some(peripheral.disable());
    }

    extern "C" {
        fn USART0();
    }
};
