extern crate embedded_hal;
extern crate linux_embedded_hal;
extern crate xca9548a;

use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
use linux_embedded_hal::I2cdev;
use xca9548a::{SlaveAddr, TCA9548A};

fn main() {
    let slave_address = 0b010_0000; // example slave address
    let write_data = [0b0101_0101, 0b1010_1010]; // some data to be sent
    let dev = I2cdev::new("/dev/i2c-1").unwrap();

    let mut switch = TCA9548A::new(dev, SlaveAddr::default());

    // Enable channel 0
    switch.select_channels(0b0000_0001).unwrap();

    // write to slave connected to channel 0 using
    // the I2C switch just as a normal I2C device
    if let Err(_) = switch.write(slave_address, &write_data) {
        println!("Error received!");
    }

    // read from the slave connected to channel 0 using the
    // I2C switch just as a normal I2C device
    let mut read_data = [0; 2];
    if let Err(_) = switch.read(slave_address, &mut read_data) {
        println!("Error received!");
    }

    // write_read from the slave connected to channel 0 using
    // the I2C switch just as a normal I2C device
    if let Err(_) = switch.write_read(slave_address, &write_data, &mut read_data) {
        println!("Error received!");
    }
}
