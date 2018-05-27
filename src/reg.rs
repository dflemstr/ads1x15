//! Definitions for ADS1x15 I2C registers.
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(missing_debug_implementations)]
#![allow(missing_copy_implementations)]

use std::mem;

use bitfield_register;
use bitfield_register_macro::register;

/// I2C registers present in an ADS1x15.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Register {
    /// The `Convert` register.
    Convert = 0x00,
    /// The `Config` register.
    Config = 0x01,
    /// The `Lowthresh` register.
    Lowthresh = 0x02,
    /// The `Hithresh` register.
    Hithresh = 0x03,
}

/// Valid values for the `Config` register.
#[register()]
pub struct Config {
    #[bitfield(at = 15)]
    os: ConfigOs,
    #[bitfield(from = 12, to = 14)]
    mux: ConfigMux,
    #[bitfield(from = 9, to = 11)]
    pga: ConfigPga,
    #[bitfield(at = 8)]
    mode: ConfigMode,
    #[bitfield(from = 5, to = 7)]
    dr: ConfigDr,
    #[bitfield(at = 4)]
    cmode: ConfigCmode,
    #[bitfield(at = 3)]
    cpol: ConfigCpol,
    #[bitfield(at = 2)]
    clat: ConfigClat,
    #[bitfield(from = 0, to = 1)]
    cque: ConfigCque,
}

impl Into<u16> for Config {
    fn into(self) -> u16 {
        (self.0[0] as u16) << 8 | self.0[1] as u16
    }
}

impl From<u16> for Config {
    fn from(value: u16) -> Self {
        Config([((value >> 8) & 0xff) as u8, (value & 0xff) as u8])
    }
}

macro_rules! cast_u8_bitfield {
    ($t:ty,forbidden: [$($e:expr),*]) => {
        impl bitfield_register::FromBitfield<[u8; 1]> for $t {
            fn from_bitfield(value: [u8; 1]) -> Self {
                unsafe { mem::transmute(value[0]) }
            }
        }

        impl bitfield_register::IntoBitfield<[u8; 1]> for $t {
            fn into_bitfield(self) -> [u8; 1] {
                [self as u8]
            }
        }
    };
    ($t:ty) => {
        cast_u8_bitfield!($t, forbidden: []);
    };
}

/// Values for the `Os` part of the `Config` register.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConfigOs {
    /// `Os`: Write: Noop.
    Noop = 0b0,
    /// `Os`: Write: Set to start a single-conversion.
    Single = 0b1,
}
cast_u8_bitfield!(ConfigOs);

/// Values for the `Mux` part of the `Config` register.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConfigMux {
    /// `Mux`: Differential P = AIN0, N = AIN1 (default).
    Diff0_1 = 0b000,
    /// `Mux`: Differential P = AIN0, N = AIN3.
    Diff0_3 = 0b001,
    /// `Mux`: Differential P = AIN1, N = AIN3.
    Diff1_3 = 0b010,
    /// `Mux`: Differential P = AIN2, N = AIN3.
    Diff2_3 = 0b011,
    /// `Mux`: Single-ended AIN0.
    Single0 = 0b100,
    /// `Mux`: Single-ended AIN1.
    Single1 = 0b101,
    /// `Mux`: Single-ended AIN2.
    Single2 = 0b110,
    /// `Mux`: Single-ended AIN3.
    Single3 = 0b111,
}
cast_u8_bitfield!(ConfigMux);

/// Values for the `Pga` part of the `Config` register.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConfigPga {
    /// `Pga`: +/-6.144V range = Gain 2/3.
    _6_144V = 0b000,
    /// `Pga`: +/-4.096V range = Gain 1.
    _4_096V = 0b001,
    /// `Pga`: +/-2.048V range = Gain 2 (default).
    _2_048V = 0b010,
    /// `Pga`: +/-1.024V range = Gain 4.
    _1_024V = 0b011,
    /// `Pga`: +/-0.512V range = Gain 8.
    _0_512V = 0b100,
    /// `Pga`: +/-0.256V range = Gain 16.
    _0_256V = 0b101,
    // unused: 0b110,
    // unused: 0b111,
}
cast_u8_bitfield!(ConfigPga);

/// Values for the `Mode` part of the `Config` register.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConfigMode {
    /// `Mode`: Continuous conversion mode.
    Contin = 0b0,
    /// `Mode`: Power-down single-shot mode (default).
    Single = 0b1,
}
cast_u8_bitfield!(ConfigMode);

/// Values for the `Dr` part of the `Config` register.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConfigDr {
    /// `Dr`: 128 samples per second.
    _128SPS = 0b000,
    /// `Dr`: 250 samples per second.
    _250SPS = 0b001,
    /// `Dr`: 490 samples per second.
    _490SPS = 0b010,
    /// `Dr`: 920 samples per second.
    _920SPS = 0b011,
    /// `Dr`: 1600 samples per second (default).
    _1600SPS = 0b100,
    /// `Dr`: 2400 samples per second.
    _2400SPS = 0b101,
    /// `Dr`: 3300 samples per second.
    _3300SPS = 0b110,
    // unused: 0b111,
}
cast_u8_bitfield!(ConfigDr);

/// Values for the `Cmode` part of the `Config` register.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConfigCmode {
    /// `Cmode`: Traditional comparator with hysteresis (default).
    Trad = 0b0,
    /// `Cmode`: Window comparator.
    Window = 0b1,
}
cast_u8_bitfield!(ConfigCmode);

/// Values for the `Cpol` part of the `Config` register.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConfigCpol {
    /// `Cpol`: ALERT/RDY pin is low when active (default).
    Actvlow = 0b0,
    /// `Cpol`: ALERT/RDY pin is high when active.
    Actvhi = 0b1,
}
cast_u8_bitfield!(ConfigCpol);

/// Values for the `Clat` part of the `Config` register.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConfigClat {
    /// `Clat`: Non-latching comparator (default).
    Nonlat = 0b0,
    /// `Clat`: Latching comparator.
    Latch = 0b1,
}
cast_u8_bitfield!(ConfigClat);

/// Values for the `Cque` part of the `Config` register.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConfigCque {
    /// `Cque`: Assert ALERT/RDY after one conversions.
    Conv1 = 0b00,
    /// `Cque`: Assert ALERT/RDY after two conversions.
    Conv2 = 0b01,
    /// `Cque`: Assert ALERT/RDY after four conversions.
    Conv4 = 0b10,
    /// `Cque`: Disable the comparator and put ALERT/RDY in high state (default).
    None = 0b11,
}
cast_u8_bitfield!(ConfigCque);
