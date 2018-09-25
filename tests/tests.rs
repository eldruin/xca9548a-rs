extern crate xca9548a;
extern crate embedded_hal;
extern crate embedded_hal_mock as hal;
use xca9548a::{ TCA9548A, PCA9548A, SlaveAddr};

const DEVICE_BASE_ADDRESS: u8 = 0b111_0000;

macro_rules! device_tests {
    ( $device_name:ident, $device_tests_mod:ident ) => {
        mod $device_tests_mod {
            use super::*;
            use embedded_hal::blocking::i2c::{ Read, Write, WriteRead };
            fn setup<'a>(data: &'a[u8]) -> $device_name<hal::I2cMock<'a>> {
                let mut dev = hal::I2cMock::new();
                dev.set_read_data(&data);
                $device_name::new(dev, SlaveAddr::default())
            }

            fn check_sent_data(switch: $device_name<hal::I2cMock>, address: u8, data: &[u8]) {
                let dev = switch.destroy();
                assert_eq!(dev.get_last_address(), Some(address));
                assert_eq!(dev.get_write_data(), &data[..]);
            }

            #[test]
            fn can_select_channels() {
                let mut switch = setup(&[0]);
                switch.select_channels(0b0000_0001).unwrap();
                check_sent_data(switch, DEVICE_BASE_ADDRESS, &[0b0000_0001]);
            }

            #[test]
            fn can_get_channel_status() {
                let status = [0b0101_0101];
                let mut switch = setup(&status);
                let read_status = switch.get_channel_status().unwrap();
                assert_eq!(status[0], read_status);
                let dev = switch.destroy();
                assert_eq!(dev.get_last_address(), Some(DEVICE_BASE_ADDRESS));
            }

            #[test]
            fn can_write_to_slave() {
                let slave_address = 0b010_0000;
                let slave_data = [0b0101_0101, 0b1010_1010];
                let mut switch = setup(&[0]);
                switch.select_channels(0b0000_0001).unwrap();
                
                switch.write(slave_address, &slave_data).unwrap();
                check_sent_data(switch, slave_address, &slave_data);
            }

            #[test]
            fn can_read_from_slave() {
                let slave_address = 0b010_0000;
                let slave_data = [0b0101_0101, 0b1010_1010];
                let mut switch = setup(&slave_data);
                switch.select_channels(0b0000_0001).unwrap();

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
                switch.select_channels(0b0000_0001).unwrap();
                
                let mut read_data = [0; 2];
                switch.write_read(slave_address, &slave_write_data, &mut read_data).unwrap();
                check_sent_data(switch, slave_address, &slave_write_data);
                assert_eq!(read_data, slave_read_data);
            }
        }
    }
}

device_tests!(TCA9548A, tca9548a_tests);
device_tests!(PCA9548A, pca9548a_tests);