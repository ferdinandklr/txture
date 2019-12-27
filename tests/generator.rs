/*

    THIS CODES ISN'T COMPILED INSIDE THE FINAL BINARIES !

     its only purpose is to check the code is working

*/

// import the needed libraries
use perlin::PerlinNoise;    // perlin noise generator
use bmp::{ Image, Pixel };  // a simpel libary to help create bmp pictures

// this test create a picture
#[test]
fn generate_perlin_picture() -> Result<(), std::io::Error> {

    // define picture's dimenions
    let picture_width: u32 = 256;

    // create a new perlin noise
    // args: picture's width, number of gradient points, should the picture be tilable
    // it returns a result with custom errors if didn't worked
    let perlin_noise = PerlinNoise::new(picture_width, 5, false).unwrap();

    // create a new picture
    let mut picture = Image::new(picture_width, picture_width);

    // save the perlin noise inside the picture
    for j in 0..picture_width {
        for i in 0..picture_width {
            let gray: u8 = perlin_noise.get_pixel_value(i, j);
            picture.set_pixel(i, j, Pixel::new(gray, gray, gray));
        }
    }

    // save picture on disk
    picture.save("tests/render.bmp")?;

    // exit
    Ok(())

}
