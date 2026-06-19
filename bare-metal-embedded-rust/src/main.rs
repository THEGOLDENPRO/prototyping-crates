#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;

use log::LevelFilter;
use rp2040_hal::gpio::Pins;
use rp2040_hal::clocks::init_clocks_and_plls;
// for now I'm writing rp2040 specific code, later I'll abstract this
use rp2040_hal::pac::Peripherals;
use rp2040_hal::usb::UsbBus;
use rp2040_hal::{self as hal, Timer, Watchdog};

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use usb_device::bus::UsbBusAllocator;
use usb_device::device::{StringDescriptors, UsbDeviceBuilder, UsbVidPid};
use usbd_serial::SerialPort;

use crate::logger::Logger;
use crate::slice_writer::SliceWriter;

mod logger;
mod slice_writer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // for now we'll just do nothing, later I 
    // want to give some sort of notice to the user.
    loop {}
}

#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

// the rp 2040's crystal oscillates at 12 MHz.
const CRYSTAL_OSCILLATOR_FREQUENCY_HZ: u32 = 12_000_000;

#[hal::entry]
fn main() -> ! {
    // unsafe {
    //     log::set_logger_racy(&Logger)
    //         .map(|()| log::set_logger_racy(LevelFilter::Info))
    //         .unwrap();
    // }

    // PAC = Peripheral Access Crate
    let mut pac = Peripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        CRYSTAL_OSCILLATOR_FREQUENCY_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    ).unwrap();

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let usb_bus = UsbBusAllocator::new(
        UsbBus::new(
            pac.USBCTRL_REGS,
            pac.USBCTRL_DPRAM,
            clocks.usb_clock,
            true,
            &mut pac.RESETS
        )
    );

    let mut serial_port = SerialPort::new(&usb_bus);

    // temporary vid and pid for dev
    let temp_vid_and_pid = UsbVidPid(0x16c0, 0x27dd);

    let mut usb_device = UsbDeviceBuilder::new(&usb_bus, temp_vid_and_pid)
        .strings(&[
            StringDescriptors::default()
                .product("Serial Console")
        ])
        .unwrap()
        .device_class(2) // 2 is for serial communication: https://www.usb.org/defined-class-codes
        .build();

    let mut receive_buffer = [0; 64]; // aka rx buf

    let mut last_timestamp = timer.get_counter();

    loop {
        if usb_device.poll(&mut [&mut serial_port]) {
            match serial_port.read(&mut receive_buffer) {
                Ok(0) => {},
                Ok(bytes_read) => {
                    let mut bytes_buffer = [0u8; 256];
                    let mut slice_writer = SliceWriter::new(&mut bytes_buffer);

                    let _ = serial_port.write(b"\x1B[2J\x1B[H");

                    slice_writer.write_bytes(&receive_buffer);
                    slice_writer.write_str("meow");

                    let _ = serial_port.write(slice_writer.as_bytes());
                },
                Err(error) => {
                    // let mut bytes_buffer = [0u8; 64];
                    // let mut slice_writer = SliceWriter::new(&mut bytes_buffer);

                    // let _ = writeln!(&mut slice_writer, "Error: {:?}", error);

                    // let _ = serial_port.write(slice_writer.as_bytes());
                }
            }
        }

        // let current_timestamp = timer.get_counter();

        // if (current_timestamp - last_timestamp).to_secs() >= 1 {
        //     last_timestamp = current_timestamp;

        //     let _ = serial_port.write(b"MEOW!!!\r\n");
        // }
    }

    // let sio = Sio::new(pac.SIO);

    // let pins = Pins::new(
    //     pac.IO_BANK0,
    //     pac.PADS_BANK0,
    //     sio.gpio_bank0,
    //     &mut pac.RESETS,
    // );

    // // this is the little LED on the rp2040 board itself.
    // let mut board_led_pin = pins.gpio25.into_push_pull_output();

    // loop {
    //     board_led_pin.set_high().unwrap();
    //     timer.delay_ms(500);

    //     board_led_pin.set_low().unwrap();
    //     timer.delay_ms(500);
    // }
}