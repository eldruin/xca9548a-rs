use super::{DoOnAcquired, Error, SelectChannels};
use core::marker::PhantomData;
use hal::blocking::i2c;

macro_rules! parts {
    ( $( $i2cx:ident, $I2CX:ident, $channel:expr ),+ ) => {
        $(  /// Slave I2C device
            pub struct $I2CX<'a, DEV: 'a, I2C>(&'a DEV, u8, PhantomData<I2C>);
        )*

        /// Slave I2C devices
        pub struct Parts<'a, DEV:'a, I2C> {
            $(
                /// Slave I2C device
                pub $i2cx: $I2CX<'a, DEV, I2C>,
            )*
        }

        impl<'a, DEV:'a, I2C> Parts<'a, DEV, I2C> {
            pub(crate) fn new(dev: &'a DEV) -> Self {
                Parts {
                    $(
                        $i2cx: $I2CX(&dev, $channel, PhantomData),
                    )*
                }
            }
        }
        $(
            impl<'a, DEV, I2C, E> i2c::Write for $I2CX<'a, DEV, I2C>
            where
                DEV: DoOnAcquired<I2C>,
                I2C: i2c::Write<Error = E>,
            {
                type Error = Error<E>;

                fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
                    self.0.do_on_acquired(|mut dev| {
                        if dev.selected_channel_mask != self.1 {
                            dev.select_channels(self.1)?;
                        }
                        dev.i2c.write(address, bytes).map_err(Error::I2C)
                    })
                }
            }

            impl<'a, DEV, I2C, E> i2c::Read for $I2CX<'a, DEV, I2C>
            where
                DEV: DoOnAcquired<I2C>,
                I2C: i2c::Write<Error = E> + i2c::Read<Error=E>,
            {
                type Error = Error<E>;

                fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
                    self.0.do_on_acquired(|mut dev| {
                        if dev.selected_channel_mask != self.1 {
                            dev.select_channels(self.1)?;
                        }
                        dev.i2c.read(address, buffer).map_err(Error::I2C)
                    })
                }
            }

            impl<'a, DEV, I2C, E> i2c::WriteRead for $I2CX<'a, DEV, I2C>
            where
                DEV: DoOnAcquired<I2C>,
                I2C: i2c::Write<Error = E> + i2c::WriteRead<Error=E>,
            {
                type Error = Error<E>;

                fn write_read(
                    &mut self,
                    address: u8,
                    bytes: &[u8],
                    buffer: &mut [u8],
                ) -> Result<(), Self::Error> {
                    self.0.do_on_acquired(|mut dev| {
                        if dev.selected_channel_mask != self.1 {
                            dev.select_channels(self.1)?;
                        }
                        dev.i2c.write_read(address, bytes, buffer).map_err(Error::I2C)
                    })
                }
            }
        )*
    }
}
parts!(
    i2c0, I2c0, 1, i2c1, I2c1, 2, i2c2, I2c2, 4, i2c3, I2c3, 8, i2c4, I2c4, 16, i2c5, I2c5, 32,
    i2c6, I2c6, 64, i2c7, I2c7, 128
);
