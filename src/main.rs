use ndarray::array;
use ray_tracer::{
    camera::Camera,
    image::Resolution,
    light::Light,
    object::{material::Material, matrix::AffineTransformation, Object, ObjectShape::Sphere},
    ppm::writer::write_to_ppm,
    scene::Scene,
    vector::HVector,
};
use std::io;

fn main() -> io::Result<()> {
    // TODO: get config from command line
    let sphere = Object::new(
        Sphere,
        AffineTransformation {
            scale: [1.0, 3.0, 1.0],
            position: [1.0, 0.5, 3.0],
            orientation: (0.0, 0.0),
        },
        Material::new(0.5, 0.3, 0.2, 0.7),
    );
    let light = Light::new([-3.0, 20.0, 1.0]);
    let scene = Scene::new(vec![sphere], vec![light]);
    let camera = Camera::new(
        HVector::new(array![0.0, 0.0, -1.0]),
        Resolution {
            width: 256,
            height: 144,
        },
    );
    let image = camera.generate_image(&scene, 0);
    write_to_ppm(image, "test.ppm")?;
    Ok(())
}
