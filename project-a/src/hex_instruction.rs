use core::ops::Add;

pub struct HexInstruction {
    pub byte_count: u8,
    pub address: u16,
    pub record_type: u8,
    pub data: u16,
    pub checksum: u8,
}

impl HexInstruction {
    pub fn new() -> HexInstruction {
        HexInstruction {
            byte_count: 0,
            address: 0,
            record_type: 0,
            data: 0,
            checksum: 0,
        }
    }

    pub fn verify_checksum(&self) -> bool {
        let mut sum: u8 = 0;
        sum = sum.add(self.byte_count);
        sum = sum.add(((self.address >> 2) & 0xFF) as u8);
        sum = sum.add((self.address & 0xFF) as u8);
        sum = sum.add(self.record_type);
        sum = sum.add(((self.data >> 2) & 0xFF) as u8);
        sum = sum.add((self.data & 0xFF) as u8);

        let lsb = sum & 0xFF;
        let checksum = (!lsb + 1) & 0xFF;

        checksum == self.checksum
    }

    pub fn check_end_of_file(&self) -> bool {
        self.record_type == 01
    }
}