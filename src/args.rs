use clap::{Parser, arg, command};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path format for saving files
    #[arg(short, long)]
    pub output_path: String,

    /// Template for rendering files
    #[arg(short, long)]
    pub file_template: PathBuf,

    /// The CSV file to process
    pub csv: PathBuf,
}
