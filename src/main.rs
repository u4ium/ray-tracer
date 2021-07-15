use ndarray::array;
use ray_tracer::{
    camera::Camera,
    matrix::AffineTransformation,
    object::{Object, ObjectShape::Sphere},
    ppm::writer::write_to_ppm,
    scene::Scene,
    vector::HVector,
};
use std::io;

fn main() -> io::Result<()> {
    // TODO: get config from command line
    let mut scene = Scene::new();
    let sphere = Object::new(
        Sphere,
        AffineTransformation {
            scale: [2.0, 4.0, 2.0],
            position: [1.0, 0.0, 5.0],
            orientation: (0.0, 0.0),
        },
    );
    scene.add_object(sphere);
    let camera = Camera::new(HVector::new(array![0.0, 0.0, -1.0, 1.0]), (32, 16));
    let image = camera.generate_image(&scene, 0);
    write_to_ppm(image, "test.ppm")?;
    Ok(())
}
