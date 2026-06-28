use std::f32::consts::PI;
use ndarray::{arr1, Array1, s};
use log::warn;

use crate::{
    ActionType,
    constants::*,
    constants::strings::strings_violin::VIOLIN_PARAMS, 
    InstrumentType,
    InstrumentNumber,
};

#[allow(non_snake_case)]
pub struct ArtifastringString {
    pub N: usize,
    pub n: Array1<f32>,
    pub fs_multiplier: u32,
    pub pc: StringPhysical,
    pub sc: StringConstants,
    pub vc: ViolinistCoefficients,
    pub ss: StringState,
    pub va: ViolinistActions,
    pub fs: f32,
    pub dt: f32,
    pub inside_phi: Array1<f32>,
}


impl ArtifastringString {
    pub fn new(
        instrument_type: InstrumentType,
        instrument_number: InstrumentNumber,
        instrument_sample_rate: u32,
        string_number: u32
    ) -> Self {

        // protected attributes
        let pc = match instrument_type {
            InstrumentType::Violin => VIOLIN_PARAMS[instrument_type.index()][instrument_number.index()].clone(),
            _ => todo!(),
        };
        #[allow(non_snake_case)]
        let N = pc.N as usize;
        // Resize delays

        let _tick_output_force: f32;
        let _num_friction_skip_over_stick: u32;
        let _debug_ticks: u32;
        let _debug_string_num = string_number;
    
        let sc = StringConstants::default();
        let vc = ViolinistCoefficients::default();
        let ss = StringState::default();
        let va = ViolinistActions::default();

        // let inv_a: Eigen::Matrix3f;
        // let inv_A_r: Eigen::Matrix2f;

        let _plucks: u32;
        let fs_multiplier = FS_MULTIPLICATION_FACTOR[instrument_type.index()][string_number as usize];
        let fs = (fs_multiplier * instrument_sample_rate) as f32;
        let dt = 1.0 / fs;

        let _audio_samples: [f32; (NORMAL_BUFFER_SIZE*3) as usize];
        let _force_samples: [f32; (NORMAL_BUFFER_SIZE*3) as usize];

        // to handle the memory alignment once
        let _ah = arr1(&vec![0.0; N]);
        // AA adh;
        // AA fn;
        //let n = arr1(&vec![0.0; N]);
        let inside_phi = arr1(&vec![0.0; N]);
        let n = arr1(&(1..=64).map(|i| i as f32).collect::<Vec<f32>>());

        let mut string = Self{ N, n, fs_multiplier, pc, sc, vc, ss, va, fs, dt, inside_phi};
        string.set_physical_constants();

        // srand( time(NULL) );
        string.reset();

        return string;
    }

    #[allow(non_snake_case)]
    pub fn set_N(&mut self){
        let N = self.N;
        self.sc.X1 = arr1(&vec![0.0; N]);
        self.sc.X2 = arr1(&vec![0.0; N]);
        self.sc.X3 = arr1(&vec![0.0; N]);
        self.sc.Y1 = arr1(&vec![0.0; N]);
        self.sc.Y2 = arr1(&vec![0.0; N]);
        self.sc.Y3 = arr1(&vec![0.0; N]);
        self.sc.G = arr1(&vec![0.0; N]);

        self.vc.phix0 = arr1(&vec![0.0; N]);
        self.vc.phix1 = arr1(&vec![0.0; N]);
        self.vc.phix2 = arr1(&vec![0.0; N]);

        self.ss.a = arr1(&vec![0.0; N]);
        self.ss.ad = arr1(&vec![0.0; N]);
        // ah.resize(N);
        // adh.resize(N);
        // fn.resize(N);

        // n.resize(N);
        // inside_phi.resize(N);
        // n = AA::LinSpaced(Eigen::Sequential, N, 1, N);

    }

    pub fn set_physical_constants(&mut self) {
        self.set_N();
        self.cache_pc_c();
        self.vc.recache = true;    
    }

