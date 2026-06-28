use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Write;
use color_eyre::eyre::{Report, Result};

pub struct MonoWav {
    pub file_path: PathBuf,
    pub buffer_size: u32,
    pub sample_rate: u32,
    pub total_samples: u32,
    pub haptic_downsample_factor: u32,
    pub data: Vec<u32>,
    pub index: u32,

    // int size;
    // int index;
    // bool is_int;
    // short *data_s;
    // int *data_i;

    // int total_samples;
    // FILE *outfile;    
}

impl MonoWav {

    pub fn new (
        file_path: PathBuf,
        buffer_size: u32,
        sample_rate: u32
    // set_is_int: bool
    ) -> Result<Self, Report> {
        let total_samples = 0;
        let haptic_downsample_factor = 1;
        let data = vec![0; buffer_size as usize];
        let index = 0;
        let outfile = prep_wav_file(&file_path, sample_rate)?;

        Ok(Self {
            file_path,
            buffer_size,
            sample_rate,
            total_samples,
            haptic_downsample_factor,
            data,
            index
        })

    }


    pub fn request_fill(&self, _delta: u32){
        // if (num_samples >= size) {
        //     increase_size(2*num_samples);
        // }
        // if ((index + num_samples) >= size) {
        //     writeBuffer();
        // }
        // // yay, pointer math!
        // short *start_fill = data_s+index;
        // index += num_samples;
        // return start_fill;
    }
}

// from marsyas WavSink.h
#[derive(Default)]
#[allow(dead_code)]
pub struct WavHdr {
    riff: String,  // "RIFF"
    file_size: u32, // in bytes
    wave: String, // "WAVE"
    fmt: String, // "fmt "
    chunk_size: u32, // in bytes (16 for PCM)
    format_tag: u16, // 1=PCM, 2=ADPCM, 3=IEEE float, 6=A-Law, 7=Mu-Law
    num_chans: u16, // 1=mono, 2=stereo
    sample_rate: u32,
    bytes_per_sec: u32,
    bytes_per_samp: u32, // 2=16-bit mono, 4=16-bit stereo
    bits_per_samp: u32,
    data: String, // "data"
    data_length: u32, // in bytes
}

pub fn prep_wav_file(file_path: &Path, sample_rate: u32) -> Result<(), Report>{
    let mut hdr_ = WavHdr::default();
    let mut file = File::create(file_path)?;
    // FILE* sfp_ = fopen(filename, "wb");

    hdr_.riff = "RIFF".to_string();
    hdr_.file_size = 44;
    hdr_.wave = "WAVE".to_string();
    hdr_.fmt = "fmt ".to_string();
    hdr_.format_tag = 1;
    hdr_.chunk_size = 16;
    hdr_.num_chans = 1;
    hdr_.sample_rate = sample_rate;
    hdr_.data_length = 0;
    hdr_.bytes_per_samp = 4;
    hdr_.bytes_per_sec = hdr_.sample_rate * 4;
    hdr_.bits_per_samp = 32;
    hdr_.data = "data".to_string();
    // fwrite(&hdr_, 4, 11, sfp_);
    // return sfp_;
    Ok(())
}