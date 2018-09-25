//! This is a platform agnostic Rust driver for the TCA9548A and
//! PCA9548A I2C switches/multiplexers, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Enable one or multiple I2C channels.
//! - Communicate with the slaves connected to the enabled channels transparently.
//!
//! ## The devices
//! TODO
//!
//! ### Datasheets
//! - [TCA9548A](http://www.ti.com/lit/ds/symlink/tca9548a.pdf)
//! - [PCA9548A](http://www.ti.com/lit/ds/symlink/pca9548a.pdf)
//! 
//! ## Usage examples (see also examples folder)
//!
//! ### Instantiating with the default address
//!
//! Import this crate and an `embedded_hal` implementation, then instantiate
//! the device:
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate xca9548a;
//!
//! use hal::I2cdev;
//! use xca9548a::{TCA9548A, SlaveAddr};
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut i2c_switch = TCA9548A::new(dev, address);
//! # }
//! ```
//!
//! ### Providing an alternative address
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate xca9548a;
//!
//! use hal::I2cdev;
//! use xca9548a::{TCA9548A, SlaveAddr};
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let (a2, a1, a0) = (false, false, true);
//! let address = SlaveAddr::Alternative(a2, a1, a0);
//! let mut i2c_switch = TCA9548A::new(dev, address);
//! # }
//! ```
//!
//! ### Selecting channel 0 (SD0/SC0 pins)
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate xca9548a;
//!
//! use hal::I2cdev;
//! use xca9548a::{TCA9548A, SlaveAddr};
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut i2c_switch = TCA9548A::new(dev, address);
//! i2c_switch.select_channels(0b0000_0001).unwrap();
//! # }
//! ```
//! ### Reading and writing to device connected to channel 0 (SD0/SC0 pins)
//!
//! ```no_run
//! extern crate embedded_hal;
//! extern crate linux_embedded_hal as hal;
//! extern crate xca9548a;
//!
//! use hal::I2cdev;
//! use embedded_hal::blocking::i2c::{ Read, Write };
//! use xca9548a::{ TCA9548A, SlaveAddr };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut i2c_switch = TCA9548A::new(dev, address);
//! i2c_switch.select_channels(0b0000_0001).unwrap();
//!
//! let slave_address = 0b010_0000; // example slave address
//! let data_for_slave = [0b0101_0101, 0b1010_1010]; // some data to be sent
//! 
//! // Read some data from a slave connected to channel 0 using the
//! // I2C switch just as a normal I2C device
//! let mut read_data = [0; 2];
//! i2c_switch.read(slave_address, &mut read_data).unwrap();
//!
//! // Write some data to the slave
//! i2c_switch.write(slave_address, &data_for_slave).unwrap();
//! # }
//! ```
//!

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

extern crate embedded_hal as hal;
use hal::blocking::i2c;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
}

/// Possible slave addresses
#[derive(Debug, Clone)]
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address providing bit values for A2, A1 and A0
    Alternative(bool, bool, bool)
}

impl Default for SlaveAddr {
    /// Default slave address
    fn default() -> Self {
        SlaveAddr::Default
    }
}

impl SlaveAddr {
    fn addr(self, default: u8) -> u8 {
        match self {
            SlaveAddr::Default => default,
            SlaveAddr::Alternative(a2, a1, a0) => default           |
                                                  ((a2 as u8) << 2) |
                                                  ((a1 as u8) << 1) |
                                                    a0 as u8
        }
    }
}
const DEVICE_BASE_ADDRESS: u8 = 0b111_0000;

macro_rules! device {
    ( $($device_name:ident),+ ) => {
        $(
            /// Device driver
            #[derive(Debug, Default)]
            pub struct $device_name<I2C> {
                /// The concrete I²C device implementation.
                i2c: I2C,
                /// The I²C device address.
                address: u8,
            }

            impl<I2C, E> $device_name<I2C>
            where
                I2C: i2c::Write<Error = E>
            {
                /// Create new instance of the device
                pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
                    $device_name {
                        i2c,
                        address: address.addr(DEVICE_BASE_ADDRESS),
                    }
                }

                /// Destroy driver instance, return I²C bus instance.
                pub fn destroy(self) -> I2C {
                    self.i2c
                }

                /// Select which channels are enabled.
                ///
                /// Each bit corresponds to a channel.
                /// Bit 0 corresponds to channel 0 and so on up to bit 7 which
                /// corresponds to channel 7.
                /// A `0` disables the channel and a `1` enables it.
                /// Several channels can be enabled at the same time
                pub fn select_channels(&mut self, channels: u8) -> Result<(), Error<E>> {
                    self.i2c
                        .write(DEVICE_BASE_ADDRESS, &[channels])
                        .map_err(Error::I2C)
                }
            }

            impl<I2C, E> i2c::Write for $device_name<I2C>
            where
                I2C: i2c::Write<Error = E> {
                type Error = E;
                
                fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
                    self.i2c.write(address, bytes)
                }
            }

            impl<I2C, E> i2c::Read for $device_name<I2C>
            where
                I2C: i2c::Read<Error = E> {
                type Error = E;
                
                fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
                    self.i2c.read(address, buffer)
                }
            }

            impl<I2C, E> i2c::WriteRead for $device_name<I2C>
            where
                I2C: i2c::WriteRead<Error = E> {
                type Error = E;
                
                fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
                    self.i2c.write_read(address, bytes, buffer)
                }
            }

        )*
    }
}

device!(TCA9548A, PCA9548A);

#[cfg(test)]
mod tests {
    extern crate embedded_hal_mock as hal;

    use super::*;

    #[test]
    fn can_get_default_address() {
        let addr = SlaveAddr::default();
        assert_eq!(DEVICE_BASE_ADDRESS, addr.addr(DEVICE_BASE_ADDRESS));
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(0b111_0000, SlaveAddr::Alternative(false, false, false).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b111_0001, SlaveAddr::Alternative(false, false,  true).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b111_0010, SlaveAddr::Alternative(false,  true, false).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b111_0100, SlaveAddr::Alternative( true, false, false).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b111_0111, SlaveAddr::Alternative( true,  true,  true).addr(DEVICE_BASE_ADDRESS));
    }
}
