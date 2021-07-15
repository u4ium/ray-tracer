use crate::{image::Image, ray::Ray, scene::Scene, vector::HVector};
use ndarray::array;

pub struct Camera {
    position: HVector,
    resolution: (usize, usize),
}
impl Camera {
    pub fn new(position: HVector, resolution: (usize, usize)) -> Camera {
        Camera {
            position,
            resolution,
        }
    }

    pub fn generate_image(&self, scene: &Scene, depth: u8) -> Image {
        let mut image = Image::new(self.resolution);
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
        let (height, width) = self.resolution;
        let row_position = row as f64 / ((width as f64 - 1.0) / 2.0) - 1.0;
        let column_position = column as f64 / ((height as f64 - 1.0) / 2.0) - 1.0;
        HVector::new(array![row_position, column_position, 0.0, 1.0])
    }
}
