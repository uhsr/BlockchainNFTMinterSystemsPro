// src/main.rs
/*
 * Main executable for BlockchainNFTMinterSystemsPro
 */

use clap::Parser;
use blockchainnftmintersystemspro::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockchainNFTMinterSystemsPro - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
