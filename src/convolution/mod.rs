#[derive(Clone)]
pub struct ArtifastringConvolution {

}

impl ArtifastringConvolution {
    pub fn new(
        _fs_multiply: u32,
        _lowpass_time_data: Vec<f32>,
        _lowpass_num_taps: u32
    ) -> Self {
        Self{}
    }

    pub fn get_input_buffer(&self) -> Vec<f32> {
        // return fft_input;
        //  fft_input = (float*)fftwf_malloc(sizeof(float) * convolution_size);
        [5.0].to_vec()
    }
}
// (int fs_multiply_get,
//         const float *kernel, const int num_samples)
// {