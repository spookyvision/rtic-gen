#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_semihosting::{debug, hprintln};
use panic_halt as _;
use stm32f1xx_hal as hal;

// NOTE: does NOT properly work on QEMU
#[rtic::app(device = stm32f103, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        // A resource
        #[init(0)]
        shared: u32,
    }

    #[init]
    fn init(_: init::Context) {
        // omitted: initialization of `CYCCNT`
        rtic::pend(Interrupt::USART1);
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        //debug::exit(debug::EXIT_SUCCESS);

        loop {}
    }

    #[task(binds = USART1, resources = [shared])]
    fn usart1(cx: usart1::Context) {
        let shared: &mut u32 = cx.resources.shared;
        *shared += 1;

        hprintln!("UART0: shared = {}", shared).unwrap();
    }
};