    pub fn cache_pc_c(&mut self) {

        self.sc.div_pc_L = 1.0 / self.pc.L;
        self.sc.sqrt_two_div_L = ( 2.0 / self.pc.L).sqrt();
        #[allow(non_snake_case)]
        let I: f32 = PI * self.pc.d * self.pc.d * self.pc.d * self.pc.d / 64.0;
        #[allow(non_snake_case)]
        let N = self.N;
        let ones =arr1(&vec![1.0; N]);
        // Restrict delays to number of modes
        let rn = arr1(&self.pc.rn[..N]);
        // Get shorter references to simplify formulas before
        let n = &self.n;
        let pc = &self.pc;
        let sc = &self.sc;
        let fs = &self.fs;
        let dt = self.dt;


        let w0 = (
            (pc.T/pc.pl) * (n*PI*sc.div_pc_L).pow2()
            + (pc.E*I/pc.pl) * (n*PI*sc.div_pc_L).pow2().pow2()
        ).sqrt();

        let highest_freq = w0[N-1] / (2.0*PI);
        if highest_freq > fs/2.0 {
            warn!("BAD FREQ!  highest freq: {highest_freq}");
            warn!("           Nyquist freq: {}", fs/2.0);
        }

        let w = (w0.pow2() - rn.pow2()).sqrt();     

        self.sc.X1 = ((&w*dt).cos() + (&rn/&w)*((&w*dt).sin())) * ((-&rn*dt).exp());
        self.sc.X2 = ((&ones / &w) * (&w*dt).sin()) * ((-&rn*dt).exp());
        self.sc.X3 = (&ones - &self.sc.X1) / (pc.pl * &w0.pow2());

        self.sc.Y1 = -(&w + &rn.pow2()/&w) * (&w*dt).sin() * (-&rn*dt).exp();
        self.sc.Y2 = ((&w*dt).cos() - (&rn/&w)*((&w*dt).sin())) * (-&rn*dt).exp();
        self.sc.Y3 = -&self.sc.Y1 / (pc.pl * &w0.pow2());

        self.sc.G = self.sc.sqrt_two_div_L as f32 * (pc.T * (n*PI*self.sc.div_pc_L) + pc.E*I*(n*PI*self.sc.div_pc_L).powi(3));

        // println!("sc.G: {:?}", self.sc.G.slice(s![..10]));

        self.inside_phi = n*PI*self.sc.div_pc_L;
        self.vc.recache = true;
    }

    pub fn reset(&mut self) {
        // init everything, just to be safe
        self.cache_pc_c();

        // let plucks = 0;

        self.vc = ViolinistCoefficients::default();
        self.vc.recache = true;
        
        self.va = ViolinistActions::default();
        self.va.Kf = K_FINGER;
        self.ss = StringState::default();

        // let debug_ticks = 0;
    }
}

#[derive(Clone)]
#[allow(non_snake_case)]
pub struct StringPhysical {
     // Tension (N)
    pub T: f32,
    // Length (m)
    pub L: f32,
    // Diameter (m)
    pub d: f32,
    // Linear Density (kg/m)
    pub pl: f32,
    // Young's elastic modulus
    pub E: f32,
    // Coefficient of static friction
    pub mu_s: f32,
    // Coefficient of dynamic friction
    pub mu_d: f32,
    // Slope of hyperbolic friction curve
    pub v0: f32,
    // Minimum sum-of-squares amplitude to maintain processing
    pub cutoff: f32,
    // Number of modes for this string (should be a multiple of 4 for SSE, or 8 for AVX)
    pub N: u32,
    // Array of modal decays
    pub rn: [f32; 128]
}

#[derive(Default)]
#[allow(dead_code, non_snake_case)]
pub struct StringConstants {
    X1: Array1<f32>, // displacement
    X2: Array1<f32>,
    X3: Array1<f32>,
    Y1: Array1<f32>, // velocity
    Y2: Array1<f32>,
    Y3: Array1<f32>,
    G:  Array1<f32>, // bridge
    div_pc_L: f32,
    sqrt_two_div_L: f32,
}

#[derive(Default)]
#[allow(dead_code, non_snake_case)]
pub struct ViolinistCoefficients {
    x0: f32,
    x1: f32,
    x2: f32,
    phix0: Array1<f32>, // position eigenvalues
    phix1: Array1<f32>,
    phix2: Array1<f32>,

    //float D1old, D2old, D3old, D4old; // pluck and release
    //float D1, D2, D3, D4; // bow
    //float D5, D6, D7; // finger during bowing

    //float D8, D9, D10, D11; // pluck release

    // extra "actions"
    K0: f32,
    K1: f32,
    K2: f32,
    R0: f32,
    R1: f32,
    R2: f32,
    y_pluck: f32, // for pluck displacement
    y_pluck_target: f32,
    pluck_samples_remaining: u32,
    recache: bool,
}

#[derive(Default)]
#[allow(dead_code)]
pub struct StringState {
    a: Array1<f32>,
    ad: Array1<f32>,
    slipstate: u32,
    actions: ActionType,
}

#[derive(Default)]
#[allow(dead_code, non_snake_case)]
pub struct ViolinistActions {
    bow_pluck_position: f32,  // bow/pluck position
    finger_position: f32,  // finger position
    Fb: f32,  // bow force
    vb: f32,  // bow velocity
    // iffy actions
    va: f32,  // bow acceleration, per dt (unit of time)
    vb_target: f32, // target bow velocity, used in acceleration
    Kf: f32,
}
