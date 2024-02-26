# rusty-teensy

Here is where I will begin to attempt to embed rust onto a Teensy 4.1. This is more so for me to keep log on what I install and whatnot, but feel free to tag along (if you dare).

[Goated Example](https://github.com/mciantyre/teensy4-rs-template/tree/master)

## Rust!

Install Rust:

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Follow the instructions from there to complete the installation, the defined PATH is the important bit, the default PATH is fine.

Then, add the target:

`rustup target add thumbv7em-none-eabihf`

The Teensy 4.1 uses an NXP iMRXT1062 chip with the ARM Cortex-M7 core, so it uses the Thumb (2) instruction set (shout out 331).

## Create a cargo package

Create the base directories for an embedded rust project:

`cargo new rusty-teensy-blink`

## Cargo.toml

Configure your dependencies, [here is a good resource](https://crates.io/crates/teensy4-bsp).

### .cargo config file

Rust needs to know what target to compile for, so we will define it here. We are compiling for the M7 which has the Thumb instruction set, as mentioned above. As a note, regular Arduinos are on the stinky AVR instruction set. 

[Another decent resource](https://imxrt-rs.github.io/book/toolchain.html)

## Update and Build

Update the codebase including the external packages/dependencies:

`cargo update`

Compile!

`cargo build`

Oh yeah baby, binary time.

## Create HEX file

Create the HEX file that the Teensy loader can recactually recognize and use:

`cargo objcopy --release -- -O ihex rust-teensy-blink.hex`

I have since created a shell script for the update, build, and HEX file convertion to make the MCU flashing quicker, not included in this repo. However, for general syntax/compile time errors you will be stuck at `cargo build`. This is where you will be spending the majority of your time.

## Teensy Loader

I'm on Ubuntu, so for ease of use I use the out-of-the-box [Teensy Loader](https://www.pjrc.com/teensy/loader_linux.html), just without the Arduino IDE. Make sure to follow the instructions on the aforementioned link for the ruleset stuff, *only applicable for Linux*.
