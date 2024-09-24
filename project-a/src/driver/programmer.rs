use crate::driver::operations::ProgramMemory;
use crate::driver::osccal_bits::OSCCALBits;
use crate::driver::special_addresses::{CONFIGURATION_WORD_ADDRESS, USER_ID_FIRST_ADDRESS};
use arduino_hal::hal::port::{PD3, PD4, PD5, PD6};
use arduino_hal::port::mode::Output;
use arduino_hal::port::Pin;

pub struct Programmer {
    pub vpp: Pin<Output, PD6>,
    pub vdd: Pin<Output, PD3>,
    pub clock: Pin<Output, PD4>,
    pub data: Option<Pin<Output, PD5>>,
    pub current_address: u16,
    pub osccal_bits: OSCCALBits,
}

impl Programmer {
    pub fn new(vpp: Pin<Output, PD6>, vdd: Pin<Output, PD3>, clock: Pin<Output, PD4>, data: Pin<Output, PD5>) -> Programmer {
        Programmer {
            vpp,
            vdd,
            clock,
            data: Some(data),
            current_address: CONFIGURATION_WORD_ADDRESS,
            osccal_bits: OSCCALBits { bits: 0, backup_bits: 0 },
        }
    }
}

impl ProgramMemory for Programmer {
    fn init(&mut self) {
        self.vpp.set_low();
        self.vdd.set_low();
        self.clock.set_low();
        self.data.take().expect("Pin should be available").set_low();
    }

    fn start_programming(&mut self) {
        self.read_and_save_osccal_bits();
        self.enter_programming_mode();
        self.increment_address();
        self.bulk_erase_program_memory();
    }

    #[inline]
    fn goto_to_address(&mut self, address: u16) {
        while self.current_address != address {
            self.increment_address();
        }
    }

    fn program(&mut self, data: u16) {
        self.load_data(data);
        self.begin_programming();
        self.end_programming();
    }

    fn stop_programming(&mut self, config: u8, user_id: u8) {
        self.exit_programming_mode();
        self.restore_osccal_bits();
        self.program_configuration(config, user_id);
    }
}

impl Programmer {
    fn program_configuration(&mut self, config: u8, user_id: u8) {
        self.enter_programming_mode();
        self.goto_to_address(CONFIGURATION_WORD_ADDRESS);
        self.program(config as u16);
        self.goto_to_address(USER_ID_FIRST_ADDRESS);
        self.program(user_id as u16);
        self.exit_programming_mode();
    }
}
