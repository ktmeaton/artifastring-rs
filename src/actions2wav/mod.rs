use crate::cli;
use crate::{Action};
use color_eyre::eyre::{Report, Result, WrapErr};
use itertools::Itertools;
use log::{debug};

/// Run actions2wav
pub fn run(args: &cli::actions2wav::Args) -> Result<(), Report> {
    debug!("actions2wav | {args:?}");

    // Read input into string
    let mut input = std::fs::read_to_string(&args.input)
        .wrap_err(format!("Failed to read file: {}", args.input.display()))?;
    if input.ends_with('\n') || input.ends_with('\r') {
        input.pop();
    }

    // Convert to vector of tab separate elements
    let actions = input
        .split('\n')
        .map(String::from)
        .filter(|l| !l.is_empty() && !l.starts_with('#')) // Ignore comment lines
        .map(|l| l.split("\t").map(String::from).collect_vec()) // Split on tab
        .map(|l|  Action::from_array(&l)) // Parse to Action
        .collect::<Result<Vec<_>, _>>()?; // Collect and handle errors

    actions.into_iter().for_each(|a| println!("{a:?}"));


    Ok(())
}
