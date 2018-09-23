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
//! TODO
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

/// TCA9548A device driver
#[derive(Debug, Default)]
pub struct TCA9548A<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// The I²C device address.
    address: u8,
}

const DEVICE_ADDRESS: u8 = 0b111_0000;

impl<I2C, E> TCA9548A<I2C>
where
    I2C: i2c::Write<Error = E>
{
    /// Create new instance of the device
    pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
        TCA9548A {
            i2c,
            address: address.addr(DEVICE_ADDRESS),
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
    pub fn select(&mut self, channels: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(DEVICE_ADDRESS, &[channels])
            .map_err(Error::I2C)
    }
}

impl<I2C, E> i2c::Write for TCA9548A<I2C>
where
    I2C: i2c::Write<Error = E> {
    type Error = E;
    
    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.i2c.write(address, bytes)
    }
}

impl<I2C, E> i2c::Read for TCA9548A<I2C>
where
    I2C: i2c::Read<Error = E> {
    type Error = E;
    
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c.read(address, buffer)
    }
}

impl<I2C, E> i2c::WriteRead for TCA9548A<I2C>
where
    I2C: i2c::WriteRead<Error = E> {
    type Error = E;
    
    fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c.write_read(address, bytes, buffer)
    }
}

#[cfg(test)]
mod tests {
    extern crate embedded_hal_mock as hal;

    use super::*;

    #[test]
    fn can_get_default_address() {
        let addr = SlaveAddr::default();
        assert_eq!(0b111_0000, addr.addr(0b111_0000));
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(0b111_0000, SlaveAddr::Alternative(false, false, false).addr(DEVICE_ADDRESS));
        assert_eq!(0b111_0001, SlaveAddr::Alternative(false, false,  true).addr(DEVICE_ADDRESS));
        assert_eq!(0b111_0010, SlaveAddr::Alternative(false,  true, false).addr(DEVICE_ADDRESS));
        assert_eq!(0b111_0100, SlaveAddr::Alternative( true, false, false).addr(DEVICE_ADDRESS));
        assert_eq!(0b111_0111, SlaveAddr::Alternative( true,  true,  true).addr(DEVICE_ADDRESS));
    }
}
