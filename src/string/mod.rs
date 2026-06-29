use std::f32::consts::PI;
use ndarray::{arr1, Array1};
use log::warn;
use color_eyre::eyre::{eyre, Report, Result};
use std::str::FromStr;


use crate::{
    Action,
    ActionType,
    constants::*,
    constants::strings::strings_violin::VIOLIN_PARAMS, 
    InstrumentType,
    InstrumentNumber,
};

#[allow(non_snake_case)]
pub struct ArtifastringString {
    pub string_number: u32,
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

        let pc = match instrument_type {
            InstrumentType::Violin => VIOLIN_PARAMS[instrument_type.index()][instrument_number.index()].clone(),
            _ => todo!(),
        };
        #[allow(non_snake_case)]
        let N = pc.N as usize;

        let _tick_output_force: f32;
        let _num_friction_skip_over_stick: u32;
        let _debug_ticks: u32;
        let _debug_string_num = string_number;
    
        let sc = StringConstants::default();
        let vc = ViolinistCoefficients::default();
        let ss = StringState::default();
        let va = ViolinistActions::default();

        let _inv_a = arr1(&vec![vec![0.0; 3]; 3]);
        #[allow(non_snake_case)]
        let _inv_A_r = arr1(&vec![vec![0.0; 2]; 2]);

        let _plucks: u32;
        let fs_multiplier = FS_MULTIPLICATION_FACTOR[instrument_type.index()][string_number as usize];
        let fs = (fs_multiplier * instrument_sample_rate) as f32;
        let dt = 1.0 / fs;

        let _audio_samples: [f32; (NORMAL_BUFFER_SIZE*3) as usize];
        let _force_samples: [f32; (NORMAL_BUFFER_SIZE*3) as usize];

        // to handle the memory alignment once
        let _ah = arr1(&vec![0.0; N]);
        let _adh = arr1(&vec![0.0; N]);
        let _fn = arr1(&vec![0.0; N]);

        let inside_phi = arr1(&vec![0.0; N]);
        let n = arr1(&(1..=N).map(|i| i as f32).collect::<Vec<f32>>());

        let mut string = Self{ string_number, N, n, fs_multiplier, pc, sc, vc, ss, va, fs, dt, inside_phi};
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
        // self.ah = arr1(&vec![0.0; N]);
        // self.adh = arr1(&vec![0.0; N]);
        // self.fn = arr1(&vec![0.0; N]);

