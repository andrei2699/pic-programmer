#![no_std]
#![no_main]

mod driver;
mod hex_instruction;
mod state;

use crate::driver::operations::{InitProgrammer, ProgramMemory, ReadMemory};
use crate::driver::programmer::Programmer;
use crate::driver::special_addresses::{CONFIGURATION_WORD_ADDRESS, USER_ID_FIRST_ADDRESS};
use crate::hex_instruction::HexInstruction;
use crate::state::States;
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

const OK_INSTRUCTION: u8 = b'Y';
const RESEND_INSTRUCTION: u8 = b'R';
const PROGRAM_INSTRUCTION: u8 = b'P';
const READ_STORED_PROGRAM_INSTRUCTION: u8 = b'D';
const DEFAULT_CONFIGURATION: u16 = 0xFF;
const DEFAULT_USER_ID: u16 = 0xAA;

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

    ufmt::uwriteln!(&mut serial, "Programmer ready!").unwrap_infallible();

    let mut state = States::WaitingToStart;
    let mut config = DEFAULT_CONFIGURATION;
    let mut user_id = DEFAULT_USER_ID;

    loop {
        match state {
            States::Finished => {
                led.toggle();
                arduino_hal::delay_ms(1000);
            }
            States::WaitingToStart => {
                if let Ok(byte) = serial.read() {
                    if byte == PROGRAM_INSTRUCTION {
                        state = States::Program;
                        programmer.start_programming();
                        ufmt::uwriteln!(&mut serial, "start").unwrap_infallible();
                    } else if byte == READ_STORED_PROGRAM_INSTRUCTION {
                        state = States::ReadContents;
                        ufmt::uwriteln!(&mut serial, "start").unwrap_infallible();
                    }
                }
            }
            States::Program => {
                let current_instruction = parse_instruction(&mut serial);

                if current_instruction.address == USER_ID_FIRST_ADDRESS {
                    user_id = current_instruction.data & 0xFF;
                } else if current_instruction.address == CONFIGURATION_WORD_ADDRESS {
                    config = current_instruction.data & 0xFF;
                }

                let checksum = current_instruction.calculate_checksum();
                if checksum == current_instruction.checksum {
                    if current_instruction.check_end_of_file() {
                        ufmt::uwriteln!(&mut serial, "{}", OK_INSTRUCTION).unwrap_infallible();

                        programmer.stop_programming(config, user_id);
                        ufmt::uwriteln!(&mut serial, "done").unwrap_infallible();
                        state = States::Finished;
                        continue;
                    }

                    programmer.program(current_instruction.address, current_instruction.data);
                    ufmt::uwriteln!(&mut serial, "{}", OK_INSTRUCTION).unwrap_infallible();
                } else {
                    ufmt::uwriteln!(&mut serial, "{}", RESEND_INSTRUCTION).unwrap_infallible();
                }
            }
            States::ReadContents => {
                programmer.start_reading();

                let mut has_printed_configuration_address = false;
                while !has_printed_configuration_address
                    || programmer.current_address != CONFIGURATION_WORD_ADDRESS
                {
                    let data = programmer.read();
                    programmer.increment_address();
                    ufmt::uwriteln!(&mut serial, "A:{:04X} | D:{:04X}", data.address, data.data)
                        .unwrap_infallible();
                    has_printed_configuration_address = true;
                }

                programmer.stop_reading();
                ufmt::uwriteln!(&mut serial, "done").unwrap_infallible();
                state = States::Finished;
            }
        }
    }
}

fn parse_instruction<USART, RX, TX, CLOCK>(
    serial: &mut Usart<USART, RX, TX, CLOCK>,
) -> HexInstruction
where
    USART: UsartOps<Atmega, RX, TX>,
{
    let mut current_instruction = HexInstruction::new();

    while !current_instruction.check_done() {
        if let Ok(ascii_byte) = serial.read() {
            if ascii_byte == b'\r' || ascii_byte == b'\n' {
                continue;
            }

            let byte = convert_byte_to_hexadecimal_if_possible(ascii_byte);

            current_instruction.add_byte_in_state(byte);
        }
    }

    current_instruction
}

fn setup_pwm_for_12v_charge_pump(tc1: TC1, pwm_pin: Pin<Input<Floating>, PB1>) {
    let timer1 = Timer1Pwm::new(tc1, Prescaler::Prescale8);

    let mut pin = pwm_pin.into_output().into_pwm(&timer1);
    pin.enable();
    pin.set_duty(127);
}

fn convert_byte_to_hexadecimal_if_possible(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'a'..=b'f' => 10 + (byte - b'a'),
        b'A'..=b'F' => 10 + (byte - b'A'),
        _ => byte,
    }
}
