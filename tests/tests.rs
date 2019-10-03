extern crate embedded_hal;
extern crate embedded_hal_mock as hal;
use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};
extern crate xca9548a;
use xca9548a::{SlaveAddr, Xca9548a};

const DEV_ADDR: u8 = 0b111_0000;

fn new(transactions: &[I2cTrans]) -> Xca9548a<I2cMock> {
    Xca9548a::new(I2cMock::new(&transactions), SlaveAddr::default())
}

use embedded_hal::prelude::*;
const SLAVE_ADDR: u8 = 0b010_0000;
const SLAVE_WRITE_DATA: [u8; 2] = [0b0101_0101, 0b1010_1010];
const SLAVE_READ_DATA: [u8; 2] = [0b1001_1001, 0b0110_0110];

#[test]
fn can_select_channels() {
    let transactions = [I2cTrans::write(DEV_ADDR, vec![0x01])];
    let mut switch = new(&transactions);
    switch.select_channels(0x01).unwrap();
    switch.destroy().done();
}

#[test]
fn can_get_channel_status() {
    let transactions = [I2cTrans::read(DEV_ADDR, vec![0b0101_0101])];
    let mut switch = new(&transactions);
    let read_status = switch.get_channel_status().unwrap();
    assert_eq!(0b0101_0101, read_status);
    switch.destroy().done();
}

#[test]
fn can_write_to_slave() {
    let transactions = [
        I2cTrans::write(DEV_ADDR, vec![0x01]),
        I2cTrans::write(SLAVE_ADDR, SLAVE_WRITE_DATA.to_vec()),
    ];
    let mut switch = new(&transactions);
    switch.select_channels(0b0000_0001).unwrap();
    switch.write(SLAVE_ADDR, &SLAVE_WRITE_DATA).unwrap();
    switch.destroy().done();
}

#[test]
fn can_read_from_slave() {
    let transactions = [
        I2cTrans::write(DEV_ADDR, vec![0x01]),
        I2cTrans::read(SLAVE_ADDR, SLAVE_READ_DATA.to_vec()),
    ];
    let mut switch = new(&transactions);
    switch.select_channels(0b0000_0001).unwrap();

    let mut read_data = [0; 2];
    switch.read(SLAVE_ADDR, &mut read_data).unwrap();
    assert_eq!(read_data, SLAVE_READ_DATA);
    switch.destroy().done();
}
#[test]
fn can_write_read_from_slave() {
    let transactions = [
        I2cTrans::write(DEV_ADDR, vec![0x01]),
        I2cTrans::write_read(
            SLAVE_ADDR,
            SLAVE_WRITE_DATA.to_vec(),
            SLAVE_READ_DATA.to_vec(),
        ),
    ];
    let mut switch = new(&transactions);
    switch.select_channels(0b0000_0001).unwrap();

    let mut read_data = [0; 2];
    switch
        .write_read(SLAVE_ADDR, &SLAVE_WRITE_DATA, &mut read_data)
        .unwrap();
    assert_eq!(read_data, SLAVE_READ_DATA);
    switch.destroy().done();
}

#[test]
fn can_split_and_communicate_with_slave() {
    let slave_read_data_2 = [0xAB, 0xCD];
    let transactions = [
        I2cTrans::write(DEV_ADDR, vec![0x01]),
        I2cTrans::write(SLAVE_ADDR, SLAVE_WRITE_DATA.to_vec()),
        I2cTrans::write(DEV_ADDR, vec![0x02]),
        I2cTrans::read(SLAVE_ADDR, SLAVE_READ_DATA.to_vec()),
        I2cTrans::write(DEV_ADDR, vec![0x04]),
        I2cTrans::write_read(
            SLAVE_ADDR,
            SLAVE_WRITE_DATA.to_vec(),
            slave_read_data_2.to_vec(),
        ),
    ];
    let switch = new(&transactions);
    {
        let mut read_data_1 = [0; 2];
        let mut read_data_2 = [0; 2];
        let mut parts = switch.split();
        parts.i2c0.write(SLAVE_ADDR, &SLAVE_WRITE_DATA).unwrap();
        parts.i2c1.read(SLAVE_ADDR, &mut read_data_1).unwrap();
        parts
            .i2c2
            .write_read(SLAVE_ADDR, &SLAVE_WRITE_DATA, &mut read_data_2)
            .unwrap();
        assert_eq!(read_data_1, SLAVE_READ_DATA);
        assert_eq!(read_data_2, slave_read_data_2);
    }
    switch.destroy().done();
}

#[test]
fn when_split_only_change_channel_if_necessary() {
    let transactions = [
        I2cTrans::write(DEV_ADDR, vec![0x01]),
        I2cTrans::write(SLAVE_ADDR, SLAVE_WRITE_DATA.to_vec()),
        I2cTrans::read(SLAVE_ADDR, SLAVE_READ_DATA.to_vec()),
    ];
    let switch = new(&transactions);
    {
        let mut read_data = [0; 2];
        let mut parts = switch.split();
        parts.i2c0.write(SLAVE_ADDR, &SLAVE_WRITE_DATA).unwrap();
        parts.i2c0.read(SLAVE_ADDR, &mut read_data).unwrap();
        assert_eq!(read_data, SLAVE_READ_DATA);
    }
    switch.destroy().done();
}
