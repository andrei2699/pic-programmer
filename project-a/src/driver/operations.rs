pub trait InitProgrammer {
    fn init(&mut self);
}

pub trait ProgramMemory: InitProgrammer {
    fn start_programming(&mut self);

    fn program(&mut self, address: u16, data: u16);

    fn stop_programming(&mut self, config: u16, user_id: u16);
}

pub struct MemoryData {
    pub address: u16,
    pub data: u16,
}

pub trait ReadMemory: InitProgrammer {
    fn start_reading(&mut self);

    fn read(&mut self) -> MemoryData;

    fn stop_reading(&mut self);
}
