use crate::programmer::serial_reader::ReadSerial;
use crate::programmer::serial_writer::WriteSerial;
use crate::programmer::states::ProgrammingStates;
use serialport::SerialPort;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::str;

const READY_MESSAGE: &'static str = "Programmer ready!";

pub struct SerialProgrammer<R: ReadSerial, W: WriteSerial> {
    reader: R,
    writer: W,
}

impl<R: ReadSerial, W: WriteSerial> SerialProgrammer<R, W> {
    pub fn new(reader: R, writer: W) -> SerialProgrammer<R, W> {
        SerialProgrammer { reader, writer }
    }

    pub fn program(&mut self, port: &mut Box<dyn SerialPort>, mut lines: Lines<BufReader<File>>) {
        let mut state: ProgrammingStates = ProgrammingStates::WaitToConnect;
        let mut received_data = String::new();

        loop {
            self.reader.read(port, &mut received_data);

            if received_data.is_empty() {
                continue;
            }

            if received_data.contains(READY_MESSAGE) {
                println!("Message received: {}", received_data);
                state = ProgrammingStates::SendData;
            }
            println!("Received {}", received_data);

            if state == ProgrammingStates::WaitToConnect
            {
                continue;
            }

            let option = lines.next();

            match option {
                None => {
                    break
                }
                Some(value) => {
                    let string = value.unwrap();
                    self.writer.write(port, &string);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::programmer::file_reader::get_lines;
    use crate::programmer::serial_programmer::SerialProgrammer;
    use crate::programmer::serial_reader::ReadSerial;
    use crate::programmer::serial_writer::WriteSerial;
    use crate::programmer::test_serial_port::TestSerialPort;
    use serialport::SerialPort;
    use std::env;
    use std::path::{Path, PathBuf};


    #[test]
    fn given_all_starting_message_and_empty_file_should_not_write_anything() {
        let reader = ReaderTest { data: vec![String::from("Programmer ready!")], index: 0 };
        let writer = WriterTest { data: vec![] };
        let mut port: Box<dyn SerialPort> = Box::new(TestSerialPort {});
        let lines = get_lines(&get_full_path("test-files/empty-file.hex").to_string_lossy().to_string());

        let mut programmer = SerialProgrammer::new(reader, writer);

        programmer.program(&mut port, lines);

        assert_eq!(programmer.writer.data.is_empty(), true);
    }

    struct ReaderTest {
        data: Vec<String>,
        index: usize,
    }
    impl ReadSerial for ReaderTest {
        fn read(&mut self, _port: &mut Box<dyn SerialPort>, received_data: &mut String) {
            let string = self.data[self.index].as_str();
            self.index += 1;
            received_data.push_str(string);
        }
    }

    struct WriterTest {
        data: Vec<String>,
    }
    impl WriteSerial for WriterTest {
        fn write(&mut self, _port: &mut Box<dyn SerialPort>, string: &String) {
            self.data.push(string.to_string())
        }
    }

    fn get_full_path(relative_path: &str) -> PathBuf {
        let current_dir = env::current_dir().expect("Failed to get current directory");
        let full_path = current_dir.join(Path::new("src/programmer")).join(Path::new(relative_path));
        full_path
    }
}