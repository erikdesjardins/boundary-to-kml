use anyhow::Error;
use geo::{Rect, coord};
use std::fs::File;
use std::io::BufReader;

mod extract;
mod opt;

#[cfg(test)]
mod tests;

fn main() -> Result<(), Error> {
    let opt::Arguments { verbose, command } = clap::Parser::parse();

    env_logger::Builder::new()
        .filter_level(match verbose {
            0 => log::LevelFilter::Warn,
            1 => log::LevelFilter::Info,
            2 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        })
        .init();

    match command {
        opt::Command::Extract { bounds, input } => {
            let mut input = BufReader::new(File::open(input)?);

            let bounds = Rect::new(
                coord! { x: bounds.x_min, y: bounds.y_min },
                coord! { x: bounds.x_max, y: bounds.y_max },
            );

            let points = extract::run(bounds, &mut input)?;

            // TODO: implement proper output
            dbg!(points);

            Ok(())
        }
    }
}
