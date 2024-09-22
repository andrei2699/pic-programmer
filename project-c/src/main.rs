mod command_line_args;

use clap::Parser;
use serialport;
use std::io::{self, Write};
use std::str;
use std::time::Duration;

fn main() {
    let args = command_line_args::CommandLineOptions::parse();

    let port_name = "COM3";
    let baud_rate = 57600;

    let port = serialport::new(port_name, baud_rate).timeout(Duration::from_millis(1000))
        .open();

    match port {
        Ok(mut port) => {
            let mut serial_buf: Vec<u8> = vec![0; 1000];
            let mut received_data = String::new();
            let mut should_break = false;
            println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);

            loop {
                match port.read(serial_buf.as_mut_slice()) {
                    Ok(bytes_read) => {
                        let content = &serial_buf[..bytes_read];
                        io::stdout().write_all(content).unwrap();
                        io::stdout().flush().unwrap();

                        if should_break {
                            println!("done!");
                            break;
                        }

                        if let Ok(chunk) = str::from_utf8(content) {
                            received_data.push_str(chunk);

                            if received_data.contains("Programmer ready!") {
                                println!("Message received: {}", received_data);

                                should_break = true;
                                let string = "abc";
                                match port.write(string.as_bytes()) {
                                    Ok(_) => {
                                        print!("{}", &string);
                                        std::io::stdout().flush().unwrap();
                                    }
                                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                                    Err(e) => eprintln!("{:?}", e),
                                }
                            }
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }


    // println!("sending file '{0}' to Arduino...", args.input_file_path);

    // list_ports();

    println!("done")
}

fn list_ports() {
    match serialport::available_ports() {
        Ok(ports) => {
            if ports.is_empty() {
                println!("no ports detected");
                return;
            }

            for p in ports {
                println!("{}", p.port_name);
            }
        }
        Err(e) => eprintln!("Error listing ports: {:?}", e),
    }
}