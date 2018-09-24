extern crate xca9548a;
extern crate embedded_hal;
extern crate embedded_hal_mock as hal;
use xca9548a::{ TCA9548A, SlaveAddr};
use embedded_hal::blocking::i2c::{ Read, Write, WriteRead };

const DEVICE_ADDRESS: u8 = 0b111_0000;

fn setup<'a>(data: &'a[u8]) -> TCA9548A<hal::I2cMock<'a>> {
    let mut dev = hal::I2cMock::new();
    dev.set_read_data(&data);
    TCA9548A::new(dev, SlaveAddr::default())
}

fn check_sent_data(expander: TCA9548A<hal::I2cMock>, address: u8, data: &[u8]) {
    let dev = expander.destroy();
    assert_eq!(dev.get_last_address(), Some(address));
    assert_eq!(dev.get_write_data(), &data[..]);
}

#[test]
fn can_select() {
    let mut switch = setup(&[0]);
    switch.select(0b0000_0001).unwrap();
    check_sent_data(switch, DEVICE_ADDRESS, &[0b0000_0001]);
}

#[test]
fn can_write_to_slave() {
    let slave_address = 0b010_0000;
    let slave_data = [0b0101_0101, 0b1010_1010];
    let mut switch = setup(&[0]);
    switch.select(0b0000_0001).unwrap();
    
    switch.write(slave_address, &slave_data).unwrap();
    check_sent_data(switch, slave_address, &slave_data);
}

#[test]
fn can_read_from_slave() {
    let slave_address = 0b010_0000;
    let slave_data = [0b0101_0101, 0b1010_1010];
    let mut switch = setup(&slave_data);
    switch.select(0b0000_0001).unwrap();

    let mut read_data = [0; 2];
    switch.read(slave_address, &mut read_data).unwrap();
    let dev = switch.destroy();
    assert_eq!(dev.get_last_address(), Some(slave_address));
    assert_eq!(read_data, slave_data);
}

#[test]
fn can_write_read_from_slave() {
    let slave_address = 0b010_0000;
    let slave_write_data = [0b0101_0101, 0b1010_1010];
    let slave_read_data =  [0b1001_1001, 0b0110_0110];
    let mut switch = setup(&slave_read_data);
    switch.select(0b0000_0001).unwrap();
    
    let mut read_data = [0; 2];
    switch.write_read(slave_address, &slave_write_data, &mut read_data).unwrap();
    check_sent_data(switch, slave_address, &slave_write_data);
    assert_eq!(read_data, slave_read_data);
}