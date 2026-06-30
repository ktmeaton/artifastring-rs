use crate::constants::*;
use rustfft::{FftPlanner, num_complex::Complex};
// use std::sync::Mutex;

#[derive(Clone, Default)]
#[allow(dead_code)]
pub struct ArtifastringConvolution {
    // fftwf_mutex: Mutex<f32>,
    fft_input: Vec<f32>,
    fft_output: Vec<f32>,
    summed_output: Vec<f32>,
    summed_read_index: u32,
    fs_multiply: u32,
    convolution_size: u32,
    interim_m: u32,

    // fftwf variables
    // void *plan_f_p;
    // void *plan_b_p;
    // void *kernel_interim;
    // void *data_interim;
}

impl ArtifastringConvolution {
    pub fn new(
        fs_multiply: u32,
        kernel: Vec<f32>,
        num_samples: u32
    ) -> Self {

        // **MUST** be a power of two!
        println!("NORMAL_BUFFER_SIZE: {NORMAL_BUFFER_SIZE}, fs_multiply: {fs_multiply}, num_samples: {num_samples}");
        let convolution_size = (NORMAL_BUFFER_SIZE as u32 *fs_multiply + num_samples).next_power_of_two();
        let interim_m = (convolution_size / 2) + 1;

        // Construct FFT Plans
        // let mut planner = FftPlanner::new();
        // //let fft = planner.plan_fft_forward(convolution_size as usize);
        // let fft = planner.plan_fft_forward(10);
        // let mut buffer = vec![Complex{ re: 0.5f32, im: 0.0f32 }; 20];
        // println!("before: {buffer:?}");
        // fft.process(&mut buffer);
        // println!("after: {buffer:?}");

        // fftwf_plan_dft_r2c_1d is a basic FFTW interface call for planning a 1-d real-to-complex FFT operation on a single, contiguous data sequence. 
        // fftwf_plan plan_f = fftwf_plan_dft_r2c_1d(
        //                         convolution_size,
        //                         fft_input,
        //                         (fftwf_complex*)data_interim,
        //                         FFTW_ESTIMATE);
        // fftwf_plan plan_b = fftwf_plan_dft_c2r_1d(
        //                         convolution_size,
        //                         (fftwf_complex*) data_interim,
        //                         fft_output,
        //                         FFTW_ESTIMATE);
        // plan_f_p = (fftwf_plan*) plan_f;
        // plan_b_p = (fftwf_plan*) plan_b;


        let convolution = ArtifastringConvolution {
            fs_multiply,
            interim_m,
            convolution_size,
            ..Default::default()
        };

        convolution.load_kernel_from_time_data(kernel, num_samples);
        convolution.reset();

        return convolution
    }

    pub fn clear_input_buffer(&self) {
        todo!()
    }

    pub fn get_input_buffer(&self) -> &Vec<f32> {
        return &self.fft_input;
    }

    pub fn reset(&self) { todo!() }


    pub fn load_kernel_from_time_data(&self, kernel: Vec<f32>, num_samples: u32) { 

        let mut kernel_fft_input = vec![Complex{ re: 0.0, im: 0.0}; self.convolution_size as usize];
        (0..num_samples).map(|i| i as usize).for_each(|i| kernel_fft_input[i] =  Complex { re: kernel[i as usize], im: 0.0 } );
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(self.convolution_size as usize);
        fft.process(&mut kernel_fft_input);
        //(0..10).for_each(|i| println!("\t{i}: {}", kernel_fft_input[i]));

        // fftwf_plan kernel_plan_f = fftwf_plan_dft_r2c_1d(
        //                             convolution_size,
        //                             kernel_fft_input,
        //                             (fftwf_complex*) kernel_interim,
        //                             FFTW_ESTIMATE);
        // fftwf_execute(kernel_plan_f);
        // fftwf_destroy_plan(kernel_plan_f);

        // fftwf_free(kernel_fft_input);

        // unlock mutex
    }

    pub fn process(&self, _num_samples: u32) -> Vec<f32> {
        // fftwf_execute((fftwf_plan)plan_f_p);

        // // pointwise multiplication
        // for (int i=0; i<interim_m; i++) {
        //     // these will be optimized out
        //     const float x = ((fftwf_complex*)(data_interim))[i][0];
        //     const float y = ((fftwf_complex*)(data_interim))[i][1];
        //     const float u = ((fftwf_complex*)(kernel_interim))[i][0];
        //     const float v = ((fftwf_complex*)(kernel_interim))[i][1];

        //     ((fftwf_complex*)(data_interim))[i][0] = (x*u - y*v);
        //     ((fftwf_complex*)(data_interim))[i][1] = (x*v + y*u);
        // }
        // /*
        // for (int i=cutoff_bin; i<interim_m; i++) {
        //     ((fftwf_complex*)(data_interim))[i] = 0;
        // }
        // */
        // //memset(data_interim[cutoff_bin], 0, sizeof(fftwf_complex)*(interim_m-cutoff_bin));
        // fftwf_execute((fftwf_plan)plan_b_p);

        // // get output
        // int summed_write_index = summed_read_index;
        // for (int i=0; i<convolution_size; i++) {
        //     // body_out is un-normalized, but we take care of that
        //     // with the gain (set in python)
        //     summed_output[summed_write_index] += fft_output[i];
        //     // update pointer
        //     summed_write_index++;
        //     summed_write_index &= convolution_size - 1;
        // }


        // for (int i=0; i<num_samples; i++) {
        //     output_buffer[i] = summed_output[summed_read_index];
        //     summed_output[summed_read_index] = 0;
        //     summed_read_index++;
        //     summed_read_index &= convolution_size - 1;
        // }

        todo!()
    }
}
// (int fs_multiply_get,
//         const float *kernel, const int num_samples)
// {