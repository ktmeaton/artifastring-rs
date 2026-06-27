use clap::Parser;
use std::path::PathBuf;
use crate::{ARTIFASTRING_INSTRUMENT_SAMPLE_RATE, HAPTIC_DOWNSAMPLE_FACTOR, InstrumentNumber, InstrumentType};

/// Convert an actions file to wav
#[derive(Clone, Debug, Parser)]
#[clap(verbatim_doc_comment)]
#[clap(arg_required_else_help = true)]
pub struct Args {
    /// Input actions file.
    #[clap(short = 'i', long, required = true)]
    pub input: PathBuf,

    /// Output wav file.
    #[clap(short = 'o', long, required = true)]
    pub output: PathBuf,

    /// Instrument type
    #[clap(short = 't', long)]
    #[clap(value_enum, default_value_t = InstrumentType::default())]
    #[clap(hide_possible_values = false)]
    pub instrument_type: InstrumentType,

    /// Instrument number
    #[clap(short = 'n', long)]
    #[clap(value_enum, default_value_t = InstrumentNumber::default())]
    #[clap(hide_possible_values = false)]
    pub instrument_number: InstrumentNumber,

    /// Sample rate
    #[clap(short = 's', long)]
    #[clap(default_value_t = ARTIFASTRING_INSTRUMENT_SAMPLE_RATE)]    
    pub sample_rate: u32,

    /// Haptic downsample factor
    #[clap(short = 'd', long)]
    #[clap(default_value_t = HAPTIC_DOWNSAMPLE_FACTOR)]    
    pub haptic_downsample_factor: u32,  
}

impl Default for Args {
    fn default() -> Self {
        Self::new()
    }
}

impl Args {
    pub fn new() -> Self {
        Args {
            input: PathBuf::new(),
            output: PathBuf::new(),
            instrument_type: InstrumentType::default(),
            instrument_number: InstrumentNumber::default(),
            sample_rate: ARTIFASTRING_INSTRUMENT_SAMPLE_RATE,
            haptic_downsample_factor: HAPTIC_DOWNSAMPLE_FACTOR,
        }
    }
}