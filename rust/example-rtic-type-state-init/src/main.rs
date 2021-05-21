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
        peripheral: Peripheral<Enabled>,
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
            peripheral: peripheral.enable(),
        }
    }

    #[task(resources = [peripheral])]
    fn task(cx: task::Context) {
        // `use_peripheral` requires `&mut self` and we got a `&mut Peripheral`
        // here. Everything's great!
        cx.resources.peripheral.use_peripheral();
    }

    extern "C" {
        fn USART0();
    }
};
