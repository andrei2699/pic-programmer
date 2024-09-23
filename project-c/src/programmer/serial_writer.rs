use serialport::SerialPort;
use std::io;

pub struct SerialWriter {}

pub trait WriteSerial {
    fn write(&mut self, port: &mut Box<dyn SerialPort>, string: &str);
}

impl SerialWriter {
    pub fn new() -> SerialWriter {
        SerialWriter {}
    }
}

impl WriteSerial for SerialWriter {
    fn write(&mut self, port: &mut Box<dyn SerialPort>, string: &str) {
        match port.write(string.as_bytes()) {
            Ok(_) => {
                print!("{}", &string);
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}