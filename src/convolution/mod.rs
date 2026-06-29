#[derive(Clone)]
pub struct ArtifastringConvolution {
    fft_input: Vec<f32>,
}

impl ArtifastringConvolution {
    pub fn new(
        _fs_multiply: u32,
        _lowpass_time_data: Vec<f32>,
        _lowpass_num_taps: u32
    ) -> Self {
        let fft_input = Vec::new();
    
        Self {fft_input}
    }

    pub fn clear_input_buffer(&self) {
        todo!()
    }

    pub fn get_input_buffer(&self) -> &Vec<f32> {
        return &self.fft_input;
    }
}
// (int fs_multiply_get,
//         const float *kernel, const int num_samples)
// {