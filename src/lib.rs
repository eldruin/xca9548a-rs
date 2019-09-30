//! This is a platform agnostic Rust driver for the TCA9548A and
//! PCA9548A I2C switches/multiplexers, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Enable one or multiple I2C channels. See `select_channels()`.
//! - Communicate with the slaves connected to the enabled channels transparently.
//! - Split the device into slave (virtual) I2C devices (one per channel). See: `split()`.
//!
//! ## The devices
//!
//! The TCA9548A and PCA9548 devices have eight bidirectional translating switches
//! that can be controlled through the I2C bus. The SCL/SDA upstream pair fans out
//! to eight downstream pairs, or channels.
//! Any individual SCn/SDn channel or combination of channels can be selected,
//! determined by the contents of the programmable control register.
//! These downstream channels can be used to resolve I2C slave address conflicts.
//! For example, if  eight identical digital temperature sensors are needed in the
//! application, one sensor can be connected at each channel: 0-7.
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
//! ### Splitting into individual I2C devices and passing them into drivers
//!
//! Drivers usually take ownership of the I2C device.
//! It does not matter if the slaves have the same address.
//! Switching will be done automatically as necessary.
//!
//! ```no_run
//! extern crate embedded_hal;
//! extern crate linux_embedded_hal as hal;
//! extern crate xca9548a;
//!
//! use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
//! use xca9548a::{TCA9548A, SlaveAddr};
//!
//! /// Some driver defined in a different crate.
//! /// Defined here for completeness.
//! struct Driver<I2C> {
//!     i2c: I2C,
//! }
//!
//! impl<I2C, E> Driver<I2C>
//! where I2C: Write<Error = E> + Read<Error = E> + WriteRead<Error = E> {
//!     pub fn new(i2c: I2C) -> Self {
//!         Driver { i2c }
//!     }
//! }
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let i2c_switch = TCA9548A::new(dev, address);
//! let parts = i2c_switch.split();
//!
//! let my_driver = Driver::new(parts.i2c0);
//! let my_other_driver = Driver::new(parts.i2c1);
//! # }
//! ```
//!
//! ### Splitting into individual I2C devices
//!
//! It does not matter if the slaves have the same address.
//! Switching will be done automatically as necessary.
//!
//! ```no_run
//! extern crate embedded_hal;
//! extern crate linux_embedded_hal as hal;
//! extern crate xca9548a;
//!
//! use embedded_hal::blocking::i2c::{ Read, Write };
//! use xca9548a::{ TCA9548A, SlaveAddr };
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let i2c_switch = TCA9548A::new(dev, address);
//! let mut parts = i2c_switch.split();
//!
//! let slave_address = 0x20;
//! let data_for_slave = [0xAB, 0xCD];
//!
//! // Write some data to the slave using normal I2C interface
//! parts.i2c0.write(slave_address, &data_for_slave).unwrap();
//!
//! // Read some data from a slave connected to channel 1
//! let mut read_data = [0; 2];
//! parts.i2c1.read(slave_address, &mut read_data).unwrap();
//! # }
//! ```

//!

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

extern crate embedded_hal as hal;
use core::cell;
use hal::blocking::i2c;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
    /// Could not acquire device. Maybe it is already acquired.
    CouldNotAcquireDevice,
}

/// Possible slave addresses
#[derive(Debug, Clone)]
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address providing bit values for A2, A1 and A0
    Alternative(bool, bool, bool),
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
            SlaveAddr::Alternative(a2, a1, a0) => {
                default | ((a2 as u8) << 2) | ((a1 as u8) << 1) | a0 as u8
            }
        }
    }
}
const DEVICE_BASE_ADDRESS: u8 = 0b111_0000;

#[doc(hidden)]
#[derive(Debug, Default)]
pub struct Xca9548a<I2C> {
    /// The concrete I²C device implementation.
    pub(crate) i2c: I2C,
    /// The I²C device address.
    pub(crate) address: u8,
    pub(crate) selected_channel_mask: u8,
}

impl<I2C, E> SelectChannels for Xca9548a<I2C>
where
    I2C: i2c::Write<Error = E>,
{
    type Error = Error<E>;
    fn select_channels(&mut self, channels: u8) -> Result<(), Self::Error> {
        self.i2c
            .write(self.address, &[channels])
            .map_err(Error::I2C)?;
        self.selected_channel_mask = channels;
        Ok(())
    }
}

#[doc(hidden)]
pub trait DoOnAcquired<I2C>: private::Sealed {
    fn do_on_acquired<R, E>(
        &self,
        f: impl FnOnce(cell::RefMut<Xca9548a<I2C>>) -> Result<R, Error<E>>,
    ) -> Result<R, Error<E>>;
}

#[doc(hidden)]
pub trait SelectChannels: private::Sealed {
    type Error;
    fn select_channels(&mut self, mask: u8) -> Result<(), Self::Error>;
}

