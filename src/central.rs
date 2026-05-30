#![no_main]
#![no_std]

#[macro_use]
mod macros;
mod keymap;
mod light;
mod vial;

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts, dma,
    flash::{Async, Flash},
    gpio::{Level, Output},
    peripherals::{DMA_CH0, UART0, USB},
    uart::{self, BufferedUart},
    usb::{Driver, InterruptHandler},
};
use panic_probe as _;
use rmk::{
    KeymapData,
    config::{
        BehaviorConfig, DeviceConfig, PositionalConfig, RmkConfig, StorageConfig, VialConfig,
    },
    debounce::default_debouncer::DefaultDebouncer,
    futures::future::join,
    host::{HostService, KeyboardContext},
    initialize_keymap_and_storage,
    keyboard::Keyboard,
    matrix::direct_pin::DirectPinMatrix,
    run_all,
    split::{SPLIT_MESSAGE_MAX_SIZE, central::run_peripheral_manager},
    usb::UsbTransport,
    watchdog::Rp2040Watchdog,
};
use static_cell::StaticCell;
use vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};

use keymap::{COL, ROW};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
    UART0_IRQ => uart::BufferedInterruptHandler<UART0>;
    DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>;
});

const FLASH_SIZE: usize = 2 * 1024 * 1024;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("RMK start!");
    let peripherals = embassy_rp::init(Default::default());
    spawner.spawn(light::led(Output::new(peripherals.PIN_25, Level::Low)).unwrap());

    let driver = Driver::new(peripherals.USB, Irqs);

    #[rustfmt::skip]
    let direct_pins = config_matrix_pins_rp! {
        peripherals: peripherals,
        direct_pins: [
            [PIN_5, PIN_4, PIN_11, PIN_15, PIN_3, PIN_2 ],
            [PIN_22, PIN_20, PIN_10, PIN_14, PIN_9, PIN_8 ],
            [PIN_21, PIN_19, PIN_6, PIN_7, PIN_13, PIN_12],
            [PIN_17, PIN_18, PIN_16, _, _, _ ]
        ]
    };

    let flash = Flash::<_, Async, FLASH_SIZE>::new(peripherals.FLASH, peripherals.DMA_CH0, Irqs);

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

    let mut behavior_config = BehaviorConfig::default();
    behavior_config.morse.enable_flow_tap = true;
    behavior_config.morse.prior_idle_time = embassy_time::Duration::from_millis(100);

    let storage_config = StorageConfig::default();
    let per_key_config = PositionalConfig::<ROW, COL>::default();
    let mut keymap_data = KeymapData::new(keymap::get_default_keymap());
    let (keymap, mut storage) = initialize_keymap_and_storage(
        &mut keymap_data,
        flash,
        &storage_config,
        &mut behavior_config,
        &per_key_config,
    )
    .await;

    let rmk_config = RmkConfig {
        device_config: DeviceConfig {
            vid: 0x4c4b,
            pid: 0x4643,
            manufacturer: "RMK",
            product_name: "keh boerd",
            serial_number: "vial:f64c2b3c:000001",
        },
        vial_config: VialConfig::new(VIAL_KEYBOARD_ID, VIAL_KEYBOARD_DEF, &[(0, 0), (0, 5)]),
        ..Default::default()
    };

    let debouncer = DefaultDebouncer::new();
    let mut matrix = DirectPinMatrix::<_, _, 4, COL, 24>::new(direct_pins, debouncer, true);
    let mut keyboard = Keyboard::new(&keymap);
    let host_ctx = KeyboardContext::new(&keymap);
    let mut host_service = HostService::new(&host_ctx, &rmk_config);
    let mut usb_transport = UsbTransport::new(driver, rmk_config.device_config);
    let mut watchdog =
        Rp2040Watchdog::default_runner(embassy_rp::watchdog::Watchdog::new(peripherals.WATCHDOG));

    join(
        run_all!(
            matrix,
            storage,
            usb_transport,
            keyboard,
            host_service,
            watchdog
        ),
        run_peripheral_manager::<4, COL, 4, 0, _>(0, uart0),
    )
    .await;
}
