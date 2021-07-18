use ndarray::Array2;
use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

pub const BLACK: Pixel = Pixel {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};

pub const WHITE: Pixel = Pixel {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
};

pub struct Resolution {
    pub height: usize,
    pub width: usize,
}

pub struct Image {
    pub pixels: Array2<Pixel>,
}
impl Image {
    pub fn new(resolution: &Resolution) -> Image {
        Image {
            pixels: Array2::from_elem((resolution.height, resolution.width), BLACK),
        }
    }
}

pub struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}
impl Colour {
    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.red, self.green, self.blue)
    }
}

impl Pixel {
    pub fn to_colour(&self) -> Colour {
        fn colour_from_float_to_int(colour: f64) -> u8 {
            // TODO: assert!(0.0 >= colour && colour <= 1.0);
            (255.0 * colour) as u8
        }
        Colour {
            red: colour_from_float_to_int(self.red),
            green: colour_from_float_to_int(self.green),
            blue: colour_from_float_to_int(self.blue),
        }
    }

    pub fn scale(&self, factor: f64) -> Pixel {
        Pixel {
            red: self.red * factor,
            green: self.green * factor,
            blue: self.blue * factor,
        }
    }
}

impl Add for Pixel {
    type Output = Pixel;

    fn add(self, rhs: Pixel) -> Self::Output {
        Pixel {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl AddAssign for Pixel {
    fn add_assign(&mut self, rhs: Pixel) {
        *self = *self + rhs;
    }
}
