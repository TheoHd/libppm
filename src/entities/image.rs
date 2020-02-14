use super::pixel::Pixel;
use ansi_term::Colour::RGB;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Create an image with an pixel buffer and width and height properties
pub struct Image {
    buffer: Vec<Pixel>,
    height: usize,
    width: usize,
}

/// Function of the Image object
impl Image {
    /// Return the pixel buffer of the image
    pub fn buffer(&self) -> &Vec<Pixel> {
        &self.buffer
    }

    /// Return the height of the image
    pub fn height(&self) -> &usize {
        &self.height
    }

    /// Return the width of the image
    pub fn width(&self) -> &usize {
        &self.width
    }

    /// Transform an image to grayscale
    pub fn grayscale(&mut self) {
        for pixel in &mut self.buffer {
            pixel.grayscale();
        }
    }

    /// Transform an image to negative
    pub fn invert(&mut self) {
        for pixel in &mut self.buffer {
            pixel.invert();
        }
    }

    /// Display the image with colors in the command line
    pub fn display(self) {
        for j in 0..self.height * self.width {
            self.buffer()[j].display();
            print!(" ");
            if (j + 1) % self.width == 0 {
                println!();
            }
        }
    }

    /// Get the first element of a Split object
    fn get_first_split_elem(line: &str, c: &str) -> String {
        for el in line.split(c) {
            return String::from(el);
        }
        String::new()
    }

    /// Filter comments in a ppm file
    fn filter_comments(line: &str) -> String {
        let mut ack_line: String;
        if line.contains("#") {
            ack_line = Self::get_first_split_elem(line, "#");
            if ack_line != "" {
                ack_line = ack_line;
            }
        } else {
            ack_line = String::from(line);
        }
        return ack_line;
    }

    /// Filter lines in a ppm file
    fn filter_lines(lines: &Vec<&str>) -> Vec<String> {
        let mut filtered_lines: Vec<String> = Vec::new();
        for line in lines {
            filtered_lines.push(Self::filter_comments(&line));
        }
        return filtered_lines;
    }

    /// Convert pixel values to str to inject them in a save file
    fn pixel_values_as_str(&self) -> String {
        let mut str_vec: Vec<String> = Vec::new();
        for j in 0..self.buffer.len() {
            str_vec.push(self.buffer[j].red().to_string());
            str_vec.push(String::from(" "));
            str_vec.push(self.buffer[j].green().to_string());
            str_vec.push(String::from(" "));
            str_vec.push(self.buffer[j].blue().to_string());
            str_vec.push(String::from("\n"));
        }
        str_vec.concat()
    }

    /// Save the current Image object to the given path
    pub fn save(&self, filename: &Path) {
        let mut file = File::create(filename).unwrap();
        file.write_all(
            format!(
                "{}\n{} {}\n{}\n{}",
                "P3",
                self.width(),
                self.height(),
                "255",
                self.pixel_values_as_str()
            )
            .as_bytes(),
        )
        .unwrap();
    }

    /// Debug a string in the command line, used as private to avoid public calling debug_str
    #[allow(dead_code)]
    fn debug_str(val: &String) {
        println!("{}", RGB(255, 0, 0).paint(format!("DEBUG: {}", val)));
    }

    /// Debug a usize in the command line, used as private to avoid public calling debug_str
    #[allow(dead_code)]
    fn debug_usize(val: usize) {
        println!("{}", RGB(255, 0, 0).paint(format!("DEBUG: {}", val)));
    }

    /// Debug an Image object in the command line, used as private to avoid public calling debug_str
    #[allow(dead_code)]
    fn debug_current_variables(
        width: &usize,
        height: &usize,
        max_val: &str,
        pixel_values: &Vec<u8>,
    ) {
        println!("width = {}", width);
        println!("height = {}", height);
        println!("max_val = {}", max_val);
        for (i, pixel_value) in pixel_values.iter().enumerate() {
            println!("{}:{}", i, pixel_value);
        }
    }

    /// Initialize a buffer from the u8 pixel values, width and height
    fn initialize_buffer(width: &usize, height: &usize, pixel_values: &Vec<u8>) -> Vec<Pixel> {
        let mut buffer: Vec<Pixel> = Vec::new();
        for j in 0..height * width {
            buffer.push(Pixel::new(
                pixel_values[3 * j], // 0, 3, 6...
                pixel_values[(3 * j) + 1], // 1, 4, 7...
                pixel_values[(3 * j) + 2], // 2, 5, 8...
            ));
        }
        buffer
    }

    /// Panic if first line is unsupported
    fn check_supported_format(v: &Vec<String>, i: &usize){
        if v[*i].trim() != "P3" {
            panic!();
        }
    }

    /// Panic if ppm line is unsupported, return width otherwise
    fn return_width_and_height(v: &Vec<String>, i: &usize) -> Vec<usize>{
        let width_and_height_vec: Vec<&str> = v[*i].split_whitespace().collect();
        if width_and_height_vec.len() != 2 {
            panic!();
        }
        let mut result = Vec::new();
        result.push(width_and_height_vec[0].trim().parse::<usize>().unwrap());
        result.push(width_and_height_vec[1].trim().parse::<usize>().unwrap());
        result
    }

    /// Return the maximum value of a pixel
    fn return_maximum_value(v: &Vec<String>, i: &usize) -> String{
        let max_vec : Vec<&str> = v[*i].split_whitespace().collect();
        String::from(max_vec[0])
    }

    /// Add pixel values in a vector
    fn add_pixel_values(v: &Vec<String>, i: &usize, pixel_values: &mut Vec<u8>){
        for item in v[*i].split_whitespace() {
            pixel_values.push(item.trim().parse::<u8>().unwrap())
        }
    }

    /// Create an Image object from a file
    pub fn new_with_file(filename: &Path) -> Self {
        let mut file = File::open(&filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Couldn't retrieve the file string content");
        let lines_uncommented: Vec<String> = Self::filter_lines(&(contents.split("\n").collect()));
        let mut width: usize = 0;
        let mut height: usize = 0;
        let mut _max_val : String = String::from("");
        let mut pixel_values: Vec<u8> = Vec::new();

        // Parse the values into variables
        for i in 0..lines_uncommented.len() {
            match i {
                0 => Self::check_supported_format(&lines_uncommented, &i),
                1 => {
                    let width_and_height = Self::return_width_and_height(&lines_uncommented, &i);
                    width = width_and_height[0];
                    height = width_and_height[1];
                },
                2 => _max_val = Self::return_maximum_value(&lines_uncommented, &i),
                _ => Self::add_pixel_values(&lines_uncommented, &i ,&mut pixel_values)
            }
        }

        // Import the variables into Image struct
        let buffer: Vec<Pixel> = Self::initialize_buffer(&width, &height, &pixel_values);

        // Return the Image object
        Self {
            buffer,
            height,
            width,
        }
    }
}