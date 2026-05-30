#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use rmk::types::action::KeyAction;
use rmk::types::modifier::ModifierCombination;
use rmk::types::morse::{MorseMode, MorseProfile};
use rmk::{a, k, lt, mtp, tg};

pub(crate) const COL: usize = 6;
pub(crate) const ROW: usize = 8;
pub(crate) const NUM_LAYER: usize = 3;

type Layer = [[KeyAction; COL]; ROW];

const na: KeyAction = a!(Transparent);
const a: KeyAction = k!(A);

const HRM: MorseProfile = MorseProfile::new(
    Some(false),
    Some(MorseMode::HoldOnOtherPress),
    Some(200),
    Some(200),
);
const SUPER: ModifierCombination = ModifierCombination::LGUI;
const CTRL: ModifierCombination = ModifierCombination::LCTRL;
const ALT: ModifierCombination = ModifierCombination::LALT;
const SHIFT: ModifierCombination = ModifierCombination::LSHIFT;

// lt - layer hold
// tg - layer toggle
// mtp - mod on hold (mainly for hrm)
#[rustfmt::skip]
const BASE: Layer = layout!(
    k!(Tab),    k!(B), k!(L), k!(D), k!(C), k!(V),  k!(J), k!(Y), k!(O), k!(U), k!(Comma), k!(Backspace),
    k!(Escape),  mtp!(N, SUPER, HRM), mtp!(R, ALT, HRM), mtp!(T, CTRL, HRM), mtp!(S, SHIFT, HRM), k!(G),  k!(P), mtp!(H, SHIFT, HRM), mtp!(A, CTRL, HRM), mtp!(E, ALT, HRM), mtp!(I, SUPER, HRM), mtp!(Semicolon, ALT, HRM),
    k!(LShift), k!(X), k!(Q), k!(M), k!(W), k!(Z),  k!(K), k!(F), k!(Quote), k!(Slash), k!(Dot),   tg!(2),
           lt!(1, LGui), k!(Space), k!(Enter),  k!(Backspace), k!(Delete), lt!(1, LGui),
);

#[rustfmt::skip]
const NAV: Layer = layout!(
    na, k!(PageUp), k!(Home), k!(Up), k!(End), na,        na, k!(Home), k!(Up), k!(End), k!(PageUp), na,
    na, k!(PageDown), k!(Left), k!(Down), k!(Right), na,  na, k!(Left), k!(Down), k!(Right), k!(PageDown), na,
    na, na, na, na, na, na,                               na, na, na, na, na, na,
                na, na, na,                               na, na, na,
);

#[rustfmt::skip]
const EMERGENCY: Layer = layout!(
    na, k!(Q), k!(W), k!(E), k!(R), k!(T),  k!(Y), k!(U), k!(I), k!(O), k!(P), na,
    na, k!(A), k!(S), k!(D), k!(F), k!(G),  k!(H), k!(J), k!(K), k!(L), na, na,
    na, k!(Z), k!(X), k!(C), k!(V), k!(B),  k!(N), k!(M), na, na, na, na,
                               na, na, na,  na, na, na,
);

pub fn get_default_keymap() -> [Layer; NUM_LAYER] {
    [BASE, NAV, EMERGENCY]
}
