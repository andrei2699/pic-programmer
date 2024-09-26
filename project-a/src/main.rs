#![no_std]
#![no_main]

mod hex_instruction;
mod driver;
mod state;

use crate::driver::operations::ProgramMemory;
use crate::driver::programmer::Programmer;
use crate::driver::special_addresses::{CONFIGURATION_WORD_ADDRESS, USER_ID_FIRST_ADDRESS};
use crate::hex_instruction::HexInstruction;
use crate::hex_instruction::HexInstructionReadState::{Address, ByteCount, Checksum, Data, Done, RecordType, Start};
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

    ufmt::uwrite!(&mut serial, "Programmer ready!").unwrap_infallible();

    let mut state = States::WaitingToStart;
    let mut config = DEFAULT_CONFIGURATION;
    let mut user_id = DEFAULT_USER_ID;
    let mut current_instruction = HexInstruction::new();

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
                    }
                }
            }
            States::Program => {
                parse_instruction(&mut serial, &mut current_instruction);

                if current_instruction.state != Done {
                    continue;
                }
                current_instruction.state = Start;

                if current_instruction.check_end_of_file() {
                    state = States::Finished;
                    programmer.stop_programming(config, user_id);
                    continue;
                } else if current_instruction.address == USER_ID_FIRST_ADDRESS {
                    user_id = current_instruction.data & 0xFF;
                } else if current_instruction.address == CONFIGURATION_WORD_ADDRESS {
                    config = current_instruction.data & 0xFF;
                }

                if current_instruction.verify_checksum() {
                    ufmt::uwrite!(&mut serial, "{}", OK_INSTRUCTION).unwrap_infallible();
                } else {
                    ufmt::uwrite!(&mut serial, "{}", RESEND_INSTRUCTION).unwrap_infallible();
                }

                programmer.program(current_instruction.address, current_instruction.data)
            }
        }
    }
}

fn parse_instruction<USART, RX, TX, CLOCK>(serial: &mut Usart<USART, RX, TX, CLOCK>, current_instruction: &mut HexInstruction)
where
    USART: UsartOps<Atmega, RX, TX>,
{
    if let Ok(byte) = serial.read() {
        if byte == b'\r' || byte == b'\n' {
            return;
        }

        let byte = convert_byte_to_hexadecimal_if_possible(byte);

        match current_instruction.state {
            Start => {
                if byte == b':' {
                    // ignore START CODE ':'
                    current_instruction.init();
                    current_instruction.state = ByteCount(2);
                }
            }
            ByteCount(hex_digits_remaining) => {
                if hex_digits_remaining > 1 {
                    current_instruction.state = ByteCount(hex_digits_remaining - 1);
                } else {
                    current_instruction.state = Address(4);
                }
                current_instruction.byte_count = (current_instruction.byte_count) << 4 | byte;
            }
            Address(hex_digits_remaining) => {
                if hex_digits_remaining > 1 {
                    current_instruction.state = Address(hex_digits_remaining - 1)
                } else {
                    current_instruction.state = RecordType(2)
                }
                current_instruction.address = (current_instruction.address << 4) | byte as u16;
            }
            RecordType(hex_digits_remaining) => {
                if hex_digits_remaining > 1 {
                    current_instruction.state = RecordType(hex_digits_remaining - 1);
                } else {
                    current_instruction.state = Data(current_instruction.byte_count * 2);
                }
                current_instruction.record_type = (current_instruction.record_type) << 4 | byte;
            }
            Data(hex_digits_remaining) => {
                if hex_digits_remaining > 1 {
                    current_instruction.state = Data(hex_digits_remaining - 1);
                } else {
                    current_instruction.state = Checksum(2);
                }
                current_instruction.data = (current_instruction.data << 4) | byte as u16;
            }
            Checksum(hex_digits_remaining) => {
                if hex_digits_remaining > 1 {
                    current_instruction.state = Checksum(hex_digits_remaining - 1);
                } else {
                    current_instruction.state = Done;
                }
                current_instruction.checksum = (current_instruction.checksum) << 4 | byte;
            }
            Done => {}
        }
    }
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