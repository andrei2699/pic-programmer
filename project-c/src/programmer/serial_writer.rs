use serialport::SerialPort;
use std::io;

pub struct SerialWriter {}

pub trait WriteSerial {
    fn write(&mut self, port: &mut Box<dyn SerialPort>, buffer: &[u8]);
}

impl SerialWriter {
    pub fn new() -> SerialWriter {
        SerialWriter {}
    }
}

impl WriteSerial for SerialWriter {
    fn write(&mut self, port: &mut Box<dyn SerialPort>, buffer: &[u8]) {
        match port.write(buffer) {
            Ok(_) => {
                print!("{}", String::from_utf8_lossy(buffer));
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
