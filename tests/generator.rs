/*

    THIS CODES DOESN'T COMPILE IN THE FINAL BINARIES !

     its only purpose is to check the code is working
*/

// import the needed libraries
use perlin::PerlinNoise;
use bmp::{ Image, Pixel };

// this test create a picture
#[test]
fn generate_perlin_picture() -> Result<(), std::io::Error> {

    // define picture's dimenions
    let picture_width = 256;

    // create a new perlin noise
    let perlin_noise = PerlinNoise::new(picture_width, 4, true).unwrap();

    // create a new picture
    let mut picture = Image::new(picture_width, picture_width);

    // make picture look like perlin noise
    for j in 0..picture_width as u32 {
        for i in 0..picture_width as u32 {
            let gray: u8 = perlin_noise.get_pixel_value(i, j);
            picture.set_pixel(i, j, Pixel::new(gray, gray, gray));
        }
    }

    // save picture
    picture.save("tests/render.bmp")?;

    // exit
    Ok(())

}
