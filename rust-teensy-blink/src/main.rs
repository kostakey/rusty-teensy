#![no_std]
#![no_main]

// $HOME/.cargo/env

use bsp::board;
use teensy4_bsp as bsp; // the board support package (hal) used to interface with the hardware
// use teensy4_panic as _; // panic handler
use core::panic::PanicInfo;
// use teensy4_panic::sos; // panic handler
use bsp::hal::timer::Blocking; // hardware access layer (HAL) includes the register access layer (RAL)

// CHANGE ME to vary the baud rate.
// const UART_BAUD: u32 = 115200;

const DELAY_MS: u32 = 500;  // LED toggle threshold

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Your panic handler here...
    loop{}
}

#[bsp::rt::entry]
fn main() -> ! {
    
    let instances = board::instances();

    let board::Resources{
        pins,
        mut gpt1,
        mut gpio2,
        ..
    } = board::t41(instances);

    // bsp::LoggingFrontend::default_log().register_usb(usb);

    let led = board::led(&mut gpio2, pins.p13);

    gpt1.disable();
    gpt1.set_divider(GPT1_DIVIDER);
    gpt1.set_clock_source(GPT1_CLOCK_SOURCE);

    let mut delay = Blocking::<_, GPT1_FREQUENCY>::from_gpt(gpt1);
    
    // let mut lpuart2: board::Lpuart2 = board::lpuart(lpuart2, pins.p14, pins.p15, UART_BAUD);
    // let lpuart2: &mut dyn embedded_hal::serial::Write<u8, Error = _> = &mut lpuart2;

    let mut counter: u32 = 0;

    loop{
        
        led.toggle();
        // log::info!("Hello from the USB logger! The count is {counter}");
        // write!(
        //     lpuart2,
        //     "Hello from the UART driver! The count is {counter}\r\n"
        // )
        // .ok();

        delay.block_ms(DELAY_MS);
        counter = counter.wrapping_add(1);
    }

}

// We're responsible for configuring our timers.
// This example uses PERCLK_CLK as the GPT1 clock source,
// and it configures a 1 KHz GPT1 frequency by computing a
// GPT1 divider.
use bsp::hal::gpt::ClockSource;

/// The intended GPT1 frequency (Hz).
const GPT1_FREQUENCY: u32 = 1_000;
/// Given this clock source...
const GPT1_CLOCK_SOURCE: ClockSource = ClockSource::HighFrequencyReferenceClock;
/// ... the root clock is PERCLK_CLK. To configure a GPT1 frequency,
/// we need a divider of...
const GPT1_DIVIDER: u32 = board::PERCLK_FREQUENCY / GPT1_FREQUENCY;