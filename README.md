# sensors-pub

Application for publishing sensor measurements.

## Sensors

| Sensor  | Measures                                                |
| ------- | ------------------------------------------------------- |
| AM2320  | Temperature & Humidity (i2c)                            |
| DS18b20 | Temperature (1-wire)                                    |
| Faker   | Generate fake measures for demo or development purpose. |

## Publishers

| Publisher | Description                                             |
| --------- | ------------------------------------------------------- |
| Stdout    | Just write to stdout.                                   |
| MQTT      | Publish with support for Home Assistant MQTT discovery. |

## Cross compilation

This example is for running on a Raspberry Pi Zero 2 W (ARMv7). For a Raspberry Pi Zero (ARMv6) replace `armv7-unknown-linux-gnueabihf` with `arm-unknown-linux-gnueabihf`

### Cargo

- Add target with `rustup` :

```bash
rustup target install armv7-unknown-linux-gnueabihf
```

_Note: you may need to install cross-compiler stuff like the `gcc-arm-linux-gnueabihf` package on Debian._

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

## Docker

### run
```bash
docker run -d --name sensors-pub --rm \
    -v "$(pwd)/config.yml:/app/config.yml:ro" \
    --device /dev/i2c-1 \
    --group-add $(getent group i2c | cut -d: -f3) \
    ghcr.io/gbredz1/sensors-pub:latest
```

### Compose
```yaml
services:
  app:
    image: ghcr.io/gbredz1/sensors-pub:latest
    restart: unless-stopped

    volumes:
      - ${PWD}/config.yml:/app/config.yml:ro

    group_add:
      - 115 # getent group i2c

    devices:
      - /dev/i2c-1
```