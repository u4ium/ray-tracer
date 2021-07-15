use ndarray::Array2;

#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}
pub const BLACK: Pixel = Pixel {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
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
        fn colour_from_float_to_int(colour: f32) -> u8 {
            // TODO: assert!(0.0 >= colour && colour <= 1.0);
            (255.0 * colour) as u8
        }
        Colour {
            red: colour_from_float_to_int(self.red),
            green: colour_from_float_to_int(self.green),
            blue: colour_from_float_to_int(self.blue),
        }
    }
}
