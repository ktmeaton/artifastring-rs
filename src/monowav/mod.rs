//use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Write, BufWriter};
use color_eyre::eyre::{Report, Result};

use std::f32::consts::PI;
use std::i16;


pub struct MonoWav {
    pub num_channels: u16,  // 1 = Mono, 2 = Stereo
    pub bits_per_sample: u16,  // 16=16-bit mono, 32=16-bit stereo
    pub sample_rate: u32,  // ex. 44100
    pub data: Vec<u8>, // Little endian bytes 
    pub total_samples: u32,
}

impl MonoWav {

    pub fn new () -> Self {
        let sample_rate = 44100;
        let data = Vec::new();
        let num_channels: u16 = 1;      // Mono
        let bits_per_sample: u16 = 16;  // 16=16-bit mono, 32=16-bit stereo
        let total_samples = 0;

        let mut monowav = Self {
            num_channels,
            bits_per_sample,
            sample_rate,
            data,
            total_samples
        };

        monowav.write_data();
        monowav.write_file().unwrap();
        //prep_wav_file();
        return monowav

    }

    pub fn write_data(&mut self){
        for t in (0 .. 44100).map(|x| x as f32 / 44100.0) {
            let sample = (t * 440.0 * 2.0 * PI).sin();
            let amplitude = i16::MAX as f32;
            let data = ((sample * amplitude) as i16).to_le_bytes();
            self.data.extend_from_slice(&data);
        }
    }

    pub fn write_file(&self) -> Result<(), Report>{

        // Create empty wav file
        let file = File::create("data/manual.wav")?;
        let mut writer = BufWriter::new(file);

        // Calculate chunk sizes for header
        let subchunk2_size = self.data.len() as u32;
        let chunk_size = 36 + subchunk2_size;

        //  RIFF Chunk
        writer.write_all(b"RIFF")?;
        writer.write_all(&chunk_size.to_le_bytes())?;
        writer.write_all(b"WAVE")?;

        // "fmt " subchunk
        writer.write_all(b"fmt ")?;
        writer.write_all(&16u32.to_le_bytes())?; // Subchunk1Size (16 for PCM)
        writer.write_all(&1u16.to_le_bytes())?;  // AudioFormat (1 for uncompressed PCM)
        writer.write_all(&self.num_channels.to_le_bytes())?;
        writer.write_all(&self.sample_rate.to_le_bytes())?;

        // ByteRate
        let byte_rate = self.sample_rate * self.num_channels as u32 * (self.bits_per_sample as u32 / 8);
        writer.write_all(&byte_rate.to_le_bytes())?;
        
        // BlockAlign
        let block_align = self.num_channels * (self.bits_per_sample / 8);
        writer.write_all(&block_align.to_le_bytes())?;
        writer.write_all(&self.bits_per_sample.to_le_bytes())?;

        // "data" subchunk
        writer.write_all(b"data")?;
        writer.write_all(&subchunk2_size.to_le_bytes())?;
        writer.write_all(&self.data)?;

        writer.flush()?;

        Ok(())
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

// pub fn prep_wav_file() -> Result<(), Report>{
//     // let mut hdr_ = WavHdr::default();
//     // let mut file = File::create(file_path)?;
//     // // FILE* sfp_ = fopen(filename, "wb");

//     // hdr_.riff = "RIFF".to_string();
//     // hdr_.file_size = 44;
//     // hdr_.wave = "WAVE".to_string();
//     // hdr_.fmt = "fmt ".to_string();
//     // hdr_.format_tag = 1;
//     // hdr_.chunk_size = 16;
//     // hdr_.num_chans = 1;
//     // hdr_.sample_rate = sample_rate;
//     // hdr_.data_length = 0;
//     // hdr_.bytes_per_samp = 4;
//     // hdr_.bytes_per_sec = hdr_.sample_rate * 4;
//     // hdr_.bits_per_samp = 32;
//     // hdr_.data = "data".to_string();
//     // fwrite(&hdr_, 4, 11, sfp_);
//     // return sfp_;

//     // 1. Define Audio Specifications
//     let sample_rate: u32 = 44100;
//     let num_channels: u16 = 1; // Mono
//     let bits_per_sample: u16 = 16;
//     let duration_seconds: u32 = 1;
//     let frequency: f32 = 440.0;

//     // 2. Generate Raw Audio Samples (PCM 16-bit)
//     let total_samples = (sample_rate * duration_seconds) as usize;
//     let mut pcm_data = Vec::with_capacity(total_samples * 2);

//     for t in (0 .. 44100).map(|x| x as f32 / 44100.0) {
//         let sample = (t * 440.0 * 2.0 * PI).sin();
//         let amplitude = i16::MAX as f32;
//         let data = ((sample * amplitude) as i16).to_le_bytes();
//         pcm_data.extend_from_slice(&data);
//     }

//     // 3. Calculate Header Sizes
//     let subchunk2_size = pcm_data.len() as u32; // Data chunk size
//     let chunk_size = 36 + subchunk2_size;       // Overall file size minus 8 bytes

//     // 4. Create and Write to File
//     let file = File::create("data/manual.wav")?;
//     let mut writer = BufWriter::new(file);

//     // --- RIFF Chunk Descriptor ---
//     writer.write_all(b"RIFF")?;
//     writer.write_all(&chunk_size.to_le_bytes())?;
//     writer.write_all(b"WAVE")?;

//     // --- "fmt " Sub-chunk ---
//     writer.write_all(b"fmt ")?;
//     writer.write_all(&16u32.to_le_bytes())?; // Subchunk1Size (16 for PCM)
//     writer.write_all(&1u16.to_le_bytes())?;  // AudioFormat (1 for uncompressed PCM)
//     writer.write_all(&num_channels.to_le_bytes())?;
//     writer.write_all(&sample_rate.to_le_bytes())?;

//     // ByteRate = SampleRate * NumChannels * BitsPerSample / 8
//     let byte_rate = sample_rate * num_channels as u32 * (bits_per_sample as u32 / 8);
//     writer.write_all(&byte_rate.to_le_bytes())?;
    
//     // BlockAlign = NumChannels * BitsPerSample / 8
//     let block_align = num_channels * (bits_per_sample / 8);
//     writer.write_all(&block_align.to_le_bytes())?;
//     writer.write_all(&bits_per_sample.to_le_bytes())?;

//     // --- "data" Sub-chunk ---
//     writer.write_all(b"data")?;
//     writer.write_all(&subchunk2_size.to_le_bytes())?;
//     writer.write_all(&pcm_data)?;

//     writer.flush()?;
//     println!("WAV file created successfully.");

//     Ok(())
// }

// fn update_wav_sizes(file_path: &Path) -> Result<(), Error> {
//     let mut file = OpenOptions::new().read(true).write(true).open("data/manual.wav")?;

//     // Total file size - 8 bytes
//     let file_len = file.metadata()?.len();
//     let total_chunk_size = (file_len - 8) as u32;

//     // Total file size - 44 bytes (standard 44-byte header)
//     let data_chunk_size = (file_len - 44) as u32;

//     // 1. Seek and write the RIFF Chunk Size at byte 4 (4 bytes)
//     file.seek(SeekFrom::Start(4))?;
//     file.write_all(&total_chunk_size.to_le_bytes())?;

//     // 2. Seek and write the Data Size at byte 40 (4 bytes)
//     file.seek(SeekFrom::Start(40))?;
//     file.write_all(&data_chunk_size.to_le_bytes())?;

//     Ok(())
// }