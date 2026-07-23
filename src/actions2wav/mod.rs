use crate::cli;
use crate::{Action, ActionType, ArtifastringInstrument, MonoWav};
use crate::constants::*;

use color_eyre::eyre::{Report, Result, WrapErr};
use itertools::Itertools;
use log::{debug};

/// Run actions2wav
pub fn run(args: &cli::actions2wav::Args) -> Result<(), Report> {
    debug!("actions2wav | {args:?}");

    // Read file input into string
    let mut input = std::fs::read_to_string(&args.input)
        .wrap_err(format!("Failed to read file: {}", args.input.display()))?;
    if input.ends_with('\n') || input.ends_with('\r') {
        input.pop();
    }

    // Convert to vector of tab d elements
    let actions = input
        .split('\n')
        .map(String::from)
        .filter(|l| !l.is_empty() && !l.starts_with('#')) // Ignore comment lines
        .map(|l| l.split("\t").map(String::from).collect_vec()) // Split on tab
        .map(|l|  Action::from_array(&l)) // Parse to Action
        .collect::<Result<Vec<_>, _>>()?; // Collect and handle errors

    // TBD: forces log file
    // TBD: string log files

    let mut artifastring_instrument = ArtifastringInstrument::new(
        args.instrument_type,
        args.instrument_number,
        args.sample_rate
    )?;
    let _buffer = play_file(&actions, &mut artifastring_instrument)?;
    let mut mono_wav = MonoWav::new();
    // convert f32 buffer to little endian bytes
    // mono_wav.data = buffer.into_iter().map(|d| (d as i16).to_le_bytes()).collect();
    mono_wav.write_sine_wave(440.0, 1 * ARTIFASTRING_INSTRUMENT_SAMPLE_RATE);
    mono_wav.write_file(&args.output)?;
    Ok(())
}

pub fn play_file(
    input: &[Action],
    instrument: &mut ArtifastringInstrument,
) -> Result<Vec<f32>, Report> {

    let mut buffer = Vec::new();

    for command in input.iter() {
        let output = wait_until(instrument, command)?;
        buffer.extend(output);
        match command.action_type {
            ActionType::Release       => instrument.reset(),
            ActionType::Finger        => instrument.finger(command),
            ActionType::Bow           => instrument.bow(command),
            ActionType::BowAccelerate => instrument.bow_accel(command),
            ActionType::Pluck         => instrument.pluck(command),
            ActionType::Wait          => (),
            ActionType::Off           => todo!(),
        }
    }
    Ok(buffer)
}

pub fn wait_until (
    instrument: &mut ArtifastringInstrument,
    command: &Action,
) -> Result<Vec<f32>, Report>
{
    let delta = command.seconds * (ARTIFASTRING_INSTRUMENT_SAMPLE_RATE as f32);
    let buffer = instrument.wait_samples_forces(delta as u32)?;
    debug!("wait_until: {command:?}");
    Ok(buffer)
}

