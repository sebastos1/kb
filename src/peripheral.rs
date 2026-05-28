#![no_main]
#![no_std]

#[macro_use]
mod macros;
mod light;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    gpio::{Level, Output},
    peripherals::UART0,
    uart::{self, BufferedUart},
};
use panic_probe as _;
use rmk::{
    debounce::default_debouncer::DefaultDebouncer,
    futures::future::join,
    matrix::direct_pin::DirectPinMatrix,
    run_all,
    split::{SPLIT_MESSAGE_MAX_SIZE, peripheral::run_rmk_split_peripheral},
    watchdog::Rp2040Watchdog,
};
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    UART0_IRQ => uart::BufferedInterruptHandler<UART0>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    spawner.spawn(light::led(Output::new(peripherals.PIN_25, Level::Low)).unwrap());

    #[rustfmt::skip]
    let direct_pins = config_matrix_pins_rp! {
        peripherals: peripherals,
        direct_pins: [
            [PIN_22, PIN_21, PIN_2, PIN_5, PIN_8, PIN_11],
            [PIN_20, PIN_19, PIN_3, PIN_6, PIN_9, PIN_12],
            [PIN_18, PIN_17, PIN_4, PIN_7, PIN_10, PIN_13],
            [PIN_15, PIN_14, PIN_16, _, _, _ ]
        ]
    };

    static TX_BUF: StaticCell<[u8; SPLIT_MESSAGE_MAX_SIZE]> = StaticCell::new();
    static RX_BUF: StaticCell<[u8; SPLIT_MESSAGE_MAX_SIZE]> = StaticCell::new();
    let uart0 = BufferedUart::new(
        peripherals.UART0,
        peripherals.PIN_0,
        peripherals.PIN_1,
        Irqs,
        &mut TX_BUF.init([0; SPLIT_MESSAGE_MAX_SIZE])[..],
        &mut RX_BUF.init([0; SPLIT_MESSAGE_MAX_SIZE])[..],
        uart::Config::default(),
    );

    let debouncer = DefaultDebouncer::new();
    let mut matrix = DirectPinMatrix::<_, _, 4, 6, 24>::new(direct_pins, debouncer, true);
    let mut watchdog =
        Rp2040Watchdog::default_runner(embassy_rp::watchdog::Watchdog::new(peripherals.WATCHDOG));

    join(run_all!(matrix, watchdog), run_rmk_split_peripheral(uart0)).await;
}
