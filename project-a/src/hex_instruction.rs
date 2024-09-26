use core::ops::Add;

pub struct HexInstruction {
    pub byte_count: u8,
    pub address: u16,
    pub record_type: u8,
    pub data: u16,
    pub checksum: u8,
    pub state: HexInstructionReadState,
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
            state: HexInstructionReadState::Start,
        }
    }

    pub fn init(&mut self) {
        self.byte_count = 0;
        self.address = 0;
        self.record_type = 0;
        self.data = 0;
        self.checksum = 0;
    }

    pub fn verify_checksum(&self) -> bool {
        let mut sum: u16 = 0;
        sum = sum.add(self.byte_count as u16);
        sum = sum.add((self.address >> 8) & 0xFF);
        sum = sum.add(self.address & 0xFF);
        sum = sum.add(self.record_type as u16);
        sum = sum.add((self.data >> 8) & 0xFF);
        sum = sum.add(self.data & 0xFF);

        let lsb = (sum & 0xFF) as u8;
        let checksum = (!lsb + 1) & 0xFF;

        checksum == self.checksum
    }

    pub fn check_end_of_file(&self) -> bool {
        self.record_type == 01
    }
}
