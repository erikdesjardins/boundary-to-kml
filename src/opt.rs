use clap::{ArgAction, Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Arguments {
    /// Logging verbosity (-v info, -vv debug, -vvv trace)
    #[clap(short = 'v', long = "verbose", action = ArgAction::Count, global = true)]
    pub verbose: u8,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Extract boundaries from a file
    Extract {
        #[command(flatten)]
        bounds: Bounds,

        /// Input PNG file
        input: PathBuf,
    },
}

#[derive(Args, Debug)]
pub struct Bounds {
    /// Bounding box min X coordinate, in EPSG:5321 - NAD83(CSRS) / Teranet Ontario Lambert
    pub x_min: f64,
    /// Bounding box min Y coordinate, in EPSG:5321 - NAD83(CSRS) / Teranet Ontario Lambert
    pub y_min: f64,
    /// Bounding box max X coordinate, in EPSG:5321 - NAD83(CSRS) / Teranet Ontario Lambert
    pub x_max: f64,
    /// Bounding box max Y coordinate, in EPSG:5321 - NAD83(CSRS) / Teranet Ontario Lambert
    pub y_max: f64,
}
