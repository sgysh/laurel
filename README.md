# Laurel
hobby OS written in Rust for an ARM Cortex-M3.

(work in progress)

by Yoshinori Sugino

---

## License
MIT

---

## How to install qemu

    sudo apt-get update
    sudo apt-get install qemu-system-arm

## How to install xargo

    cargo install xargo

## How to install nightly

    rustup install nightly

## Set to use nightly

    cd PATH_TO_PROJECT_DIRECTORY
    rustup override set nightly

This setting is written in ~/.rustup/settings.toml

## Use UART

    e.g.) picocom -b 115200 /dev/pts/28

