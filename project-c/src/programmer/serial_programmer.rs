use crate::programmer::serial_reader::ReadSerial;
use crate::programmer::serial_writer::WriteSerial;
use serialport::SerialPort;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::str;

const READY_MESSAGE: &'static str = "Programmer ready!";
const PROGRAMMING_STARTED_MESSAGE: &'static str = "start";
const DONE_MESSAGE: &'static str = "done";
const END_OF_FILE: &'static str = ":00000001FF";
const OK_INSTRUCTION: u8 = b'Y';
const RESEND_INSTRUCTION: u8 = b'R';
const PROGRAM_INSTRUCTION: u8 = b'P';

pub struct SerialProgrammer<R: ReadSerial, W: WriteSerial> {
    reader: R,
    writer: W,
    verbose: bool,
}

impl<R: ReadSerial, W: WriteSerial> SerialProgrammer<R, W> {
    pub fn new(reader: R, writer: W, verbose: bool) -> SerialProgrammer<R, W> {
        SerialProgrammer {
            reader,
            writer,
            verbose,
        }
    }

    pub fn program(&mut self, port: &mut Box<dyn SerialPort>, lines: Lines<BufReader<File>>) {
        self.wait_for_programmer_message(port, READY_MESSAGE);

        self.send_lines(port, lines);

        println!("[CLI] finished programming!")
    }

    fn wait_for_programmer_message(&mut self, port: &mut Box<dyn SerialPort>, message: &str) {
        let mut received_data = String::new();
        println!("[CLI] waiting for programmer for '{}'....", message);
        loop {
            self.reader.read(port, &mut received_data);

            if received_data.contains(message) {
                println!("[Programmer]: '{}'", received_data);
                break;
            }
        }
    }

