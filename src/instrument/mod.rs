use clap::{ValueEnum};

use crate::{Action};

pub const ARTIFASTRING_INSTRUMENT_SAMPLE_RATE: u32 = 44100;
pub const HAPTIC_DOWNSAMPLE_FACTOR: u32 = 1;

#[derive(Clone, Copy, Debug, Default, ValueEnum)]
pub enum InstrumentType {
    #[default]
    Violin,
    Viola,
    Cello
}

#[derive(Clone, Copy, Debug, Default, ValueEnum)]
pub enum InstrumentNumber {
    #[default]
    One,
    Two,
    Three,
    Four
}

pub struct ArtifastringInstrument {
    pub instrument_type: InstrumentType,
    pub instrument_number: InstrumentNumber,
}

impl ArtifastringInstrument {

    // Stops all movement
    pub fn reset(&self) { todo!(); }

    // Places finger on the string.
    pub fn finger(&self, command: &Action) {
    }

    // Plucks a string.
    pub fn pluck(&self) { todo!(); }

    // Sets the bow's action.
    pub fn bow(&self) { todo!(); }

    // Sets the bow to accelerate to a target velocity.
    pub fn bow_accel(&self) { todo!(); }

    // Advances time and writes data to a buffer.
    pub fn wait_samples(&self) -> u32 { todo!(); }

    pub fn wait_samples_forces(&self) -> u32 { todo!(); }

    pub fn wait_samples_forces_python(&self) -> u32 { todo!(); }

}