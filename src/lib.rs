//! I2C driver for the Texas Instruments ADS1015/ADS1115 ADC.
//!
//! Technical specifications:
//!
//!   - <http://www.ti.com/lit/ds/symlink/ads1015.pdf>
//!   - <http://www.ti.com/lit/ds/symlink/ads1115.pdf>
#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
        trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
        unused_qualifications)]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate failure;
extern crate i2cdev;

use std::thread;
use std::time;

pub mod error;
pub mod reg;

/// An interface to an ADS1x15 device that can be used to control the device over I2C.
#[derive(Debug)]
pub struct Ads1x15<D> {
    device: D,
    gain: Gain,
    model: Model,
}

#[derive(Debug)]
enum Model {
    ADS1015,
    ADS1115,
}

/// A channel on the ADS1x15 that contains an analog electric signal.
#[derive(Clone, Copy, Debug)]
pub enum Channel {
    /// The channel corresponding to the `A0` pin.
    A0,
    /// The channel corresponding to the `A1` pin.
    A1,
    /// The channel corresponding to the `A2` pin.
    A2,
    /// The channel corresponding to the `A3` pin.
    A3,
}

/// Configuration for the gain setting of the device.
///
/// The gain setting sets the measurable range but it is not possible to measure voltages higher
/// than the voltage of the VDD pin of the chip.
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum Gain {
    /// The measurable range is ±6.144V.
    Within6_144V,
    /// The measurable range is ±4.096V.
    Within4_096V,
    /// The measurable range is ±2.048V.
    Within2_048V,
    /// The measurable range is ±1.024V.
    Within1_024V,
    /// The measurable range is ±0.512V.
    Within0_512V,
    /// The measurable range is ±0.256V.
    Within0_256V,
}

impl<D> Ads1x15<D> {
    /// Create a new interface to an ADS1015 device.
    ///
    /// Uses the supplied I2C device.
    pub fn new_ads1015(device: D) -> Self {
        let gain = Gain::Within6_144V;
        let model = Model::ADS1015;

        Ads1x15 {
            device,
            gain,
            model,
        }
    }

    /// Create a new interface to an ADS1115 device.
    ///
    /// Uses the supplied I2C device.
    pub fn new_ads1115(device: D) -> Self {
        let gain = Gain::Within6_144V;
        let model = Model::ADS1115;

        Ads1x15 {
            device,
            gain,
            model,
        }
    }

    /// Returns the current gain setting of the device.
    pub fn gain(&self) -> Gain {
        self.gain
    }

    /// Changes the gain setting of the device.
    pub fn set_gain(&mut self, gain: Gain) {
        self.gain = gain;
    }
}

impl<D> Ads1x15<D>
where
    D: i2cdev::core::I2CDevice,
    D::Error: Send + Sync + 'static,
{
    /// Reads the single-ended voltage of one of the input channels.
    ///
    /// The returned value is the electric potential in volts (V) measured on the specified channel.
    pub fn read_single_ended(&mut self, channel: Channel) -> error::Result<f32> {
        let mut config = reg::RegConfig::default();
        config.insert(self.gain.as_reg_config());
        config.insert(channel.as_reg_config_mux_single());

        // Set 'start single-conversion' bit
        config.insert(reg::RegConfig::OsSingle);

        self.device
            .smbus_write_word_data(reg::Register::Config.bits(), config.bits())
            .map_err(|error| error::Error::I2C {
                error: Box::new(error),
            })?;

        // TODO(dflemstr): make this non-blocking, maybe using futures?
        thread::sleep(self.model.conversion_delay());

        let value = self.device
            .smbus_read_word_data(reg::Register::Convert.bits())
            .map_err(|error| error::Error::I2C {
                error: Box::new(error),
            })?;

        let value = self.model.convert_raw_voltage(self.gain, value as i16);

        Ok(value)
    }
}

impl Channel {
    /// Converts this channel value into a valid value for the I2C `Config` register, setting the
    /// mux to single-ended measurements for that channel.
    pub fn as_reg_config_mux_single(&self) -> reg::RegConfig {
        match *self {
            Channel::A0 => reg::RegConfig::MuxSingle0,
            Channel::A1 => reg::RegConfig::MuxSingle1,
            Channel::A2 => reg::RegConfig::MuxSingle2,
            Channel::A3 => reg::RegConfig::MuxSingle3,
        }
    }
}

impl Gain {
    /// Converts this gain value into a valid value for the I2C `Config` register.
    pub fn as_reg_config(&self) -> reg::RegConfig {
        match *self {
            Gain::Within6_144V => reg::RegConfig::Pga_6_144V,
            Gain::Within4_096V => reg::RegConfig::Pga_4_096V,
            Gain::Within2_048V => reg::RegConfig::Pga_2_048V,
            Gain::Within1_024V => reg::RegConfig::Pga_1_024V,
            Gain::Within0_512V => reg::RegConfig::Pga_0_512V,
            Gain::Within0_256V => reg::RegConfig::Pga_0_256V,
        }
    }
}

impl Model {
    fn conversion_delay(&self) -> time::Duration {
        match *self {
            Model::ADS1015 => time::Duration::from_millis(1),
            Model::ADS1115 => time::Duration::from_millis(8),
        }
    }

    fn convert_raw_voltage(&self, gain: Gain, value: i16) -> f32 {
        match *self {
            Model::ADS1015 => {
                let value = (value >> 4) as f32;
                match gain {
                    Gain::Within6_144V => value * 3.0000e-3,
                    Gain::Within4_096V => value * 2.0000e-3,
                    Gain::Within2_048V => value * 1.0000e-3,
                    Gain::Within1_024V => value * 5.0000e-4,
                    Gain::Within0_512V => value * 2.5000e-4,
                    Gain::Within0_256V => value * 1.2500e-4,
                }
            }
            Model::ADS1115 => {
                let value = value as f32;
                match gain {
                    Gain::Within6_144V => value * 1.8750e-4,
                    Gain::Within4_096V => value * 1.2500e-4,
                    Gain::Within2_048V => value * 6.2500e-5,
                    Gain::Within1_024V => value * 3.1250e-5,
                    Gain::Within0_512V => value * 1.5625e-5,
                    Gain::Within0_256V => value * 7.8125e-6,
                }
            }
        }
    }
}