    fn send_lines(&mut self, port: &mut Box<dyn SerialPort>, lines: Lines<BufReader<File>>) {
        let mut received_data = String::new();
        let mut programming_message_sent = false;
        let ok_instruction_string = &OK_INSTRUCTION.to_string();
        let resend_instruction_string = &RESEND_INSTRUCTION.to_string();

        for line in lines {
            let string = line.unwrap();
            let trimmed_line = string.trim();
            if trimmed_line.is_empty() {
                continue;
            }

            if !programming_message_sent {
                println!("[CLI] programming started");
                self.writer.write(port, &PROGRAM_INSTRUCTION.to_be_bytes());
                self.wait_for_programmer_message(port, PROGRAMMING_STARTED_MESSAGE);

                programming_message_sent = true;
            }

            self.writer.write(port, trimmed_line.as_bytes());

            let mut instruction_sent_correctly = false;
            received_data.clear();
            while !instruction_sent_correctly {
                self.reader.read(port, &mut received_data);

                if self.verbose {
                    println!("[Programmer] received data: '{}'", received_data);
                }

                if received_data.contains(resend_instruction_string) {
                    println!("[CLI] resending instruction {}", trimmed_line);
                    self.writer.write(port, trimmed_line.as_bytes());
                    received_data.clear();
                } else if received_data.contains(ok_instruction_string) {
                    instruction_sent_correctly = true;
                }
            }

            if trimmed_line.contains(END_OF_FILE) {
                println!("[CLI] end of file reached {}", trimmed_line);
                break;
            }
        }

        if programming_message_sent {
            self.wait_for_programmer_message(port, DONE_MESSAGE);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::programmer::file_reader::get_lines;
    use crate::programmer::serial_programmer::{
        SerialProgrammer, DONE_MESSAGE, END_OF_FILE, OK_INSTRUCTION, PROGRAMMING_STARTED_MESSAGE,
        READY_MESSAGE, RESEND_INSTRUCTION,
    };
    use crate::programmer::serial_reader::ReadSerial;
    use crate::programmer::serial_writer::WriteSerial;
    use crate::programmer::test_serial_port::TestSerialPort;
    use serialport::SerialPort;
    use std::env;
    use std::path::{Path, PathBuf};

    #[test]
    fn given_starting_message_in_multiple_chunks_and_empty_file_should_not_write_anything() {
        let reader = ReaderTest {
            data: vec![
                String::from("Programmer "),
                String::from("ready"),
                String::from("!"),
            ],
            index: 0,
        };
        let writer = WriterTest { data: vec![] };
        let mut port: Box<dyn SerialPort> = Box::new(TestSerialPort {});
        let lines = get_lines(
            &get_full_path("test-files/empty-file.hex")
                .to_string_lossy()
                .to_string(),
        );
        let mut programmer = SerialProgrammer::new(reader, writer, true);

        programmer.program(&mut port, lines);

        assert_eq!(programmer.writer.data.is_empty(), true);
    }

    #[test]
    fn given_all_starting_message_and_empty_file_should_not_write_anything() {
        let reader = ReaderTest {
            data: vec![String::from("Programmer ready!")],
            index: 0,
        };
        let writer = WriterTest { data: vec![] };
        let mut port: Box<dyn SerialPort> = Box::new(TestSerialPort {});
        let lines = get_lines(
            &get_full_path("test-files/empty-file.hex")
                .to_string_lossy()
                .to_string(),
        );
        let mut programmer = SerialProgrammer::new(reader, writer, true);

        programmer.program(&mut port, lines);

        assert_eq!(programmer.writer.data.is_empty(), true);
    }

    #[test]
    fn given_file_with_multiple_blank_lines_should_not_write_anything() {
        let reader = ReaderTest {
            data: vec![String::from("Programmer ready!")],
            index: 0,
        };
        let writer = WriterTest { data: vec![] };
        let mut port: Box<dyn SerialPort> = Box::new(TestSerialPort {});
        let lines = get_lines(
            &get_full_path("test-files/empty-file.hex")
                .to_string_lossy()
                .to_string(),
        );
        let mut programmer = SerialProgrammer::new(reader, writer, true);

        programmer.program(&mut port, lines);

        assert_eq!(programmer.writer.data.is_empty(), true);
    }

    #[test]
    fn given_file_with_only_eof_should_write_one_instruction() {
        let reader = ReaderTest {
            data: vec![
                String::from(READY_MESSAGE),
                String::from(PROGRAMMING_STARTED_MESSAGE),
                OK_INSTRUCTION.to_string(),
                String::from(DONE_MESSAGE),
            ],
            index: 0,
        };
        let writer = WriterTest { data: vec![] };
        let mut port: Box<dyn SerialPort> = Box::new(TestSerialPort {});
        let lines = get_lines(
            &get_full_path("test-files/only-eof.hex")
                .to_string_lossy()
                .to_string(),
        );
        let mut programmer = SerialProgrammer::new(reader, writer, true);

        programmer.program(&mut port, lines);

        assert_eq!(programmer.writer.data[0], "P");
        assert_eq!(programmer.writer.data[1], END_OF_FILE);
    }

    #[test]
    fn given_file_eof_at_the_beginning_should_write_one_instruction() {
        let reader = ReaderTest {
            data: vec![
                String::from(READY_MESSAGE),
                String::from(PROGRAMMING_STARTED_MESSAGE),
                OK_INSTRUCTION.to_string(),
                String::from(DONE_MESSAGE),
            ],
            index: 0,
        };
        let writer = WriterTest { data: vec![] };
        let mut port: Box<dyn SerialPort> = Box::new(TestSerialPort {});
        let lines = get_lines(
            &get_full_path("test-files/eof-at-the-beginning.hex")
                .to_string_lossy()
                .to_string(),
        );
        let mut programmer = SerialProgrammer::new(reader, writer, true);

        programmer.program(&mut port, lines);

        assert_eq!(programmer.writer.data[0], "P");
        assert_eq!(programmer.writer.data[1], END_OF_FILE);
    }

    #[test]
    fn given_file_with_all_instructions_sent_correctly_should_write_all_instructions_once() {
        let reader = ReaderTest {
            data: vec![
                String::from("Programmer ready!"),
                String::from(PROGRAMMING_STARTED_MESSAGE),
                OK_INSTRUCTION.to_string(),
                OK_INSTRUCTION.to_string(),
                OK_INSTRUCTION.to_string(),
                String::from(DONE_MESSAGE),
            ],
            index: 0,
        };
        let writer = WriterTest { data: vec![] };
        let mut port: Box<dyn SerialPort> = Box::new(TestSerialPort {});
        let lines = get_lines(
            &get_full_path("test-files/multiple-instructions.hex")
                .to_string_lossy()
                .to_string(),
        );
        let mut programmer = SerialProgrammer::new(reader, writer, true);

        programmer.program(&mut port, lines);

        assert_eq!(programmer.writer.data[0], "P");
        assert_eq!(programmer.writer.data[1], ":02002400BE1C");
        assert_eq!(programmer.writer.data[2], ":02002500BE1B");
        assert_eq!(programmer.writer.data[3], END_OF_FILE);
    }

    #[test]
    fn given_file_with_one_instruction_sent_with_problems_should_retry_the_write_of_that_instruction(
    ) {
        let reader = ReaderTest {
            data: vec![
                String::from("Programmer ready!"),
                String::from(PROGRAMMING_STARTED_MESSAGE),
                RESEND_INSTRUCTION.to_string(),
                OK_INSTRUCTION.to_string(),
                OK_INSTRUCTION.to_string(),
                OK_INSTRUCTION.to_string(),
                String::from(DONE_MESSAGE),
            ],
            index: 0,
        };
        let writer = WriterTest { data: vec![] };
        let mut port: Box<dyn SerialPort> = Box::new(TestSerialPort {});
        let lines = get_lines(
            &get_full_path("test-files/multiple-instructions.hex")
                .to_string_lossy()
                .to_string(),
        );
        let mut programmer = SerialProgrammer::new(reader, writer, true);

        programmer.program(&mut port, lines);

        assert_eq!(programmer.writer.data[0], "P");
        assert_eq!(programmer.writer.data[1], ":02002400BE1C");
        assert_eq!(programmer.writer.data[2], ":02002400BE1C");
        assert_eq!(programmer.writer.data[3], ":02002500BE1B");
        assert_eq!(programmer.writer.data[4], END_OF_FILE);
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
        fn write(&mut self, _port: &mut Box<dyn SerialPort>, buffer: &[u8]) {
            self.data.push(String::from_utf8_lossy(buffer).to_string())
        }
    }

    fn get_full_path(relative_path: &str) -> PathBuf {
        let current_dir = env::current_dir().expect("Failed to get current directory");
        let full_path = current_dir
            .join(Path::new("src/programmer"))
            .join(Path::new(relative_path));
        full_path
    }
}
