use std::f32::consts::PI;
use ndarray::{arr1, Array1};

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
        let fs = fs_multiplier * instrument_sample_rate;
        let _dt = 1.0 / fs as f32;

        let _audio_samples: [f32; (NORMAL_BUFFER_SIZE*3) as usize];
        let _force_samples: [f32; (NORMAL_BUFFER_SIZE*3) as usize];

        // to handle the memory alignment once
        // let ah = vec![0.0; N]; // AA
        // AA adh;
        // AA fn;
        let n = arr1(&vec![0.0; N]); // AA
        // AA inside_phi;

        let mut string = Self{ N, n, fs_multiplier, pc, sc, vc, ss, va};
        string.set_physical_constants();

        // srand( time(NULL) );
        string.reset();

        return string;
    }

    #[allow(non_snake_case)]
    pub fn set_N(&mut self){
        let N = self.N;
        self.sc.X1.resize(N, 0.0);
        self.sc.X2.resize(N, 0.0);
        self.sc.X3.resize(N, 0.0);
        self.sc.Y1.resize(N, 0.0);
        self.sc.Y2.resize(N, 0.0);
        self.sc.Y3.resize(N, 0.0);
        self.sc.G.resize(N, 0.0);

        self.vc.phix0.resize(N, 0.0);
        self.vc.phix1.resize(N, 0.0);
        self.vc.phix2.resize(N, 0.0);

        self.ss.a.resize(N, 0.0);
        self.ss.ad.resize(N, 0.0);
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
        let N = self.N;
        let ones = vec![1.0; N];
        let rn = self.pc.rn;
        let n = self.n.clone();
        let pc = &self.pc;
        let sc = &self.sc;


        let w0 = (
            (pc.T/pc.pl) * (n.clone()*PI*sc.div_pc_L).iter().map(|x| x * x).collect::<Array1<f32>>()
            + (pc.E*I/pc.pl) * (n*PI*sc.div_pc_L).iter().map(|x| x * x * x).collect::<Array1<f32>>()
        ).iter().map(|x| x.sqrt()).collect::<Array1<f32>>();

        // let w0 = ( 
        // (pc.T/pc.pl) * ((n*PI*sc.div_pc_L).square())
        //            + (pc.E*I/pc.pl) * ((n*PI*sc.div_pc_L).square().square()) ).sqrt();


    //     AA rn(N);
    //     rn.resize(N);
    //     for (unsigned int i = 0; i < N; ++i) {
    //         rn(i) = pc.rn[i];
    //     }

    //     const AA w0 = ( (pc.T/pc.pl) * ((n*PI*sc.div_pc_L).square())
    //                     + (pc.E*I/pc.pl) * ((n*PI*sc.div_pc_L).square().square()) ).sqrt();
    //     const float highest_freq = w0[N-1] / (2*PI);
    //     if (highest_freq > fs/2.0) {
    //         cout<<"BAD FREQ!  highest freq: "<<highest_freq;
    //         cout<<"           Nyquist freq: "<<fs/2.0<<endl;
    //     }
    //     const AA w = (w0.square() - rn.square()).sqrt();

    //     sc.X1 = ((w*dt).cos() + (rn/w)*((w*dt).sin())) * ((-rn*dt).exp());
    //     sc.X2 = ((ones / w) * (w*dt).sin()) * ((-rn*dt).exp());
    //     sc.X3 = (ones - sc.X1) / (pc.pl * w0.square());
    //     //std::cout<<"X1"<<std::endl<<sc.X1<<std::endl;
    //     //std::cout<<"X3"<<std::endl<<sc.X3<<std::endl;

    //     sc.Y1 = -(w + rn.square()/w) * (w*dt).sin() * (-rn*dt).exp();
    //     sc.Y2 = ((w*dt).cos() - (rn/w)*((w*dt).sin())) * (-rn*dt).exp();
    //     sc.Y3 = -sc.Y1 / (pc.pl * w0.square());

    //     //std::cout<<"Y3"<<std::endl<<sc.Y3<<std::endl;

    //     sc.G = sc.sqrt_two_div_L * (pc.T*(n*PI*sc.div_pc_L) + pc.E*I*(n*PI*sc.div_pc_L).cube());

    //     inside_phi = n*PI*sc.div_pc_L;
    //     vc.recache = true;
    }

    pub fn reset(&self) {
        // init everything, just to be safe
        // cache_pc_c();

        // plucks = 0;

        // vc.x0  = 0.0f;
        // vc.x1  = 0.0f;
        // vc.x2  = 0.0f;
        // vc.y_pluck = 0.0f;
        // vc.y_pluck_target = 0.0f;
        // va.Fb  = 0.0f;
        // va.vb  = 0.0f;
        // va.vb_target = 0.0f;
        // va.va = 0.0f;
        // vc.pluck_samples_remaining = 0;
        // vc.recache = true;

        // va.finger_position = 0.0f;
        // va.bow_pluck_position = 0.0f;
        // va.Kf = K_FINGER;

        // vc.K0 = 0.0f;
        // vc.R0 = 0.0f;
        // vc.K2 = 0.0f;
        // vc.K2 = 0.0f;

        // ss.actions = OFF;

        // ss.a.setZero();
        // ss.ad.setZero();
        // ss.slipstate = 0;

        // debug_ticks = 0;
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
    X1: Vec<f32>, // displacement AA (Eigen Array)
    X2: Vec<f32>,
    X3: Vec<f32>,
    Y1: Vec<f32>, // velocity
    Y2: Vec<f32>,
    Y3: Vec<f32>,
    G: Vec<f32>, // bridge
    div_pc_L: f32,
    sqrt_two_div_L: f32,
}

#[derive(Default)]
#[allow(dead_code, non_snake_case)]
pub struct ViolinistCoefficients {
    x0: f32,
    x1: f32,
    x2: f32,
    phix0: Vec<f32>, // position eigenvalues (AA)
    phix1: Vec<f32>,
    phix2: Vec<f32>,

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
    a: Vec<f32>, // AA
    ad: Vec<f32>,
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
