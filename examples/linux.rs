extern crate embedded_hal;
extern crate linux_embedded_hal;
extern crate xca9548a;

use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
use linux_embedded_hal::I2cdev;
use xca9548a::{Error, SlaveAddr, Xca9548a};

fn main() {
    let slave_address = 0b010_0000; // example slave address
    let write_data = [0b0101_0101, 0b1010_1010]; // some data to be sent
    let dev = I2cdev::new("/dev/i2c-1").unwrap();

    let mut switch = Xca9548a::new(dev, SlaveAddr::default());

    // Enable channel 0
    switch.select_channels(0b0000_0001).unwrap();

    // write to slave connected to channel 0 using
    // the I2C switch just as a normal I2C device
    if switch.write(slave_address, &write_data).is_err() {
        println!("Error received!");
    }

    // read from the slave connected to channel 0 using the
    // I2C switch just as a normal I2C device
    let mut read_data = [0; 2];
    if switch.read(slave_address, &mut read_data).is_err() {
        println!("Error received!");
    }

    // write_read from the slave connected to channel 0 using
    // the I2C switch just as a normal I2C device
    if switch
        .write_read(slave_address, &write_data, &mut read_data)
        .is_err()
    {
        println!("Error received!");
    }

    // Split the device and pass the slave (virtual) I2C devices
    // to an external driver
    let parts = switch.split();
    let mut some_driver = Driver::new(parts.i2c1);
    let mut some_other_driver = Driver::new(parts.i2c2);
    some_driver.do_something().unwrap();
    some_other_driver.do_something().unwrap();
}

/// Some driver defined in a different crate.
/// Defined here for completeness.
struct Driver<I2C> {
    i2c: I2C,
}

impl<I2C, E> Driver<I2C>
where
    I2C: Write<Error = E> + Read<Error = E> + WriteRead<Error = E>,
{
    pub fn new(i2c: I2C) -> Self {
        Driver { i2c }
    }
    pub fn do_something(&mut self) -> Result<(), Error<E>> {
        self.i2c.write(0x21, &[0x01, 0x02]).map_err(Error::I2C)
    }
}
