use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to the `Bundles` directory
    #[arg(short, long, value_name = "PATH")]
    pub path: PathBuf,

    /// Path to the output csv file
    #[arg(short, long, value_name = "FILE")]
    pub file: PathBuf,
}
