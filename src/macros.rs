#![allow(unused_macros)]

macro_rules! config_matrix_pins_rp {
    (peripherals: $p:ident, direct_pins: [$([$($pin:tt),+]),+]) => {
        [$([$( config_matrix_pin_rp!($p, $pin) ),+]),+]
    };
}

macro_rules! config_matrix_pin_rp {
    ($p:ident, _) => {
        None
    };
    ($p:ident, $pin:ident) => {
        Some(::embassy_rp::gpio::Input::new(
            $p.$pin,
            ::embassy_rp::gpio::Pull::Up,
        ))
    };
}

// todo
macro_rules! layout {
    (
        $l00:expr, $l01:expr, $l02:expr, $l03:expr, $l04:expr, $l05:expr,
        $r00:expr, $r01:expr, $r02:expr, $r03:expr, $r04:expr, $r05:expr,
        $l10:expr, $l11:expr, $l12:expr, $l13:expr, $l14:expr, $l15:expr,
        $r10:expr, $r11:expr, $r12:expr, $r13:expr, $r14:expr, $r15:expr,
        $l20:expr, $l21:expr, $l22:expr, $l23:expr, $l24:expr, $l25:expr,
        $r20:expr, $r21:expr, $r22:expr, $r23:expr, $r24:expr, $r25:expr,
        $l30:expr, $l31:expr, $l32:expr,
        $r30:expr, $r31:expr, $r32:expr $(,)?
    ) => {
        [
            [$l00, $l01, $l02, $l03, $l04, $l05],
            [$l10, $l11, $l12, $l13, $l14, $l15],
            [$l20, $l21, $l22, $l23, $l24, $l25],
            [$l30, $l31, $l32, a!(No), a!(No), a!(No)],
            [$r00, $r01, $r02, $r03, $r04, $r05],
            [$r10, $r11, $r12, $r13, $r14, $r15],
            [$r20, $r21, $r22, $r23, $r24, $r25],
            [$r30, $r31, $r32, a!(No), a!(No), a!(No)],
        ]
    };
}
