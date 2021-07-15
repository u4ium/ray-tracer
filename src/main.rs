use ndarray::array;
use ray_tracer::{camera::Camera, ppm::writer::write_to_ppm, scene::Scene, vector::HVector};
use std::io;

fn main() -> io::Result<()> {
    // TODO: get config from command line
    let scene = Scene::new();
    let camera = Camera::new(HVector::new(array![0.0, 0.0, -1.0, 1.0]), (32, 16));
    let image = camera.generate_image(&scene, 0);
    write_to_ppm(image, "test.ppm")?;
    Ok(())
}
