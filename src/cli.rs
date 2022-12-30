use clap::{command, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub in_file: std::path::PathBuf,
}

impl Args {
    pub fn get() -> Self {
        Self::parse()
    }
}
