extern crate ads1x15;
extern crate failure;
extern crate i2cdev;

use std::thread;
use std::time;

fn main() -> Result<(), failure::Error> {
    let dev = i2cdev::linux::LinuxI2CDevice::new("/dev/i2c-1", 0x48)?;
    let mut dac = ads1x15::Ads1x15::new_ads1115(dev);

    loop {
        let a0 = dac.read_single_ended(ads1x15::Channel::A0)?;
        let a1 = dac.read_single_ended(ads1x15::Channel::A1)?;
        let a2 = dac.read_single_ended(ads1x15::Channel::A2)?;
        let a3 = dac.read_single_ended(ads1x15::Channel::A3)?;

        println!("{:8.5}V {:8.5}V {:8.5}V {:8.5}V", a0, a1, a2, a3);

        thread::sleep(time::Duration::from_secs(1));
    }
}
