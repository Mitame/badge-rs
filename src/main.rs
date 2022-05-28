//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::*;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::prelude::*;
use embedded_time::rate::*;
// use panic_probe as _;
use panic_halt as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::pac::interrupt;
#[allow(unused_imports)]
use bsp::hal::prelude::*;
use bsp::hal::{
    self,
    clocks::{init_clocks_and_plls, Clock},
    gpio, pac,
    sio::Sio,
    spi,
    watchdog::Watchdog,
    Spi, Timer,
};

use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;

mod uc8151;
use crate::uc8151::Uc8151;

use numtoa::NumToA;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    // Setup USB
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    // Set up the USB Communications Class Device driver
    let mut serial = SerialPort::new(&usb_bus);

    // Create a USB device with a fake VID and PID
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build();

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let spi = Spi::<_, _, 8>::new(pac.SPI0);

    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        12_000_000u32.Hz(),
        &embedded_hal::spi::MODE_0,
    );

    let mut enable_3v3_pin = pins.gpio10.into_push_pull_output();
    enable_3v3_pin.set_high().unwrap();

    let mut led_pin = pins.led.into_push_pull_output();

    led_pin.set_high().unwrap();
    let mut display = Uc8151::new(
        pins.gpio18, // sclk
        pins.gpio19, // mosi
        pins.gpio20, // dc
        pins.gpio17, // cs
        pins.gpio26, // busy
        pins.gpio21, // reset
        spi,
        delay,
    );
    display.setup();

    led_pin.set_low().unwrap();

    let mut buf = [0u8; 2];
    display.command_read(0x42, &mut buf);
    let temperature = u16::from_be_bytes(buf);

    display.power_on(true);
    // PTOU
    display.command(0x92, &[]);

    loop {
        display.busy_wait();
        // DTM2
        display.data_start_transmission_2((0..4736).map(|_| &0xff));
        // DSP
        let _has_stopped = display.data_stop();
        display.display_refresh(true);
    }
}
