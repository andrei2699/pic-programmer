use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CommandLineOptions {
    #[arg(short, long)]
    pub input_file_path: String,
}