         self.inside_phi = arr1(&vec![0.0; N]);
         self.n = arr1(&(1..=N).map(|i| i as f32).collect::<Vec<f32>>());
    }

    pub fn set_physical_constants(&mut self) {
        self.set_N();
        self.cache_pc_c();  
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

    pub fn cache_pa_c(&mut self) {
        self.setup_vc_positions();

        self.vc.phix0 = self.sc.sqrt_two_div_L * ( self.vc.x0*&self.inside_phi ).sin();
        self.vc.phix1 = self.sc.sqrt_two_div_L * ( self.vc.x1*&self.inside_phi ).sin();

        #[allow(non_snake_case)]
        let _A00 = (&self.sc.X3 * &self.vc.phix0 * &self.vc.phix0).sum();
        #[allow(non_snake_case)]
        let A01 = (&self.sc.X3 * &self.vc.phix0 * &self.vc.phix1).sum();
        #[allow(non_snake_case)]
        let A11 = (&self.sc.X3 * &self.vc.phix1 * &self.vc.phix1).sum();
        #[allow(non_snake_case)]
        let B00 = (&self.sc.Y3 * &self.vc.phix0 * &self.vc.phix0).sum();
        #[allow(non_snake_case)]
        let B01 = (&self.sc.Y3 * &self.vc.phix0 * &self.vc.phix1).sum();
        #[allow(non_snake_case)]
        let B11 = (&self.sc.Y3 * &self.vc.phix1 * &self.vc.phix1).sum();

        match self.ss.actions {
            ActionType::Bow => {

                // bow coefficients
                #[allow(non_snake_case)]
                let L1 = 1.0 / ((B00*B11 - B01*B01)*self.vc.R1 + (A11*B00 - A01*B01)*self.vc.K1 + B00);
                self.vc.D1 = (B11*self.vc.R1 + A11*self.vc.K1 + 1.0)*L1;
                self.vc.D4 = 0.5 / self.vc.D1;
                self.vc.D2 = B01 * self.vc.K1 * L1;
                self.vc.D3 = B01 * self.vc.R1 * L1;

                // finger-during-bowing coefficients
                let L2 = -1.0 / (B11*self.vc.R1 + A11*self.vc.K1 + 1.0);
                self.vc.D5 = (B01*self.vc.R1 + A01*self.vc.K1) * L2;
                self.vc.D6 = self.vc.R1 * L2;
                self.vc.D7 = self.vc.K1 * L2;
            },
            ActionType::Pluck => {
                self.vc.phix2 = self.sc.sqrt_two_div_L * ( self.vc.x2*&self.inside_phi ).sin();
                let A02 = (&self.sc.X3 * &self.vc.phix0 * &self.vc.phix2).sum();
                let A12 = (&self.sc.X3 * &self.vc.phix1 * &self.vc.phix2).sum();
                let A22 = (&self.sc.X3 * &self.vc.phix2 * &self.vc.phix2).sum();

                // const float B02 = (sc.Y3 * vc.phix0 * vc.phix2).sum();
                // const float B12 = (sc.Y3 * vc.phix1 * vc.phix2).sum();
                // const float B22 = (sc.Y3 * vc.phix2 * vc.phix2).sum();

                // Eigen::Matrix3f matrix_A;
                // matrix_A <<
                //         B00*vc.R0 + A00*vc.K0 + 1.0f, B01*vc.R0 + A01*vc.K0,        B02*vc.R0 + A02*vc.K0,
                //             B01*vc.R1 + A01*vc.K1,        B11*vc.R1 + A11*vc.K1 + 1.0f, B12*vc.R1 + A12*vc.K1,
                //             B02*vc.R2 + A02*vc.K2,        B12*vc.R2 + A12*vc.K2,        B22*vc.R2 + A22*vc.K2 + 1.0f;
                // //qr.compute(
                // //    matrix_A);

                // inv_A = matrix_A.inverse();
            }
            _ => (),
        }

        todo!();
    }

    pub fn setup_vc_positions(&mut self)
    {
        self.vc.x1 = self.va.finger_position;
        self.vc.K1 = self.va.Kf;
        self.vc.R1 = R_FINGER * (self.va.Kf / K_FINGER);

        match self.ss.actions {
            ActionType::Bow => {
                self.vc.x0 = self.va.bow_pluck_position;
                self.vc.x2 = 0.0;
            },
            ActionType::Release => {
                self.vc.x2 = 0.0;
                self.vc.K2 = 0.0;
                self.vc.R2 = 0.0;
                if self.va.finger_position == 0.0 {
                    self.vc.x0 = 0.0;
                    self.vc.K0 = 0.0;
                    self.vc.R0 = 0.0;
                } else {
                    self.vc.K0 = self.vc.K1;
                    self.vc.R0 = self.vc.R1;
                    if self.va.finger_position < self.pc.L - FINGER_WIDTH {
                        self.vc.x0 = self.va.finger_position + FINGER_WIDTH;
                    } else {
                        let remaining_string = self.pc.L - self.va.finger_position;
                        self.vc.x0 = self.va.finger_position + 0.5 *remaining_string;
                    }
                }   
            },
            ActionType::Pluck => {
                self.vc.x0 = self.va.bow_pluck_position;
                self.vc.K0 = K_PLUCK;
                self.vc.R0 = R_PLUCK;
                self.vc.x2 = self.va.bow_pluck_position + PLUCK_WIDTH;
                self.vc.K2 = K_PLUCK;
                self.vc.R2 = R_PLUCK;
            }
            _ => (),
        }
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

    pub fn finger(&mut self, command: &Action){
        if let Some(ratio_from_nut) = &command.position && let Some(force) = &command.force {
            if *ratio_from_nut == 0.0 {
                self.va.finger_position = 0.;
            } else {
                self.va.finger_position = self.pc.L * (1.0 - ratio_from_nut);
            }
            self.va.Kf = force * K_FINGER;
            self.vc.recache = true;
        } else {
            warn!("Finger command is invalid: {:?}", &command);
        }
    }

    pub fn pluck(&mut self, command: &Action){
        if let Some(ratio_from_bridge) = &command.position && let Some(pull_distance) = &command.force {
            self.va.bow_pluck_position = self.pc.L * ratio_from_bridge;
            self.ss.actions = ActionType::Pluck;
            self.vc.pluck_samples_remaining = (PLUCK_SECONDS * self.fs) as u32;
            self.vc.y_pluck = 0.0;
            self.vc.y_pluck_target = pull_distance * PLUCK_DISPLACEMENT;
            self.vc.recache = true;
        } else {
            warn!("Pluck command is invalid: {:?}", &command);
        }
    }

    pub fn fill_buffer_forces(&mut self, _input: &[f32], _num_samples: u32){
        if self.vc.recache {
            self.cache_pa_c();
        }
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

    // bow
    D1: f32,
    D2: f32,
    D3: f32,
    D4: f32,

    // finger during bowing
    D5: f32,
    D6: f32,
    D7: f32,
    
    // pluck release
    D8: f32,
    D9: f32,
    D10: f32,
    D11: f32,

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


#[derive(Debug)]
pub enum StringNumber {
    One,
    Two,
    Three,
    Four
}

impl FromStr for StringNumber {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = match s {
            "0" => Self::One,
            "1" => Self::Two,
            "2" => Self::Three,
            "3" => Self::Four,
            _ => Err(eyre!("Failed to convert '{s}' to string number"))?,
        };

        Ok(action)
    }
}

impl StringNumber {
    pub fn index(&self) -> usize {
        match &self {
            StringNumber::One => 0,
            StringNumber::Two  => 1,
            StringNumber::Three  => 2,
            StringNumber::Four => 3,
        }
    }
}