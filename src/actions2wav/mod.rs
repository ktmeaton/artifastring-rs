use crate::cli;
use crate::Action;
use color_eyre::eyre::{Report, Result, WrapErr};
use itertools::Itertools;
use log::{debug};
use std::str::FromStr;

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
    input
        .split('\n')
        .map(String::from)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| l.split("\t").map(String::from).collect_vec()) // Split on tab
        // testing
        .map(|l| {
            Action::from_str(&l[0])
            // match l[0].as_str() {
            //     "a" => Some(Action::BowAccelerate),
            //     "b" => Some(Action::Bow),
            //     "f" => Some(Action::Finger),
            //     "p" => Some(Action::Pluck),
            //     _ => None,
            // }
        })
        .inspect(|l| println!("{l:?}") )
        .collect_vec();

    Ok(())
}
