pub trait ProgramMemory {
    fn init(&mut self);

    fn start_programming(&mut self);

    fn program(&mut self, address: u16, data: u16);

    fn stop_programming(&mut self, config: u16, user_id: u16);
}
