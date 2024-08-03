use crate::Xca954xaData;
use core::cell;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E: core::fmt::Debug> {
    /// I²C bus error
    I2C(E),
    /// Could not acquire device. Maybe it is already acquired.
    CouldNotAcquireDevice,
}

/// Possible slave addresses
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SlaveAddr {
    /// Default slave address
    #[default]
    Default,
    /// Alternative slave address providing bit values for A2, A1 and A0
    /// Note: Some devices does not have all Ax pins, these should be set to false.
    Alternative(bool, bool, bool),
}

impl SlaveAddr {
    pub(crate) fn addr(self, default: u8) -> u8 {
        match self {
            SlaveAddr::Default => default,
            SlaveAddr::Alternative(a2, a1, a0) => {
                default | ((a2 as u8) << 2) | ((a1 as u8) << 1) | a0 as u8
            }
        }
    }
}

/// Device driver for T/PCA9548A
#[derive(Debug)]
pub struct Xca9548a<I2C> {
    pub(crate) data: cell::RefCell<Xca954xaData<I2C>>,
}

/// Device driver for T/PCA9543A
#[derive(Debug)]
pub struct Xca9543a<I2C> {
    pub(crate) data: cell::RefCell<Xca954xaData<I2C>>,
}

/// Device driver for T/PCA9545A
#[derive(Debug)]
pub struct Xca9545a<I2C> {
    pub(crate) data: cell::RefCell<Xca954xaData<I2C>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DEVICE_BASE_ADDRESS as BASE_ADDR;

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
