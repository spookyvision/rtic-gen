#![deny(unsafe_code)]
#![no_main]
#![no_std]

// NOTE: does NOT properly work on QEMU
#[rtic::app(device = stm32f103)]
mod app {
    use stm32f1xx_hal as hal;

    #[resources]
    struct Resources {
        // A resource
        #[init(0)]
        shared: u32,
    }

    #[init]
    fn init(_: init::Context) -> (init::LateResources, init::Monotonics) {
        // omitted: initialization of `CYCCNT`
        rtic::pend(Interrupt::USART1);
        (init::LateResources {}, init::Monotonics())
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        //foo::schedule(/* ... */).unwrap();

        loop {}
        // exit();
    }

    #[task(binds = USART1, resources = [shared])]
    fn usart1(mut cx: usart1::Context) {
        cx.resources.shared.lock(|shared| {
            *shared += 1;
            defmt::debug!("USART1: shared = {}", shared);
        });
    }

    use core::sync::atomic::{AtomicUsize, Ordering};

    use defmt_rtt as _; // global logger
    use panic_probe as _;

    // same panicking *behavior* as `panic-probe` but doesn't print a panic message
    // this prevents the panic message being printed *twice* when `defmt::panic` is invoked
    #[defmt::panic_handler]
    fn panic() -> ! {
        cortex_m::asm::udf()
    }

    static COUNT: AtomicUsize = AtomicUsize::new(0);
    defmt::timestamp!("{=usize}", {
        // NOTE(no-CAS) `timestamps` runs with interrupts disabled
        let n = COUNT.load(Ordering::Relaxed);
        COUNT.store(n + 1, Ordering::Relaxed);
        n
    });

    /// Terminates the application and makes `probe-run` exit with exit-code = 0
    pub fn exit() -> ! {
        loop {
            cortex_m::asm::bkpt();
        }
    }
}
