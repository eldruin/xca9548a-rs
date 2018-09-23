extern crate embedded_hal;
extern crate linux_embedded_hal;
extern crate xca9548a;

use linux_embedded_hal::I2cdev;
use embedded_hal::blocking::i2c::{ Read, Write, WriteRead };
use xca9548a::{TCA9548A, SlaveAddr};

fn main() {
    let slave_address = 0b010_0000; // example slave address
    let data_for_slave = [0b0101_0101, 0b1010_1010]; // some data to be sent
    let dev = I2cdev::new("/dev/i2c-1").unwrap();

    let mut i2c_switch = TCA9548A::new(dev, SlaveAddr::default());

    // Enable channel 0
    i2c_switch.select(0b0000_0001).unwrap();

    // write to slave connected to channel 0 using the I2C switch just as a normal I2C device
    if let Err(e) = i2c_switch.write(slave_address, &data_for_slave) {
         println!("Error received: {}", e);
    }

    // read from the slave connected to channel 0 using the I2C switch just as a normal I2C device
    let mut read_data = [0; 2];
    if let Err(e) = i2c_switch.read(slave_address, &mut read_data) {
         println!("Error received: {}", e);
    }

    // write_read from the slave connected to channel 0 using the I2C switch just as a normal I2C device
    if let Err(e) = i2c_switch.write_read(slave_address, &data_for_slave, &mut read_data) {
         println!("Error received: {}", e);
    }
}
