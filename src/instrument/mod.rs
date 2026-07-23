use clap::{ValueEnum};
use color_eyre::eyre::{Report, Result};
use log::debug;

use crate::{Action, ArtifastringString, ArtifastringConvolution, StringNumber};
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
    pub string_audio_lowpass_convolution: Vec<ArtifastringConvolution>,
    pub string_force_lowpass_convolution: Vec<ArtifastringConvolution>,
    pub string_audio_lowpass_input: Vec<Vec<f32>>,
    pub string_force_lowpass_input: Vec<Vec<f32>>,
    pub body_audio_convolution: ArtifastringConvolution,
    pub body_force_convolution: ArtifastringConvolution,
    pub body_audio_input: Vec<f32>,
    pub bow_string: StringNumber,
    pub string_audio_output: Vec<Vec<f32>>,
}

impl ArtifastringInstrument {

    pub fn new(instrument_type: InstrumentType, instrument_number: InstrumentNumber, instrument_sample_rate: u32) -> Result<Self, Report> {

        // Create the strings
        let strings: Vec<ArtifastringString> = (0..NUM_VIOLIN_STRINGS).map(|st| {
            ArtifastringString::new(
                instrument_type,
                instrument_number,
                instrument_sample_rate,
                st,
            )
        }).collect();

        let string_audio_output = vec![vec![0.0 as f32; strings.len()]; strings.len()];
        let _string_force_output = string_audio_output.clone();

        // strings
        let mut string_audio_lowpass_convolution = vec![];
        let mut string_force_lowpass_convolution = vec![];
        let mut string_audio_lowpass_input = vec![];
        let mut string_force_lowpass_input = vec![];

        // body
        let body_audio_convolution;
        let body_force_convolution;
        //let body_audio_input;
        // let body_force_lowpass_input;
        // let body_force_input = vec![0.0; NORMAL_BUFFER_SIZE];

        // let body_in;
        // let body_out;
        // let f_hold; // output of body convolution
        // let f_hold_read_index: u32;

        // float *bow_string_forces;
        // float *bow_fft_output;
        // float *bow_convolution_output;
        // int bow_convolution_read_index;
        // int bow_string;

        // // fftwf_complex
        // void *kernel_interim;
        // void *body_interim;
        // void *forces_interim;
        // // fftwf_plan
        // void *body_plan_f_p;
        // void *body_plan_b_p;
        // void *forces_plan_f_p;
        // void *forces_plan_b_p;
        // int forces_M;
        // int body_M;

        // float m_bridge_force_amplify;
        // float m_bow_force_amplify;

        // map empircally determined response and its transformed sample rate
        // to the cached version at the new sample rate.
        // typedef std::pair<const float*, int> resampledTDCacheKey;
        // static std::map <resampledTDCacheKey, std::unique_ptr<float[]>> time_data_cache;
        // static std::mutex cache_mtx;


        let bow_string = StringNumber::One;

        // FFT stuff

        for (i, _) in strings.iter().enumerate() {
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

            // println!("string: {i}, lowpass_num_taps: {lowpass_num_taps}");
            // (0..10).for_each(|i| println!("\t{i}: {}", lowpass_time_data[i]));

            let audio_convolution = ArtifastringConvolution::new(fs_multiply, lowpass_time_data.clone(), lowpass_num_taps)?;
            let force_convolution = ArtifastringConvolution::new(fs_multiply, lowpass_time_data, lowpass_num_taps)?;

            let audio_buffer = audio_convolution.get_input_buffer().to_vec();
            let force_buffer = force_convolution.get_input_buffer().to_vec();

            string_audio_lowpass_convolution.push(audio_convolution);
            string_force_lowpass_convolution.push(force_convolution);

            string_audio_lowpass_input.push(audio_buffer.to_vec());
            string_force_lowpass_input.push(force_buffer.to_vec());
        }

        // body
        let body_time_data = match instrument_type {
            InstrumentType::Violin => BODY_VIOLIN_S[instrument_number.index()],
            InstrumentType::Viola  => BODY_VIOLA_S[instrument_number.index()],
            InstrumentType::Cello  => BODY_CELLO_S[instrument_number.index()],
        }.to_vec();
        let body_num_taps = body_time_data.len() as u32;
        // resample_time_data(body_time_data, body_num_taps, instrument_sample_rate);

        body_audio_convolution = ArtifastringConvolution::new(1, body_time_data.clone(), body_num_taps)?;
        body_force_convolution = ArtifastringConvolution::new(1, body_time_data, body_num_taps)?;

        let body_audio_input = body_audio_convolution.get_input_buffer().to_vec();
        // let body_force_input = body_force_convolution.get_input_buffer().to_vec();

        Ok(Self {
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
            body_audio_input,
            bow_string,
            string_audio_output,
        })
    }

