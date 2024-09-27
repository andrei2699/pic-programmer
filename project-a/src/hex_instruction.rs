use crate::hex_instruction::HexInstructionReadState::{Address, ByteCount, Checksum, Data, Done, RecordType, Start};
use core::fmt::Write;
use core::ops::Add;
use heapless::String;
use ufmt::{uDisplay, uWrite, Formatter};

pub struct HexInstruction {
    pub byte_count: u8,
    pub address: u16,
    pub record_type: u8,
    pub data: u16,
    pub checksum: u8,
    state: HexInstructionReadState,
}

#[derive(PartialEq)]
pub enum HexInstructionReadState {
    Start,
    ByteCount(u8),
    Address(u8),
    RecordType(u8),
    Data(u8),
    Checksum(u8),
    Done,
}

impl HexInstruction {
    pub fn new() -> HexInstruction {
        HexInstruction {
            byte_count: 0,
            address: 0,
            record_type: 0,
            data: 0,
            checksum: 0,
            state: Start,
        }
    }

    pub fn check_done(&self) -> bool {
        self.state == Done
    }

    pub fn add_byte_in_state(&mut self, byte: u8) {
        match self.state {
            Start => {
                self.reset_state();
                self.state = ByteCount(2);
            }
            ByteCount(hex_digits_remaining) => {
                if hex_digits_remaining > 1 {
                    self.state = ByteCount(hex_digits_remaining - 1);
                } else {
                    self.state = Address(4);
                }
                self.byte_count = (self.byte_count) << 4 | byte;
            }
            Address(hex_digits_remaining) => {
                if hex_digits_remaining > 1 {
                    self.state = Address(hex_digits_remaining - 1)
                } else {
                    self.state = RecordType(2)
                }
                self.address = (self.address << 4) | byte as u16;
            }
            RecordType(hex_digits_remaining) => {
                if hex_digits_remaining > 1 {
                    self.state = RecordType(hex_digits_remaining - 1);
                } else {
                    if self.byte_count == 0 {
                        self.state = Checksum(2);
                    } else {
                        self.state = Data(self.byte_count * 2);
                    }
                }
                self.record_type = (self.record_type) << 4 | byte;
            }
            Data(hex_digits_remaining) => {
                if hex_digits_remaining > 1 {
                    self.state = Data(hex_digits_remaining - 1);
                } else {
                    self.state = Checksum(2);
                }
                self.data = (self.data << 4) | byte as u16;
            }
            Checksum(hex_digits_remaining) => {
                if hex_digits_remaining > 1 {
                    self.state = Checksum(hex_digits_remaining - 1);
                } else {
                    self.state = Done;
                }
                self.checksum = (self.checksum) << 4 | byte;
            }
            Done => {}
        }
    }

    pub fn calculate_checksum(&self) -> u8 {
        let mut sum: u16 = 0;
        sum = sum.add(self.byte_count as u16);
        sum = sum.add((self.address >> 8) & 0xFF);
        sum = sum.add(self.address & 0xFF);
        sum = sum.add(self.record_type as u16);
        sum = sum.add((self.data >> 8) & 0xFF);
        sum = sum.add(self.data & 0xFF);

        let lsb = (sum & 0xFF) as u8;
        let checksum = (!lsb + 1) & 0xFF;

        checksum
    }

    pub fn check_end_of_file(&self) -> bool {
        self.record_type == 0x01
    }

    fn reset_state(&mut self) {
        self.byte_count = 0;
        self.address = 0;
        self.record_type = 0;
        self.data = 0;
        self.checksum = 0;
        self.state = Start;
    }
}

impl uDisplay for HexInstruction {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let mut data = String::<32>::new();

        let state = match self.state {
            Start => 's',
            ByteCount(_) => 'b',
            Address(_) => 'a',
            RecordType(_) => 'r',
            Data(_) => 'd',
            Checksum(_) => 'c',
            Done => 'f'
        };

        let _ = write!(data,
                       "({}){}-{}-{}-{}-{}",
                       state,
                       self.byte_count,
                       self.address,
                       self.record_type,
                       self.data,
                       self.checksum,
        );

        f.write_str(data.as_str())
    }
}
