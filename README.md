# Rust TCA954xA and PCA954xA I2C Switch Driver

[![crates.io](https://img.shields.io/crates/v/xca9548a.svg)](https://crates.io/crates/xca9548a)
[![Docs](https://docs.rs/xca9548a/badge.svg)](https://docs.rs/xca9548a)
![MSRV](https://img.shields.io/badge/rustc-1.62+-blue.svg)
[![Build Status](https://github.com/eldruin/xca9548a-rs/workflows/Build/badge.svg)](https://github.com/eldruin/xca9548a-rs/actions?query=workflow%3ABuild)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/xca9548a-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/xca9548a-rs?branch=master)

This is a platform agnostic Rust driver for the for TCA9548A and PCA9548A I2C
switches/multiplexers using the [`embedded-hal`] traits.

This driver allows you to:
- Enable one or multiple I2C channels. See: `select_channels()`.
- Communicate with the slaves connected to the enabled channels transparently.
- Split the device into slave (virtual) I2C devices (one per channel). See: `split()`.

## The devices

The TCA954xA and PCA954x devices have two to eight bidirectional translating switches
that can be controlled through the I2C bus. The SCL/SDA upstream pair fans out
to eight downstream pairs, or channels.
Any individual SCn/SDn channel or combination of channels can be selected,
determined by the contents of the programmable control register.
These downstream channels can be used to resolve I2C slave address conflicts.
For example, if  eight identical digital temperature sensors are needed in the
application, one sensor can be connected at each channel: 0-N.

The TCA9545/3A and PCA9545/3A devices have an assosciated interrupt pin `INT` for each channel
which can be polled to check which channels have pending interrupts.
(Tip: Can also be used as general inputs)

### Datasheets
- [TCA9548A](http://www.ti.com/lit/ds/symlink/tca9548a.pdf)
- [PCA9548A](http://www.ti.com/lit/ds/symlink/pca9548a.pdf)
- [TCA9545A](http://www.ti.com/lit/ds/symlink/tca9545a.pdf)
- [PCA9545A](http://www.ti.com/lit/ds/symlink/pca9545a.pdf)
- [TCA9543A](http://www.ti.com/lit/ds/symlink/tca9543a.pdf)
- [PCA9543A](http://www.ti.com/lit/ds/symlink/pca9543a.pdf)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
use linux_embedded_hal::I2cdev;
use xca9548a::{Error, SlaveAddr, Xca9548a};

fn main() {
    let slave_address = 0b010_0000; // example slave address
    let write_data = [0b0101_0101, 0b1010_1010]; // some data to be sent
    let dev = I2cdev::new("/dev/i2c-1").unwrap();

    let mut switch = Xca9548a::new(dev, SlaveAddr::default());

    // Enable channel 0
    switch.select_channels(0b0000_0001).unwrap();

    // write to slave connected to channel 0 using
    // the I2C switch just as a normal I2C device
    if switch.write(slave_address, &write_data).is_err() {
        println!("Error received!");
    }

    // read from the slave connected to channel 0 using the
    // I2C switch just as a normal I2C device
    let mut read_data = [0; 2];
    if switch.read(slave_address, &mut read_data).is_err() {
        println!("Error received!");
    }

    // write_read from the slave connected to channel 0 using
    // the I2C switch just as a normal I2C device
    if switch
        .write_read(slave_address, &write_data, &mut read_data)
        .is_err()
    {
        println!("Error received!");
    }

    // Split the device and pass the slave (virtual) I2C devices
    // to an external driver
    let parts = switch.split();
    let mut some_driver = Driver::new(parts.i2c1);
    let mut some_other_driver = Driver::new(parts.i2c2);
    some_driver.do_something().unwrap();
    some_other_driver.do_something().unwrap();
}

/// Some driver defined in a different crate.
/// Defined here for completeness.
struct Driver<I2C> {
    i2c: I2C,
}

impl<I2C, E> Driver<I2C>
where
    I2C: Write<Error = E> + Read<Error = E> + WriteRead<Error = E>,
{
    pub fn new(i2c: I2C) -> Self {
        Driver { i2c }
    }
    pub fn do_something(&mut self) -> Result<(), Error<E>> {
        self.i2c.write(0x21, &[0x01, 0x02]).map_err(Error::I2C)
    }
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/xca9548a-rs/issues).

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.62 and up. It *might*
compile with older versions but that may change in any new patch release.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