    // Stops all movement
    pub fn reset(&mut self) {
        self.strings.iter_mut().for_each(|st| st.reset());
    }

    // Places finger on the string.
    pub fn finger(&mut self, command: &Action) {
        if let Some(sn) = &command.string_number {
            self.strings[sn.index()].finger(command);
        }
    }

    // Plucks a string.
    pub fn pluck(&mut self, command: &Action) {
        if let Some(sn) = &command.string_number {
            self.strings[sn.index()].pluck(command);
        }
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

    pub fn handle_buffer(&mut self, num_samples: u32) -> Result<Vec<f32>, Report> {

        // -- Clear input buffers for the lowpass convolution
        // -- FIXME: maybe not necessary?  especially force?
        self.string_audio_lowpass_convolution.iter_mut().for_each(|conv| { 
            conv.clear_input_buffer();
        });
        self.string_force_lowpass_convolution.iter_mut().for_each(|conv| {
            conv.clear_input_buffer();
        });

        // calculate string buffers (ie. playing samples)
        for (i, st) in self.strings.iter_mut().enumerate() {
            println!("handle_buffer num_samples: {num_samples}");
            self.string_audio_lowpass_input[i] = st.fill_buffer_forces(num_samples)?;
        }
        // the audio lowpass input will now have f32 audio samples for a sine wav

        // -- decimate string buffers
        for (i, string) in self.strings.iter_mut().enumerate() {
            let prep = self.string_audio_lowpass_convolution[i].process(
                string.fs_multiplier*num_samples
            );
            (0..num_samples).map(|n| n as usize).for_each(|n| {
                self.string_audio_output[i][n] = prep[string.fs_multiplier as usize *n]
            });
        }

        // -- FIXME
        // -- initialize output
        // write string audio output to body audio output
        self.body_audio_convolution.clear_input_buffer();
        for (i, _) in self.strings.iter().enumerate() {
            (0..num_samples).map(|n| n as usize).for_each(|n| {
                self.body_audio_input[i] += self.string_audio_output[i][n]
            });
        }
        let output = self.body_audio_convolution.process(
            num_samples
        )[0..num_samples as usize].to_vec();

        Ok(output)
    }

    /// process num_samples in chunks of buffer size
    ///
    /// process num_samples in chunks of buffer size
    pub fn wait_samples_forces(&mut self, num_samples: u32) -> Result<Vec<f32>, Report>  {
        // process num_samples in chunks of buffer size
        let buffer = Vec::new();
        let mut remaining = num_samples as usize;

        while remaining > NORMAL_BUFFER_SIZE {
            debug!("wait_samples | remaining: {remaining}, NORMAL_BUFFER_SIZE: {NORMAL_BUFFER_SIZE}");
            // let output = self.handle_buffer(num_samples)?;
            // handle_buffer(buffer+position, NULL, NORMAL_BUFFER_SIZE);
            remaining -= NORMAL_BUFFER_SIZE;
        }
        if remaining > 0 {
            debug!("wait_samples | remaining: {remaining}, NORMAL_BUFFER_SIZE: {NORMAL_BUFFER_SIZE}");
            // handle_buffer(buffer+position, NULL, remaining);
        }

        // self.handle_buffer(mono_wav, num_samples)?;
        Ok(buffer)
    }

    pub fn wait_samples_forces_python(&self) -> u32 { todo!(); }

}