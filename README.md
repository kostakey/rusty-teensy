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

## VSCode workflow

There is a package we can install in VSCode called `rust-analyzer` which is pretty neat. However, it is a bit finiky as you need to add some configuration for it to interpret properly. In your `.vscode/settings.json` file add the following two lines:

```json
{
    "rust-analyzer.cargo.target": "thumbv7em-none-eabihf",
    "rust-analyzer.checkOnSave.allTargets": false
}
```

## Update and Build

Update the codebase including the external packages/dependencies:

`cargo update`

Compile!

`cargo build`

Oh yeah baby, binary time.

## Create the HEX file

To create the HEX file, we need to be able to convert the built ELF file that Rust spits out into HEX format. To do that, we need to install some extra cargo utilities by executing the following:

`cargo install cargo-binutils`

`rustup component add llvm-tools`

This enables us to copy the ELF into a HEX file that the Teensyloader can recognize. 

Create the HEX file that the Teensy loader can recactually recognize and use:

`cargo objcopy --release -- -O ihex rust-teensy-blink.hex`

I have since created a shell script for the update, build, and HEX file convertion to make the MCU flashing quicker, not included in this repo. However, for general syntax/compile time errors you will be stuck at `cargo build`. This is where you will be spending the majority of your time.

## Teensy Loader

I'm on Ubuntu, so for ease of use I use the out-of-the-box [Teensy Loader](https://www.pjrc.com/teensy/loader_linux.html), just without the Arduino IDE. Make sure to follow the instructions on the aforementioned link for the ruleset stuff, *only applicable for Linux*.

## minicom

There is an Ubuntu package called minicom which allows you to screen/log a USB device (UART communication).

`sudo apt install minicom`

To see what devices are connected so you can monitor the correct one:

`sudo dmesg | grep tty`

In my case, my target device was at `ttyACM1` at a baud rate of 115200, so my command line input was the following:

`sudo minicom -b 115200 -o -D /dev/ttyACM1`

My output, as an example:

```
[INFO rust_teensy_blink]: Hello from the USB logger! The count is 1220
[INFO rust_teensy_blink]: Hello from the USB logger! The count is 1221
[INFO rust_teensy_blink]: Hello from the USB logger! The count is 1222
[INFO rust_teensy_blink]: Hello from the USB logger! The count is 1223
[INFO rust_teensy_blink]: Hello from the USB logger! The count is 1224
[INFO rust_teensy_blink]: Hello from the USB logger! The count is 1225
[INFO rust_teensy_blink]: Hello from the USB logger! The count is 1226
[INFO rust_teensy_blink]: Hello from the USB logger! The count is 1227
[INFO rust_teensy_blink]: Hello from the USB logger! The count is 1228
[INFO rust_teensy_blink]: Hello from the USB logger! The count is 1229
[INFO rust_teensy_blink]: Hello from the USB logger! The count is 1230
[INFO rust_teensy_blink]: Hello from the USB logger! The count is 1231
[INFO rust_teensy_blink]: Hello from the USB logger! The count is 1232
[INFO rust_teensy_blink]: Hello from the USB logger! The count is 1233
[INFO rust_teensy_blink]: Hello from the USB logger! The count is 1234
[INFO rust_teensy_blink]: Hello from the USB logger! The count is 1235
[INFO rust_teensy_blink]: Hello from the USB logger! The count is 1236
[INFO rust_teensy_blink]: Hello from the USB logger! The count is 1237                             
```

To exit the window it brings up, do Ctrl+A, then Shift+Q (just like vim), then Enter.

## Non-blocking hardware timer

A non-blocking timer is powerful for asynchronous actions. For example, blinking an LED at a different rate than what is being logged over USB. This ensures the CPU is always active while different processes are occuring. It would really suck if you were to lose input data due to the CPU halting because you used a whole system interrupt. Luckily, the Teensy has multiple Peripheral Interrupt Timer (PIT) channels to satisfy this issue. My blinking example uses a PIT for asynchronization across the board *hehe*. Now imagine how this would be useful for protocols such as SPI or CAN :)
