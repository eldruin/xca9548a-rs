extern crate embedded_hal;
extern crate embedded_hal_mock as hal;
use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};
extern crate xca9548a;
use xca9548a::{SlaveAddr, PCA9548A, TCA9548A};

const DEV_ADDR: u8 = 0b111_0000;

fn new_tca9548a(transactions: &[I2cTrans]) -> TCA9548A<I2cMock> {
    TCA9548A::new(I2cMock::new(&transactions), SlaveAddr::default())
}

fn new_pca9548a(transactions: &[I2cTrans]) -> PCA9548A<I2cMock> {
    PCA9548A::new(I2cMock::new(&transactions), SlaveAddr::default())
}

macro_rules! device_tests {
    ( $create:ident, $device_tests_mod:ident ) => {
        mod $device_tests_mod {
            use super::*;
            use embedded_hal::prelude::*;
            const SLAVE_ADDR: u8 = 0b010_0000;
            const SLAVE_WRITE_DATA: [u8; 2] = [0b0101_0101, 0b1010_1010];
            const SLAVE_READ_DATA: [u8; 2] = [0b1001_1001, 0b0110_0110];

            #[test]
            fn can_select_channels() {
                let transactions = [I2cTrans::write(DEV_ADDR, vec![0x01])];
                let mut switch = $create(&transactions);
                switch.select_channels(0x01).unwrap();
                switch.destroy().done();
            }

            #[test]
            fn can_get_channel_status() {
                let transactions = [I2cTrans::read(DEV_ADDR, vec![0b0101_0101])];
                let mut switch = $create(&transactions);
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
                let mut switch = $create(&transactions);
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
                let mut switch = $create(&transactions);
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
                let mut switch = $create(&transactions);
                switch.select_channels(0b0000_0001).unwrap();

                let mut read_data = [0; 2];
                switch
                    .write_read(SLAVE_ADDR, &SLAVE_WRITE_DATA, &mut read_data)
                    .unwrap();
                assert_eq!(read_data, SLAVE_READ_DATA);
                switch.destroy().done();
            }
        }
    };
}

device_tests!(new_tca9548a, tca9548a_tests);
device_tests!(new_pca9548a, pca9548a_tests);
