#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use heapless::String;

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Programmer ready!\r").unwrap_infallible();

    let mut buffer: String<32> = String::new();

    loop {
        if let Ok(byte) = serial.read() {
            buffer.push(byte as char).unwrap_or_default();

            if byte == b'\n' {
                ufmt::uwriteln!(&mut serial, "Got: {}\r", buffer.as_str()).unwrap_infallible();
                buffer.clear();
            }
        }
    }
}
