use ndarray::Array2;
use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, Debug)]
pub struct Colour {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

pub const BLACK: Colour = Colour {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};

pub const WHITE: Colour = Colour {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
};

pub struct Resolution {
    pub height: usize,
    pub width: usize,
}

pub struct Image {
    pub pixels: Array2<Colour>,
}
impl Image {
    pub fn new(resolution: &Resolution) -> Image {
        Image {
            pixels: Array2::from_elem((resolution.height, resolution.width), BLACK),
        }
    }
}

pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
}
impl Pixel {
    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.red, self.green, self.blue)
    }
}

impl Colour {
    pub fn to_pixel(&self) -> Pixel {
        fn pixel_from_colour(colour: f64) -> u8 {
            // TODO: assert!(0.0 >= pixel && pixel <= 1.0);
            (255.0 * colour) as u8
        }
        Pixel {
            red: pixel_from_colour(self.red),
            green: pixel_from_colour(self.green),
            blue: pixel_from_colour(self.blue),
        }
    }

    pub fn scale(&self, factor: f64) -> Colour {
        Colour {
            red: self.red * factor,
            green: self.green * factor,
            blue: self.blue * factor,
        }
    }
}

impl Add for Colour {
    type Output = Colour;

    fn add(self, rhs: Colour) -> Self::Output {
        Colour {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl AddAssign for Colour {
    fn add_assign(&mut self, rhs: Colour) {
        *self = *self + rhs;
    }
}
