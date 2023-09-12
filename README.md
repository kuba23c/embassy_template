# embassy_template
[Embassy](https://github.com/embassy-rs/embassy.git) rust template for STM32 devices.
Can be used with `cargo generate`.

## Hardware
Created for NUCLEO-F103RB board but can be modified and used in any STM32 device.

## Install
If you are new in rust embedded then follow this rust installation guide:
[rust-embedded book](https://docs.rust-embedded.org/book/intro/install.html)

Installing this repo:
`cargo generate --git https://github.com/kuba23c/embassy_template`

## Run
- `cargo build`
- `cargo run`

To build/run `release` version just add `--release` to command.

Sometimeas there is error when connecting second time to target. 
If so, add `--connect-under-reset` to your runner in `.cargo/config.toml`.

## Project configuration
### Target
This template is created for uK STM32F103RB, target thumbv7m-none-eabi.

If you want to change this, then edit this files:
- `.cargo/config.toml`,
- `.vscode/settings.json`,
- `Cargo.toml`.

Useful links:
- https://github.com/probe-rs/probe-rs/tree/master/probe-rs/targets,
- https://github.com/embassy-rs/embassy/blob/main/embassy-stm32/Cargo.toml

### tick timer
Tick timer is set on 1000Hz. 
If you want to change this, then edit this file:
- `Cargo.toml`.

Useful link:
- https://github.com/embassy-rs/embassy/blob/main/embassy-time/Cargo.toml

## Limitations
File `rust-toolchain.toml` is copy of `embassy/rust-toolchain.toml`.
If Embassy change toolchain (contents of files are different) than you have to copy `embassy/rust-toolchain.toml` to `rust-toolchain.toml`.

## Inspiration
Based on [cortex-m-quickstart](https://github.com/rust-embedded/cortex-m-quickstart.git) and [embassy-rp-skeleton](https://github.com/SupImDos/embassy-rp-skeleton.git).

