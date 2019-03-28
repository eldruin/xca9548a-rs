# Rust TCA9548A and PCA9548A I2C Switch Driver

[![crates.io](https://img.shields.io/crates/v/xca9548a.svg)](https://crates.io/crates/xca9548a)
[![Docs](https://docs.rs/xca9548a/badge.svg)](https://docs.rs/xca9548a)
[![Build Status](https://travis-ci.org/eldruin/xca9548a-rs.svg?branch=master)](https://travis-ci.org/eldruin/xca9548a-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/xca9548a-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/xca9548a-rs?branch=master)
![Maintenance Intention](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

This is a platform agnostic Rust driver for the for TCA9548A and PCA9548A I2C
switches/multiplexers using the [`embedded-hal`] traits.

This driver allows you to:
- Enable one or multiple I2C channels.
- Communicate with the slaves connected to the enabled channels transparently.

## The devices
The TCA9548A and PCA9548 devices have eight bidirectional translating switches
that can be controlled through the I2C bus. The SCL/SDA upstream pair fans out
to eight downstream pairs, or channels.
Any individual SCn/SDn channel or combination of channels can be selected,
determined by the contents of the programmable control register.
These downstream channels can be used to resolve I2C slave address conflicts.
For example, if  eight identical digital temperature sensors are needed in the
application, one sensor can be connected at each channel: 0-7.

Datasheets:
- [TCA9548A](http://www.ti.com/lit/ds/symlink/tca9548a.pdf)
- [PCA9548A](http://www.ti.com/lit/ds/symlink/pca9548a.pdf)

## Usage

```rust
extern crate embedded_hal;
extern crate linux_embedded_hal;
extern crate xca9548a;

use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
use linux_embedded_hal::I2cdev;
use xca9548a::{SlaveAddr, TCA9548A};

fn main() {
    let slave_address = 0b010_0000; // example slave address
    let write_data = [0b0101_0101, 0b1010_1010]; // some data to be sent
    let dev = I2cdev::new("/dev/i2c-1").unwrap();

    let mut switch = TCA9548A::new(dev, SlaveAddr::default());

    // Enable channel 0
    switch.select_channels(0b0000_0001).unwrap();

    // write to slave connected to channel 0 using
    // the I2C switch just as a normal I2C device
    if let Err(e) = switch.write(slave_address, &write_data) {
        println!("Error received: {}", e);
    }

    // read from the slave connected to channel 0 using the
    // I2C switch just as a normal I2C device
    let mut read_data = [0; 2];
    if let Err(e) = switch.read(slave_address, &mut read_data) {
        println!("Error received: {}", e);
    }

    // write_read from the slave connected to channel 0 using
    // the I2C switch just as a normal I2C device
    if let Err(e) = switch.write_read(slave_address, &write_data, &mut read_data) {
        println!("Error received: {}", e);
    }
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/xca9548a-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`](https://github.com/rust-embedded/embedded-hal)
