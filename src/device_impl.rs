use crate::{
    parts::{Parts, Parts2, Parts4},
    private, Error, SlaveAddr, Xca9543a, Xca9545a, Xca9548a, DEVICE_BASE_ADDRESS,
};
use core::cell;
use embedded_hal::i2c as ehal;

#[doc(hidden)]
#[derive(Debug)]
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
