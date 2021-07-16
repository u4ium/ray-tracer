use crate::{
    image::{Image, Resolution},
    ray::Ray,
    scene::Scene,
    vector::HVector,
};
use ndarray::array;

pub struct Camera {
    position: HVector,
    resolution: Resolution,
}
impl Camera {
    pub fn new(position: HVector, resolution: Resolution) -> Camera {
        Camera {
            position,
            resolution,
        }
    }

    pub fn generate_image(&self, scene: &Scene, depth: u8) -> Image {
        let mut image = Image::new(&self.resolution);
        for (coordinates, pixel) in image.pixels.indexed_iter_mut() {
            let pixel_position = self.get_pixel_position(coordinates);
            let from = self.position.clone();
            let direction = (pixel_position - from.clone()).normalized();
            let ray = Ray { from, direction };
            *pixel = scene.trace(&ray, depth);
        }
        image
    }
    fn get_pixel_position(&self, (row, column): (usize, usize)) -> HVector {
        let row_position = row as f64 / ((self.resolution.height as f64 - 1.0) / 2.0) - 1.0;
        let column_position = column as f64 / ((self.resolution.width as f64 - 1.0) / 2.0) - 1.0;
        HVector::new(array![column_position, -row_position, 0.0, 1.0])
    }
}
