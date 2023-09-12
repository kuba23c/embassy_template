# embassy_template
Created for NUCLEO-F103RB board.

## Install
If you are new in rust embedded than follow this rust installation guide:
https://docs.rust-embedded.org/book/intro/install.html

Installing this repo:
`git clone --recurse-submodules git@github.com:kuba23c/embassy_template.git`

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

### gpio
- user button - connected to PC13,
- user LED - connected to PA5.
