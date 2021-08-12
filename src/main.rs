use ndarray::array;
use ray_tracer::{
    camera::Camera,
    image::{Colour, Resolution},
    light::Light,
    object::{material::Material, matrix::AffineTransformation, Object, ObjectShape::*},
    ppm::writer::write_to_ppm,
    scene::Scene,
    vector::HVector,
};
use std::{f64::consts::PI, io};

fn main() -> io::Result<()> {
    // TODO: get config from command line
    let sphere = Object::new(
        Sphere,
        AffineTransformation {
            scale: [1.0, 3.0, 1.0],
            position: [1.0, 0.5, 3.0],
            orientation: (0.0, 0.0),
        },
        Material::new(
            0.5,
            0.3,
            0.2,
            0.7,
            Colour {
                red: 1.0,
                green: 0.5,
                blue: 0.5,
            },
        ),
    );
    let triangle = Object::new(
        Triangle(
            // equilateral
            HVector::new(array![-1.0, 0.0, 0.0]),
            HVector::new(array![1.0, 0.0, 0.0]),
            HVector::new(array![0.0, 3.4641016151377545870548926830117, 0.0]),
        ),
        AffineTransformation {
            scale: [1.0, 1.0, 1.0],
            position: [-1.0, 0.5, 3.0],
            orientation: (PI / 6.0, PI / 4.0),
        },
        Material::new(
            0.2,
            0.2,
            0.6,
            0.85,
            Colour {
                red: 0.0,
                green: 1.0,
                blue: 0.5,
            },
        ),
    );
    let light = Light::new([-3.0, 20.0, 1.0]);
    let scene = Scene::new(vec![sphere, triangle], vec![light]);
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
