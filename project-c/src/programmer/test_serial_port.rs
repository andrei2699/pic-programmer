use serialport::{ClearBuffer, DataBits, FlowControl, Parity, SerialPort, StopBits};
use std::io;
use std::time::Duration;

pub struct TestSerialPort {}

unsafe impl Send for TestSerialPort {}

impl io::Read for TestSerialPort {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        panic!("test serial port")
    }
}

impl io::Write for TestSerialPort {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        panic!("test serial port")
    }

    fn flush(&mut self) -> io::Result<()> {
        panic!("test serial port")
    }
}

impl SerialPort for TestSerialPort {
    fn name(&self) -> Option<String> {
        panic!("test serial port")
    }

    fn baud_rate(&self) -> serialport::Result<u32> {
        panic!("test serial port")
    }

    fn data_bits(&self) -> serialport::Result<DataBits> {
        panic!("test serial port")
    }

    fn flow_control(&self) -> serialport::Result<FlowControl> {
        panic!("test serial port")
    }

    fn parity(&self) -> serialport::Result<Parity> {
        panic!("test serial port")
    }

    fn stop_bits(&self) -> serialport::Result<StopBits> {
        panic!("test serial port")
    }

    fn timeout(&self) -> Duration {
        panic!("test serial port")
    }

    fn set_baud_rate(&mut self, _baud_rate: u32) -> serialport::Result<()> {
        panic!("test serial port")
    }

    fn set_data_bits(&mut self, _data_bits: DataBits) -> serialport::Result<()> {
        panic!("test serial port")
    }

    fn set_flow_control(&mut self, _flow_control: FlowControl) -> serialport::Result<()> {
        panic!("test serial port")
    }

    fn set_parity(&mut self, _parity: Parity) -> serialport::Result<()> {
        panic!("test serial port")
    }

    fn set_stop_bits(&mut self, _stop_bits: StopBits) -> serialport::Result<()> {
        panic!("test serial port")
    }

    fn set_timeout(&mut self, _timeout: Duration) -> serialport::Result<()> {
        panic!("test serial port")
    }

    fn write_request_to_send(&mut self, _level: bool) -> serialport::Result<()> {
        panic!("test serial port")
    }

    fn write_data_terminal_ready(&mut self, _level: bool) -> serialport::Result<()> {
        panic!("test serial port")
    }

    fn read_clear_to_send(&mut self) -> serialport::Result<bool> {
        panic!("test serial port")
    }

    fn read_data_set_ready(&mut self) -> serialport::Result<bool> {
        panic!("test serial port")
    }

    fn read_ring_indicator(&mut self) -> serialport::Result<bool> {
        panic!("test serial port")
    }

    fn read_carrier_detect(&mut self) -> serialport::Result<bool> {
        panic!("test serial port")
    }

    fn bytes_to_read(&self) -> serialport::Result<u32> {
        panic!("test serial port")
    }

    fn bytes_to_write(&self) -> serialport::Result<u32> {
        panic!("test serial port")
    }

    fn clear(&self, _buffer_to_clear: ClearBuffer) -> serialport::Result<()> {
        panic!("test serial port")
    }

    fn try_clone(&self) -> serialport::Result<Box<dyn SerialPort>> {
        panic!("test serial port")
    }

    fn set_break(&self) -> serialport::Result<()> {
        panic!("test serial port")
    }

    fn clear_break(&self) -> serialport::Result<()> {
        panic!("test serial port")
    }
}