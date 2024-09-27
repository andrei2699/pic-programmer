use serialport::SerialPort;
use std::{io, str};

pub struct SerialReader {
    serial_buf: Vec<u8>,
}

pub trait ReadSerial {
    fn read(&mut self, port: &mut Box<dyn SerialPort>, received_data: &mut String);
}

impl SerialReader {
    pub fn new(serial_buf: Vec<u8>) -> SerialReader {
        SerialReader { serial_buf }
    }
}

impl ReadSerial for SerialReader {
    fn read(&mut self, port: &mut Box<dyn SerialPort>, received_data: &mut String) {
        match port.read(self.serial_buf.as_mut_slice()) {
            Ok(bytes_read) => {
                let content = &self.serial_buf[..bytes_read];

                if let Ok(chunk) = str::from_utf8(content) {
                    println!("[Programmer] raw data: '{}'", chunk);
                    received_data.push_str(chunk);
                } else {
                    panic!("[CLI] unable to convert content to string")
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => panic!("{:?}", e),
        }
    }
}
