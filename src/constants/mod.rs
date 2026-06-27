pub mod strings_violin;
pub mod lowpass;

// time length of each sample, in seconds
// const dt: f32 = 1.0 / ARTIFASTRING_SAMPLE_RATE;

pub const FS_MULTIPLICATION_FACTOR: [[u32; 4]; 3] = [
    [2,2,4,4], // Violin
    [1,1,3,3], // Viola
    [1,1,1,2], // Cello
];

// pluck constants, estimated from listening
const PLUCK_VELOCITY: f32 = 0.1; // in m/s
const PLUCK_DISPLACEMENT: f32 = 0.005; // in m
const PLUCK_SECONDS: f32 = 0.1; // in seconds

const K_FINGER: f32 = 1e5;
const R_FINGER: f32 = 30.0;
const K_PLUCK: f32  = 1e4;
const R_PLUCK: f32  = 1e1;

const PLUCK_WIDTH: f32 = 0.012; // m
const FINGER_WIDTH: f32 = 0.01; // m

// noise
const A_noise: f32 = 0.02; // estimated from listening

// // string physical constants
// #include "constants/strings_violin.h"
// #include "constants/strings_viola.h"
// #include "constants/strings_cello.h"

// #endif

