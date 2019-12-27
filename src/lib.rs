// import needed libraries
use std::fmt;
use rand::Rng;

// define common lib's errors for end user
pub enum Error {
    TooManyGradientPoints
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Error::TooManyGradientPoints => "You asked for a noise with too many gradient points"
        })
    }
}

// define the default gradients used in perlin noise generation
const DEFAULT_GRADIENTS: [[f64; 2]; 4] = [[1.0,1.0],[1.0,-1.0],[-1.0,1.0],[-1.0,-1.0]];

// define the perlin noise generator
pub struct PerlinNoise {
    image_width: u32,
    gradient_points_nbr: u32,
    gradient_grid_width: f64
}

impl PerlinNoise {

    // this function generates a new perlin noise
    pub fn new(image_width: u32, gradient_points_nbr: u32) -> Result<PerlinNoise, Error> {

        // check if requirements are acceptable
        if gradient_points_nbr > image_width {
            return Err(Error::TooManyGradientPoints);
        }

        // generate the noise
        Ok(PerlinNoise {
            image_width,
            gradient_points_nbr,
            gradient_grid_width: image_width as f64 / gradient_points_nbr as f64
        })

    }

    // get a given pixel brightness
    pub fn get_pixel(&self, x: u32, y: u32) -> f64 {
        y as f64 / self.image_width as f64
    }

    // get a given pixel value [0-255]
    pub fn get_pixel_value(&self, x: u32, y: u32) -> u8 {
        (self.get_pixel(x, y) * 256.0) as u8
    }

}
