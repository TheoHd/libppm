use ansi_term::Colour::RGB;

/// This structure define a pixel with RGB values in the interval from 0 to 255
pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
}

/// Pixel implementation have some functions that helps manipulating its RGB values
/// You can for example :
///     - Create a new pixel
///     - Invert the color of the pixel
///     - Display it in command line
impl Pixel {
    /// Describe the pixel by showing its values in the command line.
    /// Useful in debugging
    #[allow(dead_code)]
    fn describe(&self) {
        println!("RGB({}, {}, {})", &self.red, &self.green, &self.blue);
    }

    /// Return the rgb values of the Pixel as a tuple
    pub fn rgb(&self) -> (&u8, &u8, &u8) {
        (&self.red, &self.green, &self.blue)
    }

    /// Return the red value of the Pixel as a u8 (0-255)
    pub fn red(&self) -> &u8 {
        &self.red
    }

    /// Return the green value of the Pixel as a u8 (0-255)
    pub fn green(&self) -> &u8 {
        &self.green
    }

    /// Return the blue value of the Pixel as a u8 (0-255)
    pub fn blue(&self) -> &u8 {
        &self.blue
    }

    /// Create a new Pixel object
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, blue, green }
    }

    /// Transform the pixel to their grayscale value by doing the mean of its RGB values
    pub fn grayscale(&mut self) {
        let grayscale_value = self.red / 3 + self.green / 3 + self.blue / 3;
        self.red = grayscale_value;
        self.green = grayscale_value;
        self.blue = grayscale_value;
    }

    /// Transform the pixel to their negative value by inverting its RGB values
    pub fn invert(&mut self) {
        self.red = 255 - self.red;
        self.green = 255 - self.green;
        self.blue = 255 - self.blue;
    }

    /// Display the pixel in the command line
    pub fn display(&self) {
        print!("{}", RGB(self.red, self.green, self.blue).paint("■"))
    }
}

/// Implementation to verify if two pixels have the same RGB values
impl PartialEq for Pixel {
    /// Returns true if both Pixels RGB values are equal, false otherwise
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}
