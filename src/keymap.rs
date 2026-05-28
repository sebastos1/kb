#![allow(dead_code)]

use rmk::types::action::KeyAction;
use rmk::{a, k, lt, tg};

pub(crate) const COL: usize = 6;
pub(crate) const ROW: usize = 8;
pub(crate) const NUM_LAYER: usize = 3;

type Layer = [[KeyAction; COL]; ROW];

#[rustfmt::skip]
const BASE: Layer = layout!(
    k!(Tab),    k!(B), k!(L), k!(D), k!(C), k!(V),  k!(J), k!(Y), k!(O),     k!(U),     k!(Comma), k!(Backspace),
    k!(LCtrl),  k!(N), k!(R), k!(T), k!(S), k!(G),  k!(P), k!(H), k!(A),     k!(E),     k!(I),     k!(Semicolon),
    k!(LShift), k!(X), k!(Q), k!(M), k!(W), k!(Z),  k!(K), k!(F), k!(Quote), k!(Slash), k!(Dot),   tg!(2),
           lt!(1, LGui), k!(Backspace), k!(Space),  k!(Space), k!(Enter), k!(RAlt),
);

#[rustfmt::skip]
const NAV: Layer = layout!(
    a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),  a!(No), k!(Home), k!(Up),   k!(End),   k!(PageUp), a!(No),
    a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),  a!(No), k!(Left), k!(Down), k!(Right), k!(PageDown), a!(No),
    a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),  a!(No), a!(No),   a!(No),   a!(No),    a!(No),   a!(No),
                            a!(No), a!(No), a!(No),  a!(No), a!(No),   a!(No),
);

#[rustfmt::skip]
const EMERGENCY: Layer = layout!(
    a!(No), k!(Q), k!(W), k!(E), k!(R), k!(T),  k!(Y), k!(U), k!(I),  k!(O),  k!(P),  a!(No),
    a!(No), k!(A), k!(S), k!(D), k!(F), k!(G),  k!(H), k!(J), k!(K),  k!(L),  a!(No), a!(No),
    a!(No), k!(Z), k!(X), k!(C), k!(V), k!(B),  k!(N), k!(M), a!(No), a!(No), a!(No), a!(No),
                       a!(No), a!(No), a!(No),  a!(No), a!(No), a!(No),
);

pub fn get_default_keymap() -> [Layer; NUM_LAYER] {
    [BASE, NAV, EMERGENCY]
}
