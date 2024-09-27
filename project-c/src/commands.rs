use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    ListPorts,
    Program {
        #[arg(
            short,
            long,
            help = "File path to hex file that needs to be programmed."
        )]
        input_file_path: String,

        #[arg(short, long, help = "Port name to use (e.g., COM3).")]
        port_name: String,

        #[arg(
            short,
            long,
            default_value_t = 57600,
            help = "Baud rate for the connection."
        )]
        baud_rate: u32,

        #[arg(
            short,
            long,
            default_value_t = 5000,
            help = "Serial port connection timeout in milliseconds."
        )]
        timeout: u64,

        #[arg(short, long, default_value_t = false, help = "Prints more content.")]
        verbose: bool,
    },
}
