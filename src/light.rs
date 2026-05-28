use embassy_futures::select::{Either, select};
use embassy_rp::gpio::{Level, Output};
use rmk::event::{LayerChangeEvent, ModifierEvent, SubscribableEvent};

#[embassy_executor::task]
pub async fn led(mut led: Output<'static>) {
    let mut layer_sub = LayerChangeEvent::subscriber();
    let mut mod_sub = ModifierEvent::subscriber();
    let mut layer: u8 = 0;
    let mut mods: u8 = 0;
    loop {
        match select(layer_sub.next_message_pure(), mod_sub.next_message_pure()).await {
            Either::First(e) => layer = e.0,
            Either::Second(e) => mods = e.modifier.into_bits(),
        }
        let on = (layer != 0) ^ (mods != 0);
        led.set_level(if on { Level::High } else { Level::Low });
    }
}
