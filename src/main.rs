use std::env;
use ::ppm::entities::image::Image;
use std::path::Path;

fn main(){
    let args: Vec<String> = env::args().collect();
    let keyword = "yvan".to_string();

    if args.len() == 2 && args[1].to_string() == keyword {
        println!("==============================");
        println!("======= prof.ppm image =======");
        println!("==============================");
        let image = Image::new_with_file(Path::new("img/prof.ppm"));
        image.display();
        let mut image = Image::new_with_file(Path::new("img/prof.ppm"));
        image.grayscale();
        image.display();
        let mut image = Image::new_with_file(Path::new("img/prof.ppm"));
        image.invert();
        image.display();
    } else {
        println!("==============================");
            println!("======= test.ppm image =======");
            println!("==============================");
            let image = Image::new_with_file(Path::new("img/test.ppm"));
            image.display();
            let mut image = Image::new_with_file(Path::new("img/test.ppm"));
            image.grayscale();
            image.display();
            let mut image = Image::new_with_file(Path::new("img/test.ppm"));
            image.invert();
            image.display();

            println!("==============================");
            println!("========= dog.ppm image ======");
            println!("==============================");
            let image = Image::new_with_file(Path::new("img/dog.ppm"));
            image.display();
            let mut image = Image::new_with_file(Path::new("img/dog.ppm"));
            image.grayscale();
            image.display();
            let mut image = Image::new_with_file(Path::new("img/dog.ppm"));
            image.invert();
            image.display();


            println!("==============================");
            println!("========= cat.ppm image ======");
            println!("==============================");
            let image = Image::new_with_file(Path::new("img/cat.ppm"));
            image.display();
            let mut image = Image::new_with_file(Path::new("img/cat.ppm"));
            image.grayscale();
            image.display();
            let mut image = Image::new_with_file(Path::new("img/cat.ppm"));
            image.invert();
            image.display();
    }
}