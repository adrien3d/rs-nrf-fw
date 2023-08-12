# Rust nRF Firmware
## Installing
GNU Embedded Toolchain for ARM [download](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads)
nRF command line tools [download](https://www.nordicsemi.com/Products/Development-tools/nrf-command-line-tools/download#infotabs)
rustup install [download](https://www.rust-lang.org/tools/install)

## Building
cargo build

## Flashing
cd target/thumbv7em-none-eabihf/debug/
arm-none-eabi-objcopy -O ihex rs-nrf-fw rs-nrf-fw.hex

## Monitoring
nrfjprog -e
nrfjprog --program rs-nrf-fw.hex
nrfjprog -r