#![no_std]
#![no_main]

// cargo PATH
// $HOME/.cargo/env

// hardware access layer (HAL) includes the register access layer (RAL)

use bsp::board;
use teensy4_bsp as bsp; // the board support package (hal) used to interface with the hardware

// use core::panic::PanicInfo;
use teensy4_panic as _; // panic handler that blinks the LED (SOS)

// use bsp::hal::timer::Blocking; // shitty because it blocks

// formatting and printing strings
// use core::fmt::Write as _;

// change to vary the baud rate.
// const UART_BAUD: u32 = 115200; // baud rate for serial comms

// PIT freq (1MHz) / 1000 * 500 --> about 500ms in ticks
// const DELAY_MS: u32 = board::PERCLK_FREQUENCY / 1_000 * 500;  // LED toggle threshold

#[bsp::rt::entry]
fn main() -> ! {
    
    let instances = board::instances();

    // exposes board resources we will be using (Teensy 4.1 in this case)
    let board::Resources{
        // objects of pins on teensy
        pins,
        // // hardware timer, used for blocking delays
        mut gpt1,
        // mut pit, // the Peripheral Interrupt Timer .... used for interrupts
        // used to configure the led as a gpio output
        mut gpio2,
        // low-level usb resources
        usb,
        // UART we are creating
        // lpuart2,
        ..
    } = board::t41(instances);

    // ONLY works when logging is enabled in the .toml file
    bsp::LoggingFrontend::default_log().register_usb(usb);

    // configures the LED as an output
    let led = board::led(&mut gpio2, pins.p13);
    
    // the non-blocking interrupting count down (good so the CPU doesn't halt)
    // aka peripheral interrupt
    // pit.0.set_load_timer_value(DELAY_MS);
    // pit.0.enable();

    gpt1.disable();
    gpt1.set_divider(GPT1_DIVIDER);
    gpt1.set_clock_source(GPT1_CLOCK_SOURCE);
    gpt1.enable();

    // nah we dont wanna block
    // let mut timer = Blocking::<_, GPT1_FREQUENCY>::from_gpt(gpt1);
    
    // let mut lpuart2: board::Lpuart2 = board::lpuart(lpuart2, pins.p14, pins.p15, UART_BAUD);
    // let lpuart2: &mut dyn embedded_hal::serial::Write<u8, Error = _> = &mut lpuart2;

    let mut counter: u32 = 0;
    let mut prev_time: u32 = 0;

    loop{
        
        let curr_time = gpt1.count();
        
        // if pit.0.is_elapsed(){
        //     pit.0.clear_elapsed();
        //     led.toggle();
        // }
        
        log::info!("The ongoing timer count: {curr_time}");
        
        // LED blinks every sec
        // 1000ms, so 1 second
        if (curr_time - prev_time >= 1000) && (curr_time > 0) {
            prev_time = curr_time;
            led.toggle();
        }

        log::info!("The prev timer value: {prev_time}");
        
        let time_difference = prev_time - curr_time;
        log::info!("The time difference: {time_difference}");

        log::info!("Hello from the USB logger! The count is {counter}");
        
        // when using the separate UART ports on the board (Rx/Tx)
        // write!(
        //     lpuart2,
        //     "Hello from the UART driver! The count is {counter}\r\n"
        // )
        // .ok();
        
        counter = counter.wrapping_add(1);

    }

}

// We're responsible for configuring our timers.
// This example uses PERCLK_CLK as the GPT1 clock source,
// and it configures a 1 KHz GPT1 frequency by computing a
// GPT1 divider.
use bsp::hal::gpt::ClockSource;

/// The intended GPT1 frequency (Hz).
/// we running at 1ms babyyy
const GPT1_FREQUENCY: u32 = 1_000;
/// Given this clock source...
const GPT1_CLOCK_SOURCE: ClockSource = ClockSource::HighFrequencyReferenceClock;
/// ... the root clock is PERCLK_CLK. To configure a GPT1 frequency,
/// we need a divider of...
const GPT1_DIVIDER: u32 = board::PERCLK_FREQUENCY / GPT1_FREQUENCY;