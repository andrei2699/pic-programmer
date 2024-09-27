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
        println!(
            "[CLI] writing: '{}' ({} bytes)",
            String::from_utf8_lossy(buffer),
            buffer.len()
        );

        while bytes_written < buffer_len {
            match port.write(&buffer[bytes_written..bytes_written + 1]) {
                Ok(n) => {
                    bytes_written += n;
                    // TODO: enable more logging with a debug option from cli
                    // println!("[CLI] bytes written '{}', total {}", n, bytes_written);
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
