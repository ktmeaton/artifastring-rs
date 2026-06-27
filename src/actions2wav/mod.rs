use crate::cli;
use crate::{Action, ActionType, ArtifastringInstrument, MonoWav};
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

    // TBD: forces log file
    // TBD: string log files

    let artifastring_instrument = ArtifastringInstrument::new(args.instrument_type, args.instrument_number);
    let mut wav_file = MonoWav {
        file_path: args.output.clone(),
        byte_size: 4096,
        sample_rate: args.sample_rate,
        haptic_downsample_factor: args.haptic_downsample_factor,
        total_samples: 0
    };
    play_file(&actions, &artifastring_instrument, &mut wav_file);

    Ok(())
}

pub fn play_file(
    input: &[Action],
    instrument: &ArtifastringInstrument,
    wav_file: &mut MonoWav,
) {
    wav_file.total_samples = 0;

    input.iter().for_each(|command| {
        wait_until(instrument, wav_file, command);
        match command.action_type {
            ActionType::Release       => instrument.reset(),
            ActionType::Finger        => instrument.finger(command),
            ActionType::Bow           => instrument.bow(command),
            ActionType::BowAccelerate => instrument.bow_accel(command),
            ActionType::Pluck         => instrument.pluck(command),
            ActionType::Wait          => (),
        }
    });
    // delete violin
    // delete wavfile
}

pub fn command_bow(
    instrument: &ArtifastringInstrument,
    wav_file: &mut MonoWav,
    command: &Action,
) {
    wait_until(instrument, wav_file, command);
    instrument.bow(command);
}

pub fn command_bow_accel(
    instrument: &ArtifastringInstrument,
    wav_file: &mut MonoWav,
    command: &Action,
) {
    wait_until(instrument, wav_file, command);
    instrument.bow_accel(command);
}

pub fn command_finger(
    instrument: &ArtifastringInstrument,
    wav_file: &mut MonoWav,
    command: &Action,
) {
    wait_until(instrument, wav_file, command);
    instrument.finger(command);
}

pub fn command_pluck(
    instrument: &ArtifastringInstrument,
    wav_file: &mut MonoWav,
    command: &Action,
) {
    wait_until(instrument, wav_file, command);
    instrument.finger(command);
}

pub fn command_reset(
    instrument: &ArtifastringInstrument,
    wav_file: &mut MonoWav,
    command: &Action,
) {
    wait_until(instrument, wav_file,  command);
    instrument.reset();
}


pub fn command_wait(
    instrument: &ArtifastringInstrument,
    wav_file: &mut MonoWav,
    command: &Action,
) {
    wait_until(instrument, wav_file, command);
}



pub fn wait_until(
    instrument: &ArtifastringInstrument,
    wav_file: &mut MonoWav,
    command: &Action,
)
{
    println!("{command:?}");
    let mut delta: i32 = (command.seconds * (wav_file.sample_rate as f32) - (wav_file.total_samples as f32)) as i32;
    if delta >= 0 {
        let mut delta = delta as u32;
        delta = wav_file.haptic_downsample_factor * ( delta / wav_file.haptic_downsample_factor);
        wav_file.total_samples += delta;
    } else {
        println!("ERROR: going back in time!");
        // TBD: more logging
    }
    //println!("until: {}, delta: {delta}, total_samples: {}", command.seconds, wav_file.total_samples);
}

