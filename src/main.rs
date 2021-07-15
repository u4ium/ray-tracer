use ndarray::array;
use ray_tracer::{
    camera::Camera,
    image::Resolution,
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
            scale: [1.0, 2.0, 1.0],
            position: [0.0, 0.0, 3.0],
            orientation: (0.0, 0.0),
        },
    );
    scene.add_object(sphere);
    let camera = Camera::new(
        HVector::new(array![0.0, 0.0, -1.0]),
        Resolution {
            width: 128,
            height: 72,
        },
    );
    let image = camera.generate_image(&scene, 0);
    write_to_ppm(image, "test.ppm")?;
    Ok(())
}
