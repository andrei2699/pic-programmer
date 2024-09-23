use crate::programmer::serial_reader::ReadSerial;
use crate::programmer::serial_writer::WriteSerial;
use serialport::SerialPort;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::str;

const READY_MESSAGE: &'static str = "Programmer ready!";
const END_OF_FILE: &'static str = ":00000001FF";
const OK_INSTRUCTION: &'static str = "Y";
const RESEND_INSTRUCTION: &'static str = "R";

pub struct SerialProgrammer<R: ReadSerial, W: WriteSerial> {
    reader: R,
    writer: W,
}

impl<R: ReadSerial, W: WriteSerial> SerialProgrammer<R, W> {
    pub fn new(reader: R, writer: W) -> SerialProgrammer<R, W> {
        SerialProgrammer { reader, writer }
    }

    pub fn program(&mut self, port: &mut Box<dyn SerialPort>, lines: Lines<BufReader<File>>) {
        self.wait_for_programmer_to_be_ready(port);

        self.send_lines(port, lines);

        println!("finished programming!")
    }

    fn wait_for_programmer_to_be_ready(&mut self, port: &mut Box<dyn SerialPort>) {
        let mut received_data = String::new();
        loop {
            self.reader.read(port, &mut received_data);

            if received_data.contains(READY_MESSAGE) {
                println!("Programmer start message: {}", received_data);
                break;
            }
        }
    }

    fn send_lines(&mut self, port: &mut Box<dyn SerialPort>, lines: Lines<BufReader<File>>) {
        let mut received_data = String::new();

        'lines: for line in lines {
            let string = line.unwrap();
            let trimmed_line = string.trim();
            if trimmed_line.is_empty() {
                continue;
            }

            received_data.clear();
            while !received_data.contains(OK_INSTRUCTION) {
                self.writer.write(port, trimmed_line);

                if trimmed_line.contains(END_OF_FILE) {
                    break 'lines;
                }

                self.reader.read(port, &mut received_data);
                if received_data.contains(RESEND_INSTRUCTION) {
                    println!("resending instruction {}", trimmed_line);
                    received_data.clear();
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::programmer::file_reader::get_lines;
    use crate::programmer::serial_programmer::{SerialProgrammer, END_OF_FILE, OK_INSTRUCTION, RESEND_INSTRUCTION};
    use crate::programmer::serial_reader::ReadSerial;
    use crate::programmer::serial_writer::WriteSerial;
    use crate::programmer::test_serial_port::TestSerialPort;
    use serialport::SerialPort;
    use std::env;
    use std::path::{Path, PathBuf};

    #[test]
    fn given_starting_message_in_multiple_chunks_and_empty_file_should_not_write_anything() {
        let reader = ReaderTest { data: vec![String::from("Programmer "), String::from("ready"), String::from("!")], index: 0 };
        let writer = WriterTest { data: vec![] };
        let mut port: Box<dyn SerialPort> = Box::new(TestSerialPort {});
        let lines = get_lines(&get_full_path("test-files/empty-file.hex").to_string_lossy().to_string());
        let mut programmer = SerialProgrammer::new(reader, writer);

        programmer.program(&mut port, lines);

        assert_eq!(programmer.writer.data.is_empty(), true);
    }

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

    #[test]
    fn given_file_with_multiple_blank_lines_should_not_write_anything() {
        let reader = ReaderTest { data: vec![String::from("Programmer ready!")], index: 0 };
        let writer = WriterTest { data: vec![] };
        let mut port: Box<dyn SerialPort> = Box::new(TestSerialPort {});
        let lines = get_lines(&get_full_path("test-files/empty-file.hex").to_string_lossy().to_string());
        let mut programmer = SerialProgrammer::new(reader, writer);

        programmer.program(&mut port, lines);

        assert_eq!(programmer.writer.data.is_empty(), true);
    }

    #[test]
    fn given_file_with_only_eof_should_write_one_instruction() {
        let reader = ReaderTest { data: vec![String::from("Programmer ready!")], index: 0 };
        let writer = WriterTest { data: vec![] };
        let mut port: Box<dyn SerialPort> = Box::new(TestSerialPort {});
        let lines = get_lines(&get_full_path("test-files/only-eof.hex").to_string_lossy().to_string());
        let mut programmer = SerialProgrammer::new(reader, writer);

        programmer.program(&mut port, lines);

        assert_eq!(programmer.writer.data[0], END_OF_FILE);
    }

    #[test]
    fn given_file_eof_at_the_beginning_should_write_one_instruction() {
        let reader = ReaderTest { data: vec![String::from("Programmer ready!")], index: 0 };
        let writer = WriterTest { data: vec![] };
        let mut port: Box<dyn SerialPort> = Box::new(TestSerialPort {});
        let lines = get_lines(&get_full_path("test-files/eof-at-the-beginning.hex").to_string_lossy().to_string());
        let mut programmer = SerialProgrammer::new(reader, writer);

        programmer.program(&mut port, lines);

        assert_eq!(programmer.writer.data[0], END_OF_FILE);
    }

    #[test]
    fn given_file_with_all_instructions_sent_correctly_should_write_all_instructions_once() {
        let reader = ReaderTest {
            data: vec![
                String::from("Programmer ready!"),
                String::from(OK_INSTRUCTION),
                String::from(OK_INSTRUCTION)
            ],
            index: 0,
        };
        let writer = WriterTest { data: vec![] };
        let mut port: Box<dyn SerialPort> = Box::new(TestSerialPort {});
        let lines = get_lines(&get_full_path("test-files/multiple-instructions.hex").to_string_lossy().to_string());
        let mut programmer = SerialProgrammer::new(reader, writer);

        programmer.program(&mut port, lines);

        assert_eq!(programmer.writer.data[0], ":02002400BE1C");
        assert_eq!(programmer.writer.data[1], ":02002500BE1B");
        assert_eq!(programmer.writer.data[2], END_OF_FILE);
    }

    #[test]
    fn given_file_with_one_instruction_sent_with_problems_should_retry_the_write_of_that_instruction() {
        let reader = ReaderTest {
            data: vec![
                String::from("Programmer ready!"),
                String::from(RESEND_INSTRUCTION),
                String::from(OK_INSTRUCTION),
                String::from(OK_INSTRUCTION)
            ],
            index: 0,
        };
        let writer = WriterTest { data: vec![] };
        let mut port: Box<dyn SerialPort> = Box::new(TestSerialPort {});
        let lines = get_lines(&get_full_path("test-files/multiple-instructions.hex").to_string_lossy().to_string());
        let mut programmer = SerialProgrammer::new(reader, writer);

        programmer.program(&mut port, lines);

        assert_eq!(programmer.writer.data[0], ":02002400BE1C");
        assert_eq!(programmer.writer.data[1], ":02002400BE1C");
        assert_eq!(programmer.writer.data[2], ":02002500BE1B");
        assert_eq!(programmer.writer.data[3], END_OF_FILE);
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
        fn write(&mut self, _port: &mut Box<dyn SerialPort>, string: &str) {
            self.data.push(string.to_string())
        }
    }

    fn get_full_path(relative_path: &str) -> PathBuf {
        let current_dir = env::current_dir().expect("Failed to get current directory");
        let full_path = current_dir.join(Path::new("src/programmer")).join(Path::new(relative_path));
        full_path
    }
}