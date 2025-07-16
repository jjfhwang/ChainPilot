// src/main.rs
/*
 * Main executable for ChainPilot
 */

use clap::Parser;
use chainpilot::{Result, run};

#[derive(Parser)]
#[command(version, about = "ChainPilot - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
