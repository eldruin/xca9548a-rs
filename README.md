# Rust TCA9548A and PCA9548A I2C Switch Driver [![crates.io](https://img.shields.io/crates/v/xca9548a.svg)](https://crates.io/crates/xca9548a) [![Docs](https://docs.rs/xca9548a/badge.svg)](https://docs.rs/xca9548a) [![Build Status](https://travis-ci.org/eldruin/xca9548a-rs.svg?branch=master)](https://travis-ci.org/eldruin/xca9548a-rs)

This is a platform agnostic Rust driver for the for TCA9548A and PCA9548A I2C
switches/multiplexers, based on the
[`embedded-hal`](https://github.com/rust-embedded/embedded-hal) traits.

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

