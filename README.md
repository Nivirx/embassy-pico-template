# Template for embassy rp2040 projects

This template is intended as a starting point for writing your own firmware based on the embassy libraries for the rp2040 w/ the CYW43 wireless module (rPi PicoW)

It includes all of the `knurling-rs` tooling as showcased in <https://github.com/knurling-rs/app-template> (`defmt`, `defmt-rtt`, `panic-probe`, `flip-link`) to make development as easy as possible.

`probe-rs` in SWD mode is configured as the default runner, so you can run your binary with

```sh
cargo run --release
```

## Requirements
  
- The standard Rust tooling (cargo, rustup) which you can install from <https://rustup.rs/>

- Toolchain support for the cortex-m0+ processors in the rp2040 (thumbv6m-none-eabi)

- flip-link - this allows you to detect stack-overflows on the first core, which is the only supported target for now.

- (by default) A [`probe-rs` installation](https://probe.rs/docs/getting-started/installation/)

- A [`probe-rs` compatible](https://probe.rs/docs/getting-started/probe-setup/) probe

## Installation of development dependencies

```sh
rustup target install thumbv6m-none-eabi
cargo install flip-link
# Installs the probe-rs tools, including probe-rs run, our recommended default runner
cargo install --locked probe-rs-tools
# If you want to use elf2uf2-rs instead, do...
cargo install --locked elf2uf2-rs
```

If you get the error ``binary `cargo-embed` already exists`` during installation of probe-rs, run `cargo uninstall cargo-embed` to uninstall your older version of cargo-embed before trying again.

## Running

For a debug build

```sh
cargo run
```

For a release build

```sh
cargo run --release
```

If you do not specify a DEFMT_LOG level, it will be set to `debug`.
That means `println!("")`, `info!("")` and `debug!("")` statements will be printed.
If you wish to override this, you can change it in `.cargo/config.toml`

```toml
[env]
DEFMT_LOG = "off"
```

You can also set this inline (on Linux/MacOS)  

```sh
DEFMT_LOG=trace cargo run
```

## Flashing CYW43 WiFI+BT module firmware

If you wish you use the on-board LED or the WiFi/BT module you will need to flash the module firmware.

`NOTE: By default firmware is placed at the 1MiB mark in ROM`

For WiFi + Bluetooth firmware

```sh
./flashwb.sh
```

For a WiFi only install (update base addresses in main.rs)

```sh
./flashw.sh
```

The Code in main.rs is expecting the firmware at these locations by default.

```Rust
const WIFI_FIRMWARE_BASE: u32 = 0x1010_0000;
const BT_FIRMWARE_BASE: u32 = 0x1014_0000;
const CLM_FIRMWARE_BASE: u32 = 0x1014_4000;
```

or

```Rust
const WIFI_FIRMWARE_BASE: u32 = 0x1010_0000;
const CLM_FIRMWARE_BASE: u32 = 0x1014_0000;
```
