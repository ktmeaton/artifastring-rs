use crate::{constants::*, constants::lowpass::*, InstrumentType, InstrumentNumber};  

pub struct ArtifastringString {
    pub fs_multiply: u32,
}

impl ArtifastringString {
    pub fn new(
        instrument_type: InstrumentType,
        instrument_number: InstrumentNumber,
        instrument_sample_rate: u32,
        string_number: u32
    ) -> Self {
        let fs_multiply = FS_MULTIPLICATION_FACTOR[instrument_type.index()][string_number as usize];

        let fs = fs_multiply * instrument_sample_rate;
        let dt = 1.0 / fs as f32;

        // match instrument_type {
        //     InstrumentType::Violin => set_physical_constants( violin_params[instrument_number.index()][string_number] );
        // };

        Self {fs_multiply}
    }

    pub fn reset(&self) {
        todo!();
    }
}

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

// typedef struct {
//     float T;  /**< \brief Tension          (N) */
//     float L;  /**< \brief Length           (m) */
//     float d;  /**< \brief Diameter         (m) */
//     float pl; /**< \brief Linear Density   (kg/m) */
//     float E;  /**< \brief Young's elastic modulus */
//     float mu_s; /**< \brief Coefficient of static friction */
//     float mu_d; /**< \brief Coefficient of dynamic friction */
//     float v0; /**< \brief Slope of hyperbolic friction curve */
//     float cutoff; /**< \brief Minimum sum-of-squares amplitude to maintain processing */
//     unsigned int N; /**< \brief Number of modes for this string (should be a multiple of 4 for SSE, or 8 for AVX) */
//     float rn[MAX_MODAL_DECAY_MODES]; /**< Array of modal decays */
// } String_Physical;

