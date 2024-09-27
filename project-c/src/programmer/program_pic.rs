use crate::programmer::file_reader::get_lines;
use serialport::SerialPort;
use std::fs::File;
use std::io;
use std::io::BufReader;

use crate::programmer::serial_programmer::SerialProgrammer;
use crate::programmer::serial_reader::SerialReader;
use crate::programmer::serial_writer::SerialWriter;
use std::time::Duration;

pub fn program_pic(input_file_path: &String, port_name: &String, baud_rate: &u32, timeout: &u64) {
    println!("[CLI] sending file '{0}' to Arduino connected at {1} with baud rate {2}...", input_file_path, port_name, baud_rate);
    let baud_rate = *baud_rate;
    let timeout = *timeout;

    let port = serialport::new(port_name, baud_rate).timeout(Duration::from_millis(timeout))
        .open();

    match port {
        Ok(mut port) => {
            println!("[CLI] created connection data on {} at {} baud:", &port_name, &baud_rate);
            let lines = get_lines(input_file_path);
            program(lines, &mut port);
        }
        Err(e) => {
            eprintln!("[CLI] Failed to open \"{}\". Error: {}", port_name, e);
            std::process::exit(1);
        }
    }
}

fn program(lines: io::Lines<BufReader<File>>, port: &mut Box<dyn SerialPort>) {
    let serial_reader = SerialReader::new(vec![0; 1000]);
    let serial_writer = SerialWriter::new();
    let mut serial_programmer = SerialProgrammer::new(serial_reader, serial_writer);

    serial_programmer.program(port, lines);
}
