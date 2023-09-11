# embassy_template
embassy template

## Target
This template is created for uK STM32F103RB, target thumbv7m-none-eabi.

If you wwant to change this, then edit this files:
- `.cargo/config.toml`,
- `.vscode/settings.json`,
- `Cargo.toml`.

Links:
- https://github.com/probe-rs/probe-rs/tree/master/probe-rs/targets,
- https://github.com/embassy-rs/embassy/blob/main/embassy-stm32/Cargo.toml

## tick timer
Tick timer is set on 1000Hz. 
If you wwant to change this, then edit this files:
- `Cargo.toml`.

Links:
- https://github.com/embassy-rs/embassy/blob/main/embassy-time/Cargo.toml