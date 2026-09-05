use serialport::SerialPort;

use crate::programmer::serial_programmer::SerialProgrammer;
use crate::programmer::serial_reader::SerialReader;
use crate::programmer::serial_writer::SerialWriter;
use std::time::Duration;

pub fn read_pic(port_name: &String, baud_rate: u32, timeout: u64, verbose: bool) {
    println!(
        "[CLI] will receive from Arduino at {0} with baud rate {1}...",
        port_name, baud_rate
    );

    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(timeout))
        .open();

    match port {
        Ok(mut port) => {
            println!(
                "[CLI] created connection data on {} at {} baud:",
                &port_name, &baud_rate
            );
            read(&mut port, verbose);
        }
        Err(e) => {
            eprintln!("[CLI] Failed to open \"{}\". Error: {}", port_name, e);
            std::process::exit(1);
        }
    }
}

fn read(port: &mut Box<dyn SerialPort>, verbose: bool) {
    let serial_reader = SerialReader::new(vec![0; 1000], verbose);
    let serial_writer = SerialWriter::new(verbose);
    let mut serial_programmer = SerialProgrammer::new(serial_reader, serial_writer, verbose);

    serial_programmer.read(port);
}
