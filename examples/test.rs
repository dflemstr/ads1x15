#![feature(generators)]

extern crate ads1x15;
extern crate failure;
extern crate futures_await as futures;
extern crate i2cdev;
extern crate tokio;
extern crate tokio_timer;

use futures::prelude::async;
use futures::prelude::await;

use std::time;

fn main() -> Result<(), failure::Error> {
    use futures::Future;

    let dev = i2cdev::linux::LinuxI2CDevice::new("/dev/i2c-1", 0x48)?;
    let dac = ads1x15::Ads1x15::new_ads1115(dev);

    tokio::run(run(dac).map_err(|e| eprintln!("Error: {}", e)));
    Ok(())
}

#[async]
fn run(dac: ads1x15::Ads1x15<i2cdev::linux::LinuxI2CDevice>) -> Result<(), failure::Error> {
    loop {
        let a0 = await!(dac.read_single_ended(ads1x15::Channel::A0))?;
        let a1 = await!(dac.read_single_ended(ads1x15::Channel::A1))?;
        let a2 = await!(dac.read_single_ended(ads1x15::Channel::A2))?;
        let a3 = await!(dac.read_single_ended(ads1x15::Channel::A3))?;

        println!("{:8.5}V {:8.5}V {:8.5}V {:8.5}V", a0, a1, a2, a3);

        await!(tokio_timer::sleep(time::Duration::from_secs(1)))?;
    }
}
