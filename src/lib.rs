//! This is a platform agnostic Rust driver for the TCA954xA and
//! PCA954xA I2C switches/multiplexers, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Enable one or multiple I2C channels. See [`select_channels()`].
//! - Communicate with the slaves connected to the enabled channels transparently.
//! - Split the device into slave (virtual) I2C devices (one per channel). See: [`split()`].
//!
//! [`select_channels()`]: struct.Xca9548a.html#method.select_channels
//! [`split()`]: struct.Xca9548a.html#method.split
//!
//! ## The devices
//!
//! The TCA954xA and PCA954x devices have two to eight bidirectional translating switches
//! that can be controlled through the I2C bus. The SCL/SDA upstream pair fans out
//! to eight downstream pairs, or channels.
//! Any individual SCn/SDn channel or combination of channels can be selected,
//! determined by the contents of the programmable control register.
//! These downstream channels can be used to resolve I2C slave address conflicts.
//! For example, if  eight identical digital temperature sensors are needed in the
//! application, one sensor can be connected at each channel: 0-N.
//!
//! The TCA9545/3A and PCA9545/3A devices have an assosciated interrupt pin `INT` for each channel
//! which can be polled to check which channels have pending interrupts.
//! (Tip: Can also be used as general inputs)
//!
//! ### Datasheets
//! - [TCA9548A](http://www.ti.com/lit/ds/symlink/tca9548a.pdf)
//! - [PCA9548A](http://www.ti.com/lit/ds/symlink/pca9548a.pdf)
//! - [TCA9545A](http://www.ti.com/lit/ds/symlink/tca9545a.pdf)
//! - [PCA9545A](http://www.ti.com/lit/ds/symlink/pca9545a.pdf)
//! - [TCA9543A](http://www.ti.com/lit/ds/symlink/tca9543a.pdf)
//! - [PCA9543A](http://www.ti.com/lit/ds/symlink/pca9543a.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the device.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Instantiating with the default address
//!
//! Import this crate and an `embedded_hal` implementation, then instantiate
//! the device:
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use xca9548a::{Xca9548a, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut i2c_switch = Xca9548a::new(dev, address);
//! ```
//!
//! ### Providing an alternative address
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use xca9548a::{Xca9548a, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let (a2, a1, a0) = (false, false, true);
//! let address = SlaveAddr::Alternative(a2, a1, a0);
//! let mut i2c_switch = Xca9548a::new(dev, address);
//! ```
//!
//! ### Selecting channel 0 (SD0/SC0 pins)
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use xca9548a::{Xca9548a, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut i2c_switch = Xca9548a::new(dev, address);
//! i2c_switch.select_channels(0b0000_0001).unwrap();
//! ```
//! ### Reading and writing to device connected to channel 0 (SD0/SC0 pins)
//!
//! ```no_run
//! use embedded_hal::i2c::I2c;
//! use linux_embedded_hal::I2cdev;
//! use xca9548a::{Xca9548a, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut i2c_switch = Xca9548a::new(dev, address);
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
//! ```
//!
//! ### Splitting into individual I2C devices and passing them into drivers
//!
//! Drivers usually take ownership of the I2C device.
//! It does not matter if the slaves have the same address.
//! Switching will be done automatically as necessary.
//!
//! ```no_run
//! use embedded_hal::i2c::I2c;
//! use linux_embedded_hal::I2cdev;
//! use xca9548a::{Xca9548a, SlaveAddr};
//!
//! /// Some driver defined in a different crate.
//! /// Defined here for completeness.
//! struct Driver<I2C> {
//!     i2c: I2C,
//! }
//!
//! impl<I2C, E> Driver<I2C>
//! where I2C: I2c<Error = E> {
//!     pub fn new(i2c: I2C) -> Self {
//!         Driver { i2c }
//!     }
//! }
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let i2c_switch = Xca9548a::new(dev, address);
//! let parts = i2c_switch.split();
//!
//! let my_driver = Driver::new(parts.i2c0);
//! let my_other_driver = Driver::new(parts.i2c1);
//! ```
//!
//! ### Splitting into individual I2C devices
//!
//! It does not matter if the slaves have the same address.
//! Switching will be done automatically as necessary.
//!
//! ```no_run
//! use embedded_hal::i2c::I2c;
//! use linux_embedded_hal::I2cdev;
//! use xca9548a::{Xca9548a, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let i2c_switch = Xca9548a::new(dev, address);
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
//! ```
//!

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

use core::cell;
use embedded_hal::i2c as ehal;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E: core::fmt::Debug> {
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
    /// Note: Some devices does not have all Ax pins, these should be set to false.
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
pub struct Xca954xaData<I2C> {
    /// The concrete I²C device implementation.
    pub(crate) i2c: I2C,
    /// The I²C device address.
    pub(crate) address: u8,
    pub(crate) selected_channel_mask: u8,
}

impl<I2C, E> SelectChannels for Xca954xaData<I2C>
where
    I2C: ehal::I2c<Error = E>,
    E: core::fmt::Debug,
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
    fn do_on_acquired<R, E: ehal::Error>(
        &self,
        f: impl FnOnce(cell::RefMut<Xca954xaData<I2C>>) -> Result<R, Error<E>>,
    ) -> Result<R, Error<E>>;
}

#[doc(hidden)]
pub trait SelectChannels: private::Sealed {
    type Error;
    fn select_channels(&mut self, mask: u8) -> Result<(), Self::Error>;
}

/// Device driver for T/PCA9548A
#[derive(Debug, Default)]
pub struct Xca9548a<I2C> {
    pub(crate) data: cell::RefCell<Xca954xaData<I2C>>,
}

/// Device driver for T/PCA9543A
#[derive(Debug, Default)]
pub struct Xca9543a<I2C> {
    pub(crate) data: cell::RefCell<Xca954xaData<I2C>>,
}

/// Device driver for T/PCA9545A
#[derive(Debug, Default)]
pub struct Xca9545a<I2C> {
    pub(crate) data: cell::RefCell<Xca954xaData<I2C>>,
}

impl<E> ehal::Error for Error<E>
where
    E: ehal::Error,
{
    fn kind(&self) -> ehal::ErrorKind {
        match self {
            Error::I2C(e) => e.kind(),
            Error::CouldNotAcquireDevice => ehal::ErrorKind::Other,
        }
    }
}

macro_rules! i2c_traits {
    ( $name:ident ) => {
        impl<I2C> DoOnAcquired<I2C> for $name<I2C> {
            fn do_on_acquired<R, E: ehal::Error>(
                &self,
                f: impl FnOnce(cell::RefMut<Xca954xaData<I2C>>) -> Result<R, Error<E>>,
            ) -> Result<R, Error<E>> {
                let dev = self
                    .data
                    .try_borrow_mut()
                    .map_err(|_| Error::CouldNotAcquireDevice)?;
                f(dev)
            }
        }

        impl<I2C, E> ehal::ErrorType for $name<I2C>
        where
            I2C: ehal::I2c<Error = E>,
            E: ehal::Error,
        {
            type Error = Error<E>;
        }

        impl<I2C, E> ehal::I2c for $name<I2C>
        where
            I2C: ehal::I2c<Error = E>,
            E: ehal::Error,
        {
            fn transaction(
                &mut self,
                address: u8,
                operations: &mut [ehal::Operation<'_>],
            ) -> Result<(), Error<E>> {
                self.do_on_acquired(|mut dev| {
                    dev.i2c.transaction(address, operations).map_err(Error::I2C)
                })
            }

            fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
                self.do_on_acquired(|mut dev| dev.i2c.read(address, read).map_err(Error::I2C))
            }

            fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
                self.do_on_acquired(|mut dev| dev.i2c.write(address, write).map_err(Error::I2C))
            }

            fn write_read(
                &mut self,
                address: u8,
                write: &[u8],
                read: &mut [u8],
            ) -> Result<(), Self::Error> {
                self.do_on_acquired(|mut dev| {
                    dev.i2c.write_read(address, write, read).map_err(Error::I2C)
                })
            }
        }
    };
}

macro_rules! impl_device {
    ( $name:ident, $parts:ident ) => {
        impl<I2C> $name<I2C> {
            /// Create new instance of the device
            pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
                let data = Xca954xaData {
                    i2c,
                    address: address.addr(DEVICE_BASE_ADDRESS),
                    selected_channel_mask: 0,
                };
                $name {
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
            pub fn split(&self) -> $parts<$name<I2C>, I2C> {
                $parts::new(&self)
            }
        }
    };
    ( $name:ident, $parts:ident, no_interrupts ) => {
        impl_device!($name, $parts);

        impl<I2C, E> $name<I2C>
        where
            I2C: ehal::I2c<Error = E>,
            E: ehal::Error,
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

        impl<I2C, E> $name<I2C>
        where
            I2C: ehal::I2c<Error = E>,
            E: ehal::Error,
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
    };
    ( $name:ident, $parts:ident, $mask:expr, interrupts ) => {
        impl_device!($name, $parts);

        impl<I2C, E> $name<I2C>
        where
            I2C: ehal::I2c<Error = E>,
            E: ehal::Error,
        {
            /// Get status of channels.
            ///
            /// Each bit corresponds to a channel.
            /// Bit 0 corresponds to channel 0, bit 1 to channel 1 and so on.
            /// A `0` means the channel is disabled and a `1` that the channel is enabled.
            pub fn get_channel_status(&mut self) -> Result<u8, Error<E>> {
                let mut data = [0];
                self.do_on_acquired(|mut dev| {
                    let address = dev.address;
                    dev.i2c
                        .read(address, &mut data)
                        .map_err(Error::I2C)
                        .and(Ok(data[0] & $mask))
                })
            }

            /// Get status of channel interrupts.
            ///
            /// Each bit corresponds to a channel.
            /// Bit 0 corresponds to channel 0, bit 1 to channel 1 and so on.
            /// A `1` means the channel's interrupt is high and a `0` that the channel's interrupt is low.
            /// Note: I2C interrupts are usually active LOW!
            pub fn get_interrupt_status(&mut self) -> Result<u8, Error<E>> {
                let mut data = [0];
                self.do_on_acquired(|mut dev| {
                    let address = dev.address;
                    dev.i2c
                        .read(address, &mut data)
                        .map_err(Error::I2C)
                        .and(Ok((data[0] >> 4) & $mask))
                })
            }
        }

        impl<I2C, E> $name<I2C>
        where
            I2C: ehal::I2c<Error = E>,
            E: ehal::Error,
        {
            /// Select which channels are enabled.
            ///
            /// Each bit corresponds to a channel.
            /// Bit 0 corresponds to channel 0, bit 1 to channel 1 and so on.
            /// A `0` disables the channel and a `1` enables it.
            /// Several channels can be enabled at the same time.
            ///
            /// Channels/bits that does not exist for the specific device are ignored.
            pub fn select_channels(&mut self, channels: u8) -> Result<(), Error<E>> {
                self.do_on_acquired(|mut dev| dev.select_channels(channels & $mask))
            }
        }
    };
}

impl_device!(Xca9548a, Parts, no_interrupts);
i2c_traits!(Xca9548a);

impl_device!(Xca9543a, Parts2, 0x03, interrupts);
i2c_traits!(Xca9543a);

impl_device!(Xca9545a, Parts4, 0x0f, interrupts);
i2c_traits!(Xca9545a);

mod parts;
pub use crate::parts::{I2cSlave, Parts, Parts2, Parts4};

mod private {
    use super::*;

    pub trait Sealed {}
    impl<I2C> Sealed for Xca954xaData<I2C> {}
    impl<I2C> Sealed for Xca9548a<I2C> {}
    impl<I2C> Sealed for Xca9543a<I2C> {}
    impl<I2C> Sealed for Xca9545a<I2C> {}
    impl<'a, DEV, I2C> Sealed for Parts<'a, DEV, I2C> {}
    impl<'a, DEV, I2C> Sealed for Parts2<'a, DEV, I2C> {}
    impl<'a, DEV, I2C> Sealed for Parts4<'a, DEV, I2C> {}
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
