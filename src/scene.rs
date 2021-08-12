use crate::image::{Colour, BLACK};
use crate::light::Light;
use crate::object::Object;
use crate::ray::{Hit, Ray};
use crate::vector::HVector;

pub struct Scene {
    objects: Vec<Object>,
    lights: Vec<Light>,
}
impl Scene {
    pub fn new(objects: Vec<Object>, lights: Vec<Light>) -> Scene {
        Scene { objects, lights }
    }
    pub fn trace(&self, ray: &Ray, depth: u8) -> Colour {
        let mut best: Option<(f64, &Object, Hit)> = None;
        for object in self.objects.iter() {
            match object.intersect(ray) {
                Some(hit) => {
                    let distance = (hit.normal.from.clone() - ray.from.clone()).magnitude_squared();
                    best = match best {
                        None => Some((distance, object, hit)),
                        Some((d, o, h)) => {
                            if d > distance {
                                Some((distance, object, hit))
                            } else {
                                Some((d, o, h))
                            }
                        }
                    };
                }
                None => {}
            }
        }
        match best {
            Some((_, object, hit)) => self.get_colour(object, &ray.direction, &hit, depth),
            None => BLACK,
        }
    }
    pub fn get_colour(&self, object: &Object, direction: &HVector, hit: &Hit, depth: u8) -> Colour {
        //let reflection = if depth == 0 {
        //    BLACK
        //} else {
        //    BLACK // TODO: recurse
        //};
        //let refraction = BLACK; // TODO
        let material = object.get_material();

        // ambient
        let ambient_light = material.colour.scale(material.ambient);

        let incident_reversed = direction.reverse();
        let mut light_contributions: Colour = BLACK;
        for light in self.lights.iter() {
            // diffuse
            let light_direction = light.direction_from(&hit.normal.from);
            let diffuse_factor = light_direction.dot(&hit.normal.direction);
            if diffuse_factor < 0.0 {
                continue;
            }
            light_contributions += material.colour.scale(material.diffuse * diffuse_factor);
            // specular
            let reflected_light = light_direction.reflect(&hit.normal.direction);
            let specular_factor = incident_reversed.dot(&reflected_light);
            if specular_factor < 0.0 {
                continue;
            }
            light_contributions += light.colour.scale(material.specular * specular_factor);
        }
        ambient_light + light_contributions.scale(1.0 / self.lights.len() as f64)
    }
}
