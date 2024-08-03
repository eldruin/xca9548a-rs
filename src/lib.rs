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

const DEVICE_BASE_ADDRESS: u8 = 0b111_0000;
mod types;
pub use types::{Error, SlaveAddr, Xca9543a, Xca9545a, Xca9548a};
mod device_impl;
pub use device_impl::{DoOnAcquired, SelectChannels, Xca954xaData};
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
