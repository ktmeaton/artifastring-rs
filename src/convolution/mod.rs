use crate::constants::*;

use color_eyre::eyre::{Report, Result};
use realfft::{RealFftPlanner, RealToComplex, ComplexToReal};
// use std::sync::Mutex;
use std::sync::Arc;
use num_complex::Complex;

#[allow(dead_code)]
pub struct ArtifastringConvolution {
    // fftwf_mutex: Mutex<f32>,
    fs_multiply: u32,
    fft_input: Vec<f32>,
    fft_output: Vec<f32>,
    summed_output: Vec<f32>,
    summed_read_index: u32,
    convolution_size: usize,
    interim_m: usize,
    plan_f_p: Arc<dyn RealToComplex<f32>>,
    plan_b_p: Arc<dyn ComplexToReal<f32>>,
    kernel_interim: Vec<Complex<f32>>,

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
    ) -> Result<Self, Report> {

        // Other variables
        let summed_read_index = 0;

        // -- Construct the convolution size, this is needed for the FFT plan
        // -- **MUST** be a power of two!
        let convolution_size = (NORMAL_BUFFER_SIZE as u32 *fs_multiply + num_samples).next_power_of_two() as usize;
        // println!("Convolution | NORMAL_BUFFER_SIZE: {NORMAL_BUFFER_SIZE}, fs_multiply: {fs_multiply}, num_samples: {num_samples}, convolution_size: {convolution_size}");

        // -- Memory size for the interim kernel and data
        let interim_m = (convolution_size / 2) + 1;

        // -- Lock the fftwf mutex, increment the shared reference counter
        //    rust: probably not necessary, as rustfft uses an Arc

        // -- Initialize float * buffers for I/O using (float*)fftwf_malloc(sizeof(float) * convolution_size);
        //    fft_input, fft_output, summed_output
        let fft_input = vec![0.0; convolution_size];
        let fft_output = vec![0.0; convolution_size];
        let summed_output = vec![0.0; convolution_size];
    
        // -- Initialize buffers for interim kernel and data using fftwf_malloc(sizeof(fftwf_complex) * interim_m);
        //    data_interim, kernel_interim
        //    rust: probably don't need interim?
        let kernel_interim = vec![Complex{re: 0.0, im: 0.0}; interim_m];
    
        // -- Fill kernel_interim with 0

        // -- Create 2 FFT plans, plan_f and plan_b
        // -- plan_f = fftwf_plan_dft_r2c_1d | 1-dimensional real to complex with flag FFTW_ESTIMATE
        //             input: fft_input, output: (fftwf_complex*)data_interim,
        // -- plan_b = fftwf_plan_dft_c2r_1d | 1-dimensional complex to real with flag FFTW_ESTIMATE
        //             input: (fftwf_complex*) data_interim, output: fft_output
        // -- Save the plans (plan_f_p, plan_b_p) as type (fftwf_plan*)
        let mut planner = RealFftPlanner::<f32>::new();

        let plan_f_p = planner.plan_fft_forward(convolution_size);
        let plan_b_p = planner.plan_fft_inverse(convolution_size);

        let mut convolution = ArtifastringConvolution {
            fs_multiply,
            interim_m,
            convolution_size,
            plan_f_p,
            plan_b_p,
            fft_input,
            fft_output,
            summed_output,
            summed_read_index,
            kernel_interim,
        };

        // -- Unlock the mutex

        // -- Run load_kernel_from_time_data
        convolution.load_kernel_from_time_data(kernel, num_samples)?;

        // -- Run reset
        convolution.reset();

        Ok(convolution)
    }

    pub fn clear_input_buffer(&mut self) {
        self.fft_input = vec![0.0; self.convolution_size];
    }

    pub fn get_input_buffer(&self) -> &Vec<f32> {
        return &self.fft_input;
    }

    pub fn reset(&mut self) { 
        // Clear the input buffer
        self.clear_input_buffer();

        // -- Fill fft_output and summed_output buffers with 0
        self.fft_output = vec![0.0; self.convolution_size];
        self.summed_output = vec![0.0; self.convolution_size];

        // -- Set summed_read_index to 0
        self.summed_read_index = 0;
    }


    pub fn load_kernel_from_time_data(&mut self, kernel: Vec<f32>, num_samples: u32) -> Result<(), Report> { 

        // Initialize an input with 0s
        let mut kernel_fft_input = vec![0.0; self.convolution_size];

        // -- copy over data from kernel based on the number of samples
        // -- the haptic responses are already pre-sampled, I think.
        (0..num_samples).map(|i| i as usize).for_each(|i| kernel_fft_input[i] =  kernel[i] );

        // -- Create a temporary kernel plan (real2complex)
        let mut planner = RealFftPlanner::<f32>::new();
        let kernel_plan_f = planner.plan_fft_forward(self.convolution_size);
        kernel_plan_f.process(&mut kernel_fft_input, &mut self.kernel_interim)?;

        Ok(())
    }

    pub fn process(&self, _num_samples: u32) -> Vec<f32> {

        // -- Execute the plan_f plan, fftwf_execute((fftwf_plan)plan_f_p);

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

        // -- Execute the plan_b plan, fftwf_execute((fftwf_plan)plan_b_p);

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