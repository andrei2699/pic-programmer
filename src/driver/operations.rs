pub trait ProgramMemory {
    fn init(&mut self);

    fn start_programming(&mut self);

    fn goto_to_address(&mut self, address: u16);

    fn program(&mut self, data: u16);

    fn stop_programming(&mut self, config: u8, user_id: u8);
}
