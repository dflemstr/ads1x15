//! Common error definitions.
use std::result;
use i2cdev;

/// A result returned by a driver operation.
pub type Result<A, D> = result::Result<A, Error<D>>;

/// An error generated by a driver operation.
#[derive(Debug, Fail)]
pub enum Error<D> where D: i2cdev::core::I2CDevice {
    /// An error originating from the I2C bus.
    #[fail(display = "I2C error: {}", cause)]
    I2C {
        /// The underlying I2C error cause.
        #[cause]
        cause: D::Error,
    }
}
