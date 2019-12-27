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
    gradient_point_nbr: u32,
    tilable: bool,
    gradient_grid_width: f64,
    gradient_grid: Vec<Vec<usize>>
}

impl PerlinNoise {

    // this function generates a new perlin noise
    pub fn new(image_width: u32, gradient_point_nbr: u32, tilable: bool) -> Result<PerlinNoise, Error> {

    // CHECKUP BEFORE GENERATING NOISE
        // check if requirements are acceptable
        if gradient_point_nbr > image_width {
            return Err(Error::TooManyGradientPoints);
        }

    // START NOISE GENERATION
        // create the random generator
        let mut rng = rand::thread_rng();

        // create the gradient variable
        let mut gradient_grid: Vec<Vec<usize>> = Vec::new();

        // generate the gradient
        for _ in 0..gradient_point_nbr+1 {
            let mut gradient_grid_line: Vec<usize> = Vec::new();
            for _ in 0..gradient_point_nbr+1 {
                // create a random gradient
                gradient_grid_line.push((rng.gen::<f64>()*4.0) as usize);
            }
            if tilable {
                // make sure the noise repeat on x axis
                gradient_grid_line.push(gradient_grid_line[0]);
            }
            // save calculated line
            gradient_grid.push(gradient_grid_line);
        }
        // compute last line
        let mut gradient_grid_line: Vec<usize> = Vec::new();
        if tilable {
            // make sure the noise repeats on y axis
            for i in 0..gradient_point_nbr+1 {
                gradient_grid_line.push(gradient_grid[0][i as usize]);
            }
        }
        gradient_grid.push(gradient_grid_line);

    // SAVE THE CALCULATED NOISE
        // generate the noise
        Ok(PerlinNoise {
            image_width,
            gradient_point_nbr,
            tilable,
            gradient_grid_width: image_width as f64 / gradient_point_nbr as f64,
            gradient_grid
        })

    }

    // get a given pixel brightness
    pub fn get_pixel(&self, x: u32, y: u32) -> f64 {

        // // get pixel's origin index
        // let po: [u32; 2] = [y/self.gradient_grid_width, x/self.gradient_grid_width];
        // // get pixel's remaped position
        // let pm: [f64; 2] = [
        //     (y%self.gradient_grid_width) as f64/self.gradient_grid_width as f64,
        //     (x%self.gradient_grid_width) as f64/self.gradient_grid_width as f64
        // ];
        // // get each dot product
        // let d0 = PerlinGenerator::dot([-pm[0],pm[1]], DEFAULT_GRADIENTS[self.gradient_points[po[0] as usize][po[1] as usize]]);
        // let d1 = PerlinGenerator::dot([-pm[0],pm[1]-1.0], DEFAULT_GRADIENTS[self.gradient_points[po[0] as usize][po[1] as usize+1]]);
        // let d2 = PerlinGenerator::dot([-pm[0]+1.0,pm[1]], DEFAULT_GRADIENTS[self.gradient_points[po[0] as usize+1][po[1] as usize]]);
        // let d3 = PerlinGenerator::dot([-pm[0]+1.0,pm[1]-1.0], DEFAULT_GRADIENTS[self.gradient_points[po[0] as usize+1][po[1] as usize+1]]);
        // // calculate brightness
        // PerlinGenerator::double_lerp(d0, d1, d2, d3, pm[1], pm[0])

        y as f64 / self.image_width as f64
    }

    // get a given pixel value [0-255]
    pub fn get_pixel_value(&self, x: u32, y: u32) -> u8 {
        (self.get_pixel(x, y) * 256.0) as u8
    }

    // functions neededed during the calculation process
    fn dot(vect1: [f64;2], vect2: [f64;2]) -> f64 {
        (((vect1[0]*vect2[0]+vect1[1]*vect2[1]))+1.0)/2.0
    }
    fn fade(value: f64) -> f64 {
        ((6.0*value-15.0)*value+10.0)*value*value*value
    }
    fn lerp(v1: f64, v2: f64, t: f64) -> f64 {
        v1+(v2-v1)*PerlinGenerator::fade(t)
    }
    fn double_lerp(v1:f64, v2:f64, v3:f64, v4:f64, t1:f64, t2:f64) -> f64 {
        PerlinGenerator::lerp(PerlinGenerator::lerp(v1, v2, t1), PerlinGenerator::lerp(v3, v4, t1), t2)
    }

}
