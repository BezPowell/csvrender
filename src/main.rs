use clap::Parser;
use csvrender::{Args, process_csv};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    process_csv(args)
}
