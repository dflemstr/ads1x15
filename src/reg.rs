//! Definitions for ADS1x15 I2C registers.
#![allow(non_upper_case_globals)]

bitflags! {
    /// I2C registers present in an ADS1x15.
    pub struct Register: u8 {
        /// The `Convert` register.
        const Convert = 0x00;
        /// The `Config` register.
        const Config = 0x01;
        /// The `Lowthresh` register.
        const Lowthresh = 0x02;
        /// The `Hithresh` register.
        const Hithresh = 0x03;
        /// All register bits set.
        const Mask = 0x03;
    }
}

bitflags! {
    /// Valid values of the `Config` register.
    pub struct RegConfig: u16 {
        /// All `Os` bits set.
        const OsMask      = 0b1_000_000_0_000_0_0_0_00;
        /// All `Mux` bits set.
        const MuxMask     = 0b0_111_000_0_000_0_0_0_00;
        /// All `Pga` bits set.
        const PgaMask     = 0b0_000_111_0_000_0_0_0_00;
        /// All `Mode` bits set.
        const ModeMask    = 0b0_000_000_1_000_0_0_0_00;
        /// All `Dr` bits set.
        const DrMask      = 0b0_000_000_0_111_0_0_0_00;
        /// All `Cmode` bits set.
        const CmodeMask   = 0b0_000_000_0_000_1_0_0_00;
        /// All `Cpol` bits set.
        const CpolMask    = 0b0_000_000_0_000_0_1_0_00;
        /// All `Clat` bits set.
        const ClatMask    = 0b0_000_000_0_000_0_0_1_00;
        /// All `Cque` bits set.
        const CqueMask    = 0b0_000_000_0_000_0_0_0_11;

        /// `Os`: Write: Set to start a single-conversion.
        const OsSingle    = 0b1_000_000_0_000_0_0_0_00;
        /// `Os`: Read: Bit = 0 when conversion is in progress.
        const OsBusy      = 0b0_000_000_0_000_0_0_0_00;
        /// `Os`: Read: Bit = 1 when device is not performing a conversion.
        const OsNotbusy   = 0b1_000_000_0_000_0_0_0_00;

        /// `Mux`: Differential P = AIN0, N = AIN1 (default).
        const MuxDiff0_1  = 0b0_000_000_0_000_0_0_0_00;
        /// `Mux`: Differential P = AIN0, N = AIN3.
        const MuxDiff0_3  = 0b0_001_000_0_000_0_0_0_00;
        /// `Mux`: Differential P = AIN1, N = AIN3.
        const MuxDiff1_3  = 0b0_010_000_0_000_0_0_0_00;
        /// `Mux`: Differential P = AIN2, N = AIN3.
        const MuxDiff2_3  = 0b0_011_000_0_000_0_0_0_00;
        /// `Mux`: Single-ended AIN0.
        const MuxSingle0  = 0b0_100_000_0_000_0_0_0_00;
        /// `Mux`: Single-ended AIN1.
        const MuxSingle1  = 0b0_101_000_0_000_0_0_0_00;
        /// `Mux`: Single-ended AIN2.
        const MuxSingle2  = 0b0_110_000_0_000_0_0_0_00;
        /// `Mux`: Single-ended AIN3.
        const MuxSingle3  = 0b0_111_000_0_000_0_0_0_00;

        /// `Pga`: +/-6.144V range = Gain 2/3.
        const Pga_6_144V  = 0b0_000_000_0_000_0_0_0_00;
        /// `Pga`: +/-4.096V range = Gain 1.
        const Pga_4_096V  = 0b0_000_001_0_000_0_0_0_00;
        /// `Pga`: +/-2.048V range = Gain 2 (default).
        const Pga_2_048V  = 0b0_000_010_0_000_0_0_0_00;
        /// `Pga`: +/-1.024V range = Gain 4.
        const Pga_1_024V  = 0b0_000_011_0_000_0_0_0_00;
        /// `Pga`: +/-0.512V range = Gain 8.
        const Pga_0_512V  = 0b0_000_100_0_000_0_0_0_00;
        /// `Pga`: +/-0.256V range = Gain 16.
        const Pga_0_256V  = 0b0_000_101_0_000_0_0_0_00;
        // unused:          0b0_000_110_0_000_0_0_0_00;
        // unused:          0b0_000_111_0_000_0_0_0_00;

        /// `Mode`: Continuous conversion mode.
        const ModeContin  = 0b0_000_000_0_000_0_0_0_00;
        /// `Mode`: Power-down single-shot mode (default).
        const ModeSingle  = 0b0_000_000_1_000_0_0_0_00;

        /// `Dr`: 128 samples per second.
        const Dr_128SPS   = 0b0_000_000_0_000_0_0_0_00;
        /// `Dr`: 250 samples per second.
        const Dr_250SPS   = 0b0_000_000_0_001_0_0_0_00;
        /// `Dr`: 490 samples per second.
        const Dr_490SPS   = 0b0_000_000_0_010_0_0_0_00;
        /// `Dr`: 920 samples per second.
        const Dr_920SPS   = 0b0_000_000_0_011_0_0_0_00;
        /// `Dr`: 1600 samples per second (default).
        const Dr_1600SPS  = 0b0_000_000_0_100_0_0_0_00;
        /// `Dr`: 2400 samples per second.
        const Dr_2400SPS  = 0b0_000_000_0_101_0_0_0_00;
        /// `Dr`: 3300 samples per second.
        const Dr_3300SPS  = 0b0_000_000_0_110_0_0_0_00;
        // unused:          0b0_000_000_0_111_0_0_0_00;

        /// `Cmode`: Traditional comparator with hysteresis (default).
        const CmodeTrad   = 0b0_000_000_0_000_0_0_0_00;
        /// `Cmode`: Window comparator.
        const CmodeWindow = 0b0_000_000_0_000_1_0_0_00;

        /// `Cpol`: ALERT/RDY pin is low when active (default).
        const CpolActvlow = 0b0_000_000_0_000_0_0_0_00;
        /// `Cpol`: ALERT/RDY pin is high when active.
        const CpolActvhi  = 0b0_000_000_0_000_0_1_0_00;

        /// `Clat`: Non-latching comparator (default).
        const ClatNonlat  = 0b0_000_000_0_000_0_0_0_00;
        /// `Clat`: Latching comparator.
        const ClatLatch   = 0b0_000_000_0_000_0_0_1_00;

        /// `Cque`: Assert ALERT/RDY after one conversions.
        const Cque1Conv   = 0b0_000_000_0_000_0_0_0_00;
        /// `Cque`: Assert ALERT/RDY after two conversions.
        const Cque2Conv   = 0b0_000_000_0_000_0_0_0_01;
        /// `Cque`: Assert ALERT/RDY after four conversions.
        const Cque4Conv   = 0b0_000_000_0_000_0_0_0_10;
        /// `Cque`: Disable the comparator and put ALERT/RDY in high state (default).
        const CqueNone    = 0b0_000_000_0_000_0_0_0_11;
    }
}

impl Default for RegConfig {
    fn default() -> Self {
        let mut result = RegConfig::empty();

        result.insert(RegConfig::MuxDiff0_1);
        result.insert(RegConfig::Pga_2_048V);
        result.insert(RegConfig::ModeSingle);
        result.insert(RegConfig::Dr_1600SPS);
        result.insert(RegConfig::CmodeTrad);
        result.insert(RegConfig::CpolActvlow);
        result.insert(RegConfig::ClatNonlat);
        result.insert(RegConfig::CqueNone);

        result
    }
}