macro_rules! device {
    ( $device_name:ident ) => {
        /// Device driver
        #[derive(Debug, Default)]
        pub struct $device_name<I2C> {
            pub(crate) data: cell::RefCell<Xca9548a<I2C>>,
        }

        impl<I2C> $device_name<I2C> {
            /// Create new instance of the device
            pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
                let data = Xca9548a {
                    i2c,
                    address: address.addr(DEVICE_BASE_ADDRESS),
                    selected_channel_mask: 0,
                };
                $device_name {
                    data: cell::RefCell::new(data),
                }
            }

            /// Destroy driver instance, return I²C bus instance.
            pub fn destroy(self) -> I2C {
                self.data.into_inner().i2c
            }

            /// Split device into individual I2C devices
            ///
            /// It is not possible to know the compatibilities between channels
            /// so when talking to a split I2C device, only its channel
            /// will be selected.
            pub fn split<'a>(&'a self) -> Parts<'a, $device_name<I2C>, I2C> {
                Parts::new(&self)
            }
        }

        impl<I2C> DoOnAcquired<I2C> for $device_name<I2C> {
            fn do_on_acquired<R, E>(
                &self,
                f: impl FnOnce(cell::RefMut<Xca9548a<I2C>>) -> Result<R, Error<E>>,
            ) -> Result<R, Error<E>> {
                let dev = self
                    .data
                    .try_borrow_mut()
                    .map_err(|_| Error::CouldNotAcquireDevice)?;
                f(dev)
            }
        }

        impl<I2C, E> $device_name<I2C>
        where
            I2C: i2c::Write<Error = E>,
        {
            /// Select which channels are enabled.
            ///
            /// Each bit corresponds to a channel.
            /// Bit 0 corresponds to channel 0 and so on up to bit 7 which
            /// corresponds to channel 7.
            /// A `0` disables the channel and a `1` enables it.
            /// Several channels can be enabled at the same time
            pub fn select_channels(&mut self, channels: u8) -> Result<(), Error<E>> {
                self.do_on_acquired(|mut dev| dev.select_channels(channels))
            }
        }

        impl<I2C, E> $device_name<I2C>
        where
            I2C: i2c::Read<Error = E>,
        {
            /// Get status of channels.
            ///
            /// Each bit corresponds to a channel.
            /// Bit 0 corresponds to channel 0 and so on up to bit 7 which
            /// corresponds to channel 7.
            /// A `0` means the channel is disabled and a `1` that the channel is enabled.
            pub fn get_channel_status(&mut self) -> Result<u8, Error<E>> {
                let mut data = [0];
                self.do_on_acquired(|mut dev| {
                    let address = dev.address;
                    dev.i2c
                        .read(address, &mut data)
                        .map_err(Error::I2C)
                        .and(Ok(data[0]))
                })
            }
        }

        impl<I2C, E> i2c::Write for $device_name<I2C>
        where
            I2C: i2c::Write<Error = E>,
        {
            type Error = Error<E>;

            fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
                self.do_on_acquired(|mut dev| dev.i2c.write(address, bytes).map_err(Error::I2C))
            }
        }

        impl<I2C, E> i2c::Read for $device_name<I2C>
        where
            I2C: i2c::Read<Error = E>,
        {
            type Error = Error<E>;

            fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
                self.do_on_acquired(|mut dev| dev.i2c.read(address, buffer).map_err(Error::I2C))
            }
        }

        impl<I2C, E> i2c::WriteRead for $device_name<I2C>
        where
            I2C: i2c::WriteRead<Error = E>,
        {
            type Error = Error<E>;

            fn write_read(
                &mut self,
                address: u8,
                bytes: &[u8],
                buffer: &mut [u8],
            ) -> Result<(), Self::Error> {
                self.do_on_acquired(|mut dev| {
                    dev.i2c
                        .write_read(address, bytes, buffer)
                        .map_err(Error::I2C)
                })
            }
        }
    };
}

device!(TCA9548A);
device!(PCA9548A);

mod parts;
pub use parts::{I2cSlave, Parts};

mod private {
    use super::*;

    pub trait Sealed {}
    impl<I2C> Sealed for Xca9548a<I2C> {}
    impl<I2C> Sealed for PCA9548A<I2C> {}
    impl<I2C> Sealed for TCA9548A<I2C> {}
    impl<'a, DEV, I2C> Sealed for Parts<'a, DEV, I2C> {}
    impl<'a, DEV, I2C> Sealed for I2cSlave<'a, DEV, I2C> {}
}

#[cfg(test)]
mod tests {
    use super::DEVICE_BASE_ADDRESS as BASE_ADDR;
    use super::*;

    #[test]
    fn can_get_default_address() {
        let addr = SlaveAddr::default();
        assert_eq!(BASE_ADDR, addr.addr(BASE_ADDR));
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(
            0b111_0000,
            SlaveAddr::Alternative(false, false, false).addr(BASE_ADDR)
        );
        assert_eq!(
            0b111_0001,
            SlaveAddr::Alternative(false, false, true).addr(BASE_ADDR)
        );
        assert_eq!(
            0b111_0010,
            SlaveAddr::Alternative(false, true, false).addr(BASE_ADDR)
        );
        assert_eq!(
            0b111_0100,
            SlaveAddr::Alternative(true, false, false).addr(BASE_ADDR)
        );
        assert_eq!(
            0b111_0111,
            SlaveAddr::Alternative(true, true, true).addr(BASE_ADDR)
        );
    }
}
