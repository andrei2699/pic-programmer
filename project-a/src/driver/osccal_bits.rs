use super::programmer::Programmer;
use crate::driver::operations::ProgramMemory;
use crate::driver::special_addresses::{BACKUP_OSCCAL_ADDRESS, OSCCAL_ADDRESS};

pub struct OSCCALBits {
    pub bits: u8,
    pub backup_bits: u8,
}

impl Programmer {
    pub fn read_and_save_osccal_bits(&mut self) {
        self.enter_programming_mode();

        while self.current_address != OSCCAL_ADDRESS {
            self.increment_address();
        }

        let bits = self.read_data();

        while self.current_address != BACKUP_OSCCAL_ADDRESS {
            self.increment_address();
        }

        let backup_bits = self.read_data();

        self.exit_programming_mode();

        self.osccal_bits = OSCCALBits {
            bits: (bits & 0xFF) as u8,
            backup_bits: (backup_bits & 0xFF) as u8,
        }
    }

    pub fn restore_osccal_bits(&mut self) {
        self.enter_programming_mode();

        while self.current_address != OSCCAL_ADDRESS {
            self.increment_address()
        }

        // should write the osccal bits as the operand of a MOVLWF instruction
        // 1100 kkkk kkkk
        let data: u16 = (0b1100 << 8) | self.osccal_bits.bits as u16;
        self.program(data);

        while self.current_address != BACKUP_OSCCAL_ADDRESS {
            self.increment_address();
        }

        self.load_data(self.osccal_bits.backup_bits as u16);

        self.exit_programming_mode()
    }
}
