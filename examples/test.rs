extern crate i2cdev;
extern crate ads1x15;
extern crate failure;

use std::thread;
use std::time;

fn main() -> Result<(), failure::Error> {
    let dev = i2cdev::linux::LinuxI2CDevice::new("/dev/i2c-1", 0x48)?;
    let mut dac = ads1x15::Ads1x15::new_ads1115(dev);

    loop {
        let value = dac.read_single_ended(ads1x15::Channel::A0)?;
        eprintln!("A0 = {}V", value);
        thread::sleep(time::Duration::from_secs(1));
    }
}
