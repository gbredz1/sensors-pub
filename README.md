# sensors-pub

Application for publishing sensor measurements.

## Sensors
|Sensor | Measures 
|--------|----------
| AM2320 | Temperature & Humidity (i2c)
| DS18b20 | Temperature (1-wire)
| Faker | Generate fake measures for demo or development purpose.

## Publishers
|Publisher | Description 
|--------|----------
| Stdout | Just write to stdout.
| MQTT | Publish with support for Home Assistant MQTT discovery.

## Cross compilation

This example is for running on a Raspberry Pi Zero 2 W (ARMv7). For a Raspberry Pi Zero (ARMv6) replace `armv7-unknown-linux-gnueabihf` with `arm-unknown-linux-gnueabihf` 

### Cargo

- Add target with `rustup` :
```bash
rustup target install armv7-unknown-linux-gnueabihf
```
*Note: you may need to install cross-compiler stuff like the `gcc-arm-linux-gnueabihf` package on Debian.*

- Build with `cargo` :
```bash
cargo build --release --target armv7-unknown-linux-gnueabihf
```

### With Cross (https://github.com/cross-rs/cross) 
- Installation:
```bash
cargo install cross --git https://github.com/cross-rs/cross
```
- build with `cross` :
```bash
cross build --release --target armv7-unknown-linux-gnueabihf
```