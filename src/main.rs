use artifastring::cli::{Cli, Command};
use clap::Parser;
use color_eyre::eyre::{Report, Result};
use env_logger::Builder;
use std::io::Write;

fn main() -> Result<(), Report> {
    // ------------------------------------------------------------------------
    // CLI Setup

    // Parse CLI Parameters
    let args = Cli::parse();

    // initialize color_eyre crate for colorized logs
    color_eyre::install()?;

    // Set logging/verbosity level
    Builder::new()
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .filter(None, args.verbosity.to_level_filter())
        .init();


    // check which CLI command we're running (dataset, run, plot)
    match args.command {
        // actions2wav
        Command::Actions2Wav(args) => artifastring::actions2wav::run(&args)?,
    }

    Ok(())
}
