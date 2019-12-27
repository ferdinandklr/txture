// import needed libraries
use rand::Rng;
use crate::error::Error;
use crate::perlin_noise::PerlinNoise;

// define the harmonic perlin noise generator
pub struct PerlinNoiseHarmonic {
    image_width: u32,
    initial_gradient_point_nbr: u32,
    tilable: bool,
    sub_perlin_noise: Vec<PerlinNoise>,
    number_of_harmonics: u8
}

impl PerlinNoiseHarmonic {

    // this function generates a new harmonic perlin noise
    pub fn new(image_width: u32, initial_gradient_point_nbr: u32, tilable: bool, number_of_harmonics: u8) -> Result<PerlinNoiseHarmonic, Error> {

    // CHECK BEFORE GENERATING NOISE //
        // check if requirements are acceptable
        if initial_gradient_point_nbr*(2 as u32).pow(number_of_harmonics as u32+1) > image_width {
            return Err(Error::TooManyGradientPointsRequested);
        }

    // START CREATING HARMONICS //
        let mut sub_perlin_noise: Vec<PerlinNoise> = Vec::new();
        for i in 0..number_of_harmonics+1 {
            sub_perlin_noise.push(PerlinNoise::new(
                image_width,
                initial_gradient_point_nbr * (2 as u32).pow(i as u32),
                tilable
            ).unwrap());
        }

    // RETURN THE CALCULATED HARMONIC NOISE //
        Ok(PerlinNoiseHarmonic {
            image_width,
            initial_gradient_point_nbr,
            tilable,
            sub_perlin_noise,
            number_of_harmonics
        })

    }

    // get a given pixel brightness
    pub fn get_pixel(&self, x: u32, y: u32) -> f64 {
        let mut brightness: f64 = 0.0;
        for i in 0..self.number_of_harmonics+1 {
            brightness += self.sub_perlin_noise[i as usize].get_pixel(x, y)*(0.5 as f64).powf(i as f64);
        }
        brightness / 2.0
    }

    // get a given pixel value [0-255]
    pub fn get_pixel_value(&self, x: u32, y: u32) -> u8 {
        (self.get_pixel(x, y) * 256.0) as u8
    }

}
