use super::programmer::Programmer;
use crate::driver::operations::ProgramMemory;
use crate::driver::special_addresses::{ADDRESS_SIZE, CONFIGURATION_WORD_ADDRESS};
use crate::driver::timing_configurations::{T_DIS, T_DLY2, T_ERA, T_HLD0, T_HLD1, T_PPDP, T_PROG, T_RESET, T_SET};
use arduino_hal::hal::port::PD5;
use arduino_hal::port::mode::{Input, PullUp};
use arduino_hal::port::Pin;

pub const LOAD_DATA_COMMAND: u8 = 0b00_0010;
pub const READ_DATA_COMMAND: u8 = 0b00_0100;
pub const INCREMENT_ADDRESS_COMMAND: u8 = 0b00_0110;
pub const BEGIN_PROGRAMMING_COMMAND: u8 = 0b00_1000;
pub const END_PROGRAMMING_COMMAND: u8 = 0b00_1110;
pub const BULK_ERASE_COMMAND: u8 = 0b00_1001;

const START_BIT: u8 = 0;
const DATA_DONT_CARE_BIT: u8 = 0;
const STOP_BIT: u8 = 0;

impl Programmer {
    fn send_serial_lsb_data(&mut self, data: u8) {
        if (data & 1) == 1 {
            self.data.take().expect("Pin should be available").set_high();
        } else {
            self.data.take().expect("Pin should be available").set_low();
        }

        self.clock.set_high();
        arduino_hal::delay_us(T_SET.to_micros());
        self.clock.set_low();

        arduino_hal::delay_us(T_HLD1.to_micros());
    }

    fn read_serial_lsb_data(&mut self, data_in: &Pin<Input<PullUp>, PD5>) -> u8 {
        self.clock.set_high();
        arduino_hal::delay_us(T_SET.to_micros());
        self.clock.set_low();

        let mut value = 0;
        if data_in.is_high() {
            value = 1;
        }

        arduino_hal::delay_us(T_HLD1.to_micros());

        value
    }

    fn send_command(&mut self, command: u8) {
        let mut command_to_send: u8 = command;

        for _ in 0..6 {
            self.send_serial_lsb_data(command_to_send);
            command_to_send >>= 1;
        }
        arduino_hal::delay_us(T_DLY2.to_micros());
    }

    pub fn enter_programming_mode(&mut self) {
        self.init();
        self.vdd.set_high();
        arduino_hal::delay_us(T_PPDP.to_micros());
        self.vpp.set_high();
        arduino_hal::delay_us(T_HLD0.to_micros());

        self.current_address = CONFIGURATION_WORD_ADDRESS;
    }

    pub fn exit_programming_mode(&mut self) {
        self.init();
        arduino_hal::delay_ms(T_RESET.to_millis() as u16);
    }

    pub fn load_data(&mut self, data: u16) {
        self.send_command(LOAD_DATA_COMMAND);

        let mut data_to_send = data;
        self.send_serial_lsb_data(START_BIT);
        for _ in 0..12 {
            self.send_serial_lsb_data(data_to_send as u8);
            data_to_send >>= 1;
        }
        self.send_serial_lsb_data(DATA_DONT_CARE_BIT);
        self.send_serial_lsb_data(DATA_DONT_CARE_BIT);
        self.send_serial_lsb_data(STOP_BIT);
    }

    pub fn read_data(&mut self) -> u16 {
        self.send_command(READ_DATA_COMMAND);

        let data_in: Pin<Input<PullUp>, PD5> = self.data.take().expect("Pin should be available").into_pull_up_input();

        let mut received_data: u16 = 0;
        self.read_serial_lsb_data(&data_in); // start bit
        for index in 0..12 {
            let data = self.read_serial_lsb_data(&data_in) as u16;
            received_data = received_data | (data << index);
        }
        self.read_serial_lsb_data(&data_in); // ignored MSB 1
        self.read_serial_lsb_data(&data_in); // ignored MSB 2
        self.read_serial_lsb_data(&data_in); // stop bit

        self.data = Some(data_in.into_output());

        received_data
    }

    pub fn increment_address(&mut self) {
        self.send_command(INCREMENT_ADDRESS_COMMAND);
        self.current_address = (self.current_address + 1) % ADDRESS_SIZE;
    }

    pub fn begin_programming(&mut self) {
        self.send_command(BEGIN_PROGRAMMING_COMMAND);
        arduino_hal::delay_ms(T_PROG.to_millis() as u16);
    }

    pub fn end_programming(&mut self) {
        self.send_command(END_PROGRAMMING_COMMAND);
        arduino_hal::delay_us(T_DIS.to_micros());
    }

    pub fn bulk_erase_program_memory(&mut self) {
        self.send_command(BULK_ERASE_COMMAND);
        arduino_hal::delay_ms(T_ERA.to_millis() as u16);
    }
}
