use clap::{ValueEnum};
use color_eyre::eyre::{eyre, Report, Result};

use crate::{Action, ArtifastringString, ArtifastringConvolution};
use crate::{constants::*, constants::lowpass::*};

pub const ARTIFASTRING_INSTRUMENT_SAMPLE_RATE: u32 = 44100;
pub const HAPTIC_DOWNSAMPLE_FACTOR: u32 = 1;
pub const NUM_VIOLIN_STRINGS: u32 = 4;
#[derive(Clone, Copy, Debug, Default, ValueEnum)]
pub enum InstrumentType {
    #[default]
    Violin,
    Viola,
    Cello
}

impl InstrumentType {
    pub fn index(&self) -> usize {
        match &self {
            InstrumentType::Violin => 0,
            InstrumentType::Viola  => 1,
            InstrumentType::Cello  => 2,
        }
    }
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
    pub strings: Vec<ArtifastringString>,
    pub string_audio_lowpass_convolution: Vec<Option<ArtifastringConvolution>>,
    pub string_force_lowpass_convolution: Vec<Option<ArtifastringConvolution>>,
    pub string_audio_lowpass_input: Vec<Option<f32>>,
    pub string_force_lowpass_input: Vec<Option<f32>>,
    pub body_audio_convolution: Option<f32>,
    pub body_force_convolution: Option<f32>,
}

impl ArtifastringInstrument {

    pub fn new(instrument_type: InstrumentType, instrument_number: InstrumentNumber) -> Self {
        // Create the strings
        let strings: Vec<ArtifastringString> = (1..NUM_VIOLIN_STRINGS).map(|st| {
            // let fs_multiply = 1;
            // let fs_multiply = FS_MULTIPLICATION_FACTOR[m_instrument_type][st];
            ArtifastringString::new(
                instrument_type,
                instrument_number,
                st,
                //fs_multiply, instrument_sample_rate
            )
        }).collect();

        let bow_string = 0;

        // FFT stuff

        // Lowpass
        let string_audio_lowpass_convolution = vec![None; strings.len()];
        let string_force_lowpass_convolution = vec![None; strings.len()];
        let string_audio_lowpass_input = vec![None; strings.len()];
        let string_force_lowpass_input = vec![None; strings.len()];

        let body_audio_convolution = None;
        let body_force_convolution = None;

        strings.iter().enumerate().for_each(|(i, st)| {
            // let fs_multiply = 1;
            let fs_multiply = FS_MULTIPLICATION_FACTOR[instrument_type.index()][i];

            // lowpass setup
            let mut lowpass_time_data: &[f32];
            let mut lowpass_num_taps: i32;

            println!("TS");
            let lowpass_time_data = match fs_multiply {
                1 => LOWPASS_1.to_vec(),
                2 => LOWPASS_2.to_vec(),
                3 => LOWPASS_3.to_vec(),
                _ => LOWPASS_1.to_vec(),
                // 2 => &LOWPASS_2,
                // 3 => &LOWPASS_3,
                // 4 => &LOWPASS_4,
            };
            //     lowpass_time_data = LOWPASS_1,
            //     lowpass_num_taps = NUM_TAPS_LOWPASS_1;
            // } else if (fs_multiply == 2) {
            //     lowpass_time_data = LOWPASS_2,
            //     lowpass_num_taps = NUM_TAPS_LOWPASS_2;
            // } else if (fs_multiply == 3) {
            //     lowpass_time_data = LOWPASS_3,
            //     lowpass_num_taps = NUM_TAPS_LOWPASS_3;
            // } else if (fs_multiply == 4) {
            //     lowpass_time_data = LOWPASS_4,
            //     lowpass_num_taps = NUM_TAPS_LOWPASS_4;
            // }
        });

        Self {
            instrument_type,
            instrument_number,
            strings,
            string_audio_lowpass_convolution,
            string_force_lowpass_convolution,
            string_audio_lowpass_input,
            string_force_lowpass_input,
            body_audio_convolution,
            body_force_convolution,
        }
    }

    // Stops all movement
    pub fn reset(&self) {

    }

    // Places finger on the string.
    pub fn finger(&self, command: &Action) {
    }

    // Plucks a string.
    pub fn pluck(&self, command: &Action) {

    }

    // Sets the bow's action.
    pub fn bow(&self, command: &Action) {

    }

    // Sets the bow to accelerate to a target velocity.
    pub fn bow_accel(&self, command: &Action) {

    }

    // Advances time and writes data to a buffer.
    pub fn wait_samples(&self, command: &Action) -> u32 {
        todo!();
    }

    pub fn wait_samples_forces(&self) -> u32 { todo!(); }

    pub fn wait_samples_forces_python(&self) -> u32 { todo!(); }

}