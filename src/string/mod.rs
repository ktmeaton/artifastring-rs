use crate::{
    ActionType,
    constants::*,
    constants::strings::strings_violin::VIOLIN_PARAMS, 
    InstrumentType,
    InstrumentNumber,
};

pub struct ArtifastringString {
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
        // originally capitalized N
        let _n = pc.n as usize;

        let _tick_output_force: f32;
        let _num_friction_skip_over_stick: u32;
        let _debug_ticks: u32;
        let _debug_string_num: u32;
    
        let sc =  StringConstants::default();
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
        // AA ah;
        // AA adh;
        // AA fn;
        // AA n;
        // AA inside_phi;

        let mut string = Self{ fs_multiplier, pc, sc, vc, ss, va};
        string.set_physical_constants();

        return string;
    }

    pub fn set_n(&mut self){
        let n = self.pc.n as usize;
        self.sc.x1.resize(n, 0.0);
        self.sc.x2.resize(n, 0.0);
        self.sc.x3.resize(n, 0.0);
        self.sc.y1.resize(n, 0.0);
        self.sc.y2.resize(n, 0.0);
        self.sc.y3.resize(n, 0.0);
        self.sc.g.resize(n, 0.0);

        self.vc.phix0.resize(n, 0.0);
        self.vc.phix1.resize(n, 0.0);
        self.vc.phix2.resize(n, 0.0);

        self.ss.a.resize(n, 0.0);
        self.ss.ad.resize(n, 0.0);
        // ah.resize(N);
        // adh.resize(N);
        // fn.resize(N);

        // n.resize(N);
        // inside_phi.resize(N);
        // n = AA::LinSpaced(Eigen::Sequential, N, 1, N);
    }

    pub fn set_physical_constants(&mut self) {
        self.set_n();
        self.cache_pc_c();
        self.vc.recache = true;    
    }

    pub fn cache_pc_c(&mut self) {

        self.sc.div_pc_l = 1.0 / self.pc.l;
    //     sc.sqrt_two_div_L = sqrt( 2.0f / pc.L);
    //     const float I = PI * pc.d*pc.d*pc.d*pc.d / 64.0f;

    //     AA ones(N);
    //     ones.resize(N);
    //     ones.setOnes();

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
        todo!();
    }
}

#[derive(Clone)]
pub struct StringPhysical {
     // Tension (N)
    pub t: f32,
    // Length (m)
    pub l: f32,
    // Diameter (m)
    pub d: f32,
    // Linear Density (kg/m)
    pub pl: f32,
    // Young's elastic modulus
    pub e: f32,
    // Coefficient of static friction
    pub mu_s: f32,
    // Coefficient of dynamic friction
    pub mu_d: f32,
    // Slope of hyperbolic friction curve
    pub v0: f32,
    // Minimum sum-of-squares amplitude to maintain processing
    pub cutoff: f32,
    // Number of modes for this string (should be a multiple of 4 for SSE, or 8 for AVX)
    pub n: u32,
    // Array of modal decays
    pub rn: [f32; 128]
}

#[derive(Default)]
#[allow(dead_code)]
pub struct StringConstants {
    x1: Vec<f32>, // displacement AA (Eigen Array)
    x2: Vec<f32>,
    x3: Vec<f32>,
    y1: Vec<f32>, // velocity
    y2: Vec<f32>,
    y3: Vec<f32>,
    g: Vec<f32>, // bridge
    div_pc_l: f32,
    sqrt_two_div_l: f32,
}

#[derive(Default)]
#[allow(dead_code)]
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
    k0: f32,
    k1: f32,
    k2: f32,
    r0: f32,
    r1: f32,
    r2: f32,
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
#[allow(dead_code)]
pub struct ViolinistActions {
    bow_pluck_position: f32,  // bow/pluck position
    finger_position: f32,  // finger position
    fb: f32,  // bow force
    vb: f32,  // bow velocity
    // iffy actions
    va: f32,  // bow acceleration, per dt (unit of time)
    vb_target: f32, // target bow velocity, used in acceleration
    kf: f32,
}
