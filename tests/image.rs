#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use ppm::entities::image::Image;
    use ppm::entities::pixel::Pixel;
    use std::path::Path;
    use test::Bencher;

    /// Test image importation
    #[test]
    pub fn image_import_works() {
        let img: Image = Image::new_with_file(Path::new("img/test.ppm"));
        img.save(Path::new("img/save.ppm"));
    }


    #[bench]
    pub fn bench_save(b: &mut Bencher) {
        let img: Image =  Image::new_with_file(Path::new("img/test.ppm"));
        b.iter(|| img.save(Path::new("img/save.ppm")));
    }

    /// Test image grayscale
    #[test]
    pub fn image_grayscale_works() {
        let img: Image = Image::new_with_file(Path::new("img/test.ppm"));
        let img_old_values: &Vec<Pixel> = img.buffer();
        let mut img2: Image = Image::new_with_file(Path::new("img/test.ppm"));
        img2.grayscale();
        let img_new_values: &Vec<Pixel> = img2.buffer();
        for i in 0..img_old_values.len() {
            let ro = img_old_values[i].red();
            let go = img_old_values[i].green();
            let bo = img_old_values[i].blue();
            let rn = img_new_values[i].red();
            let gn = img_new_values[i].green();
            let bn = img_new_values[i].blue();
            let grayscale_old = ro / 3 + go / 3 + bo / 3;

            assert_eq!(&grayscale_old, rn);
            assert_eq!(&grayscale_old, gn);
            assert_eq!(&grayscale_old, bn);
        }
    }

    /// Test image inversion
    #[test]
    pub fn image_inversion_works() {
        let img: Image = Image::new_with_file(Path::new("img/test.ppm"));
        let img_old_values: &Vec<Pixel> = img.buffer();
        let mut img2: Image = Image::new_with_file(Path::new("img/test.ppm"));
        img2.invert();
        let img_new_values: &Vec<Pixel> = img2.buffer();
        for i in 0..img_old_values.len() {
            let ro = img_old_values[i].red();
            let go = img_old_values[i].green();
            let bo = img_old_values[i].blue();
            let rn = img_new_values[i].red();
            let gn = img_new_values[i].green();
            let bn = img_new_values[i].blue();

            assert_eq!(255 - rn, *ro);
            assert_eq!(255 - gn, *go);
            assert_eq!(255 - bn, *bo);
        }
    }

    /// Display the image in the command line
    #[test]
    pub fn image_display_works() {
        let img: Image = Image::new_with_file(Path::new("img/test.ppm"));
        img.display();
    }

    // Display the image in the command line after grayscale
    #[test]
    pub fn image_display_grayscale_works() {
        let mut img: Image = Image::new_with_file(Path::new("img/test.ppm"));
        img.grayscale();
        img.display();
    }

    // Display the image in the command line after inversion
    #[test]
    pub fn image_display_inversion_works() {
        let mut img: Image = Image::new_with_file(Path::new("img/test.ppm"));
        img.invert();
        img.display();
    }
}