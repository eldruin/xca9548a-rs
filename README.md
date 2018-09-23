# Rust TCA9548A and PCA9548A I2C Switch Driver

This is a platform agnostic Rust driver for the for TCA9548A and PCA9548A I2C
switches/multiplexers, based on the
[`embedded-hal`](https://github.com/rust-embedded/embedded-hal) traits.

This driver allows you to:
- Enable one or multiple I2C channels.
- Communicate with the slaves connected to the enabled channels transparently.

## The devices
TODO

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

