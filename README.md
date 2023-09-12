# embassy_template
Based on [cortex-m-quickstart](https://github.com/rust-embedded/cortex-m-quickstart.git).
Created for NUCLEO-F103RB board.

Provided example code contains:
- multi-threading,
- gpio input with interrupt,
- gpio output,
- mutex communication between tasks.

## Limitations
File `rust-toolchain.toml` is copy of `embassy/rust-toolchain.toml`.
If Embassy change toolchain (contents of files are different) than you have to copy `embassy/rust-toolchain.toml` to `rust-toolchain.toml`.
## Install
If you are new in rust embedded than follow this rust installation guide:
[rust-embedded book](https://docs.rust-embedded.org/book/intro/install.html)

Installing this repo:
`git clone --recurse-submodules git@github.com:kuba23c/embassy_template.git`
or
`cargo generate --git https://github.com/kuba23c/embassy_template.git`

## Run
To run example:
`cargo build --bin user_led`
`cargo run --bin user_led`

To run main program:
`cargo build --bin {project-name}`
`cargo run --bin {project-name}`

If you have only `main.rs` file:
`cargo build`
`cargo run`

To build/run `release` version just add `--release` to command.

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

### gpio for NUCLEO-F103RB board
- user button - connected to PC13,
- user LED - connected to PA5.
