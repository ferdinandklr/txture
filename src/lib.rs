/*

    PERLIN NOISE GENERATOR
    by Ferdinand Keller - 2019

    this lib create a simple interface between user and noise (｡◕‿◕｡)

*/

// IMPORT NEEDED LIBRARIES

// import the custom errors
mod error;
pub use crate::error::Error;

// import the perlin noise generator
mod perlin_noise;
pub use crate::perlin_noise::PerlinNoise;
mod harmonic_perlin_noise;
pub use crate::harmonic_perlin_noise::HarmonicPerlinNoise;
