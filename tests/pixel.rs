use ppm::entities::pixel::Pixel;

/// Test if pixel have the same RGB values of the instantiated object
#[test]
fn pixel_creation_works() {
    let pixel = Pixel::new(128, 128, 128);
    assert_eq!(pixel.red(), &128u8);
    assert_eq!(pixel.green(), &128u8);
    assert_eq!(pixel.blue(), &128u8);
}

/// Prints a pixel
#[test]
fn pixel_display_works() {
    let pixel = Pixel::new(128, 64, 196);
    pixel.display();
}

/// Test if the inversion of a Pixel works
#[test]
fn pixel_invert_works() {
    let mut pixel = Pixel::new(128, 64, 196);
    pixel.invert();
    assert_eq!(
        (&(255u8 - 128u8), &(255u8 - 64u8), &(255u8 - 196u8)),
        (pixel.red(), pixel.green(), pixel.blue())
    );
}

/// Test the equality between two pixels
#[test]
fn pixel_eq_works() {
    let pixel_1 = Pixel::new(128, 64, 196);
    let pixel_2 = Pixel::new(128, 64, 196);
    let pixel_3 = Pixel::new(127, 63, 195);

    assert_eq!(true, pixel_1 == pixel_2);
    assert_eq!(false, pixel_1 == pixel_3);
}

/// Test the grayscale function
#[test]
fn pixel_grayscale_works() {
    let mut pixel = Pixel::new(128, 64, 178);
    let rgb_mean: u8 = 128 / 3 + 64 / 3 + 178 / 3;
    pixel.grayscale();
    assert_eq!(&rgb_mean, pixel.red());
    assert_eq!(&rgb_mean, pixel.green());
    assert_eq!(&rgb_mean, pixel.blue());
}

/// Test if mutability is unique for each pixel object
#[test]
fn pixel_mutability_works() {
    let mut pixel_1 = Pixel::new(128, 64, 23);
    let pixel_2 = Pixel::new(255, 255, 255);
    pixel_1.grayscale();
    assert_eq!(true, pixel_1 != pixel_2);
}
