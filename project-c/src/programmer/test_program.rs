use std::io;
use std::io::Write;
use std::str;
use std::time::Duration;

pub fn test_program(input_file_path: &String, port_name: &String, baud_rate: &u32, timeout: &u64) {
    println!("sending file '{0}' to Arduino connected at {1} with baud rate {2}...", input_file_path, port_name, baud_rate);

    let baud_rate = *baud_rate;
    let timeout = *timeout;

    let port = serialport::new(port_name, baud_rate).timeout(Duration::from_millis(timeout))
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
}