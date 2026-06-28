use clap::{ValueEnum};

use crate::{Action, ArtifastringString, ArtifastringConvolution};
use crate::{constants::*, constants::lowpass::*, constants::body::*};

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
    Four,
    Five,
}

impl InstrumentNumber {
    pub fn index(&self) -> usize {
        match &self {
            InstrumentNumber::One => 0,
            InstrumentNumber::Two => 1,
            InstrumentNumber::Three => 2,
            InstrumentNumber::Four => 3,
            InstrumentNumber::Five => 4,
        }
    }
}

pub struct ArtifastringInstrument {
    pub instrument_type: InstrumentType,
    pub instrument_number: InstrumentNumber,
    pub instrument_sample_rate: u32,
    pub strings: Vec<ArtifastringString>,
    pub string_audio_lowpass_convolution: Vec<Option<ArtifastringConvolution>>,
    pub string_force_lowpass_convolution: Vec<Option<ArtifastringConvolution>>,
    pub string_audio_lowpass_input: Vec<Option<Vec<f32>>>,
    pub string_force_lowpass_input: Vec<Option<Vec<f32>>>,
    pub body_audio_convolution: Option<ArtifastringConvolution>,
    pub body_force_convolution: Option<ArtifastringConvolution>,
}

impl ArtifastringInstrument {

    pub fn new(instrument_type: InstrumentType, instrument_number: InstrumentNumber, instrument_sample_rate: u32) -> Self {
        // Create the strings
        let strings: Vec<ArtifastringString> = (1..NUM_VIOLIN_STRINGS).map(|st| {
            ArtifastringString::new(
                instrument_type,
                instrument_number,
                instrument_sample_rate,
                st,
            )
        }).collect();

        let _bow_string = 0;

        // FFT stuff

        // Lowpass
        let mut string_audio_lowpass_convolution = vec![None; strings.len()];
        let mut string_force_lowpass_convolution = vec![None; strings.len()];
        let mut string_audio_lowpass_input = vec![None; strings.len()];
        let mut string_force_lowpass_input = vec![None; strings.len()];
        //let mut body_audio_convolution;
        //let mut body_force_convolution;

        strings.iter().enumerate().for_each(|(i, _)| {
            // let fs_multiply = 1;
            let fs_multiply = FS_MULTIPLICATION_FACTOR[instrument_type.index()][i];

            // lowpass setup
            let lowpass_time_data = match fs_multiply {
                1 => LOWPASS_1.to_vec(),
                2 => LOWPASS_2.to_vec(),
                3 => LOWPASS_3.to_vec(),
                4 => LOWPASS_4.to_vec(),
                _ =>todo!(),
            };
            let lowpass_num_taps = lowpass_time_data.len() as u32;

            // If the requested sample rate is the default sample rate, we're done.
            // If not, the supplied convolution has to be scaled for the new sample
            // rate. resample_time_data takes care of that, with memoization
            // resample_time_data(lowpass_time_data, lowpass_num_taps, instrument_sample_rate);

            let convolution = ArtifastringConvolution::new(fs_multiply, lowpass_time_data, lowpass_num_taps);
            let input_buffer = convolution.get_input_buffer();

            string_audio_lowpass_convolution[i] = Some(convolution.clone());
            string_force_lowpass_convolution[i] = Some(convolution);

            string_audio_lowpass_input[i] = Some(input_buffer.clone());
            string_force_lowpass_input[i] = Some(input_buffer);
        });

        // body
        let body_time_data = match instrument_type {
            InstrumentType::Violin => BODY_VIOLIN_S[instrument_number.index()],
            InstrumentType::Viola  => BODY_VIOLA_S[instrument_number.index()],
            InstrumentType::Cello  => BODY_CELLO_S[instrument_number.index()],
        }.to_vec();
        let body_num_taps = body_time_data.len() as u32;
        // resample_time_data(body_time_data, body_num_taps, instrument_sample_rate);

        let body_convolution = ArtifastringConvolution::new(1, body_time_data, body_num_taps);
        let _body_audio_input = Some(body_convolution.get_input_buffer());
        let body_audio_convolution = Some(body_convolution.clone());
        let body_force_convolution  =Some(body_convolution);

        Self {
            instrument_type,
            instrument_number,
            instrument_sample_rate,
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
    pub fn reset(&mut self) {
        self.strings.iter_mut().for_each(|st| st.reset());
    }

    // Places finger on the string.
    pub fn finger(&self, _command: &Action) {
    }

    // Plucks a string.
    pub fn pluck(&self, _command: &Action) {

    }

    // Sets the bow's action.
    pub fn bow(&self, _command: &Action) {

    }

    // Sets the bow to accelerate to a target velocity.
    pub fn bow_accel(&self, _command: &Action) {

    }

    // Advances time and writes data to a buffer.
    pub fn wait_samples(&self, _command: &Action) -> u32 {
        todo!();
    }

    pub fn wait_samples_forces(&self) -> u32 { todo!(); }

    pub fn wait_samples_forces_python(&self) -> u32 { todo!(); }

}