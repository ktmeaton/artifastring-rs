use std::path::PathBuf;

pub struct MonoWav {
    pub file_path: PathBuf,
    pub byte_size: u32,
    pub sample_rate: u32,
    pub total_samples: u32,
    pub haptic_downsample_factor: u32,
}
