use serialport::SerialPort;
use std::io;
use std::io::Write;

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
        let mut bytes_written = 0;
        let buffer_len = buffer.len();
        println!("[CLI] writing: '{}'", String::from_utf8_lossy(buffer));

        while bytes_written < buffer_len {
            match port.write(&buffer[bytes_written..]) {
                Ok(n) => {
                    bytes_written += n;
                }
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {}
                Err(e) => {
                    eprintln!("Error writing to serial port: {:?}", e);
                    break;
                }
            }
        }
    }
}
