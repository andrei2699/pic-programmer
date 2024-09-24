#![no_std]
#![no_main]

mod hex_instruction;
mod driver;

use crate::driver::operations::ProgramMemory;
use crate::driver::programmer::Programmer;
use crate::hex_instruction::HexInstruction;
use arduino_hal::hal::port::PB1;
use arduino_hal::hal::{Atmega, Usart};
use arduino_hal::pac::TC1;
use arduino_hal::port::mode::{Floating, Input};
use arduino_hal::port::Pin;
use arduino_hal::prelude::*;
use arduino_hal::simple_pwm::*;
use arduino_hal::usart::UsartOps;
use arduino_hal::{pins, Peripherals};
#[allow(unused_imports)]
use panic_halt as _;

const OK_INSTRUCTION: &'static str = "Y";
const RESEND_INSTRUCTION: &'static str = "R";

#[arduino_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut led = pins.d13.into_output();

    let mut programmer = Programmer::new(
        pins.d6.into_output(),
        pins.d3.into_output(),
        pins.d4.into_output(),
        pins.d5.into_output(),
    );

    programmer.init();
    setup_pwm_for_12v_charge_pump(dp.TC1, pins.d9);

    ufmt::uwrite!(&mut serial, "Programmer ready!").unwrap_infallible();

    let mut finished_programming = false;

    loop {
        if finished_programming {
            led.toggle();
            arduino_hal::delay_ms(1000);
            continue;
        }

        // TODO: initialize PIC microcontroller

        let current_instruction = parse_instruction(&mut serial);

        if current_instruction.check_end_of_file() {
            finished_programming = true;
            continue;
        }

        if current_instruction.verify_checksum() {
            ufmt::uwrite!(&mut serial, "{}", OK_INSTRUCTION).unwrap_infallible();
        } else {
            ufmt::uwrite!(&mut serial, "{}", RESEND_INSTRUCTION).unwrap_infallible();
        }

        // TODO: write instruction to PIC microcontroller
    }
}

fn parse_instruction<USART, RX, TX, CLOCK>(serial: &mut Usart<USART, RX, TX, CLOCK>) -> HexInstruction
where
    USART: UsartOps<Atmega, RX, TX>,
{
    let mut current_instruction = HexInstruction::new();
    if let Ok(_) = serial.read() {
        // ignore START CODE ':'
    }
    if let Ok(byte) = serial.read() {
        current_instruction.byte_count = byte;
    }

    if let Ok(byte) = serial.read() {
        current_instruction.address = byte as u16;
    }
    if let Ok(byte) = serial.read() {
        current_instruction.address = (current_instruction.address << 2) | byte as u16;
    }

    if let Ok(byte) = serial.read() {
        current_instruction.record_type = byte;
    }

    for index in 0..current_instruction.byte_count {
        if let Ok(byte) = serial.read() {
            current_instruction.data[index as usize] = byte;
        }
    }

    if let Ok(byte) = serial.read() {
        current_instruction.checksum = byte;
    }
    current_instruction
}

fn setup_pwm_for_12v_charge_pump(tc1: TC1, pwm_pin: Pin<Input<Floating>, PB1>) {
    let timer1 = Timer1Pwm::new(tc1, Prescaler::Prescale8);

    let mut pin = pwm_pin.into_output().into_pwm(&timer1);
    pin.enable();
    pin.set_duty(127);
}
