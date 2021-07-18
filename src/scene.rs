use crate::image::{Pixel, BLACK, WHITE};
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
    pub fn trace(&self, ray: &Ray, depth: u8) -> Pixel {
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
    pub fn get_colour(&self, object: &Object, direction: &HVector, hit: &Hit, depth: u8) -> Pixel {
        //let reflection = if depth == 0 {
        //    BLACK
        //} else {
        //    BLACK // TODO: recurse
        //};
        //let refraction = BLACK; // TODO
        const IA: Pixel = WHITE;
        let material = object.get_material();

        // ambient
        let ambient_light = IA.scale(material.ambient);

        let incident_reversed = direction.reverse();
        let mut light_contributions: Pixel = BLACK;
        for light in self.lights.iter() {
            let light_colour = light.get_colour();
            //diffuse
            let light_direction = light.direction_from(&hit.normal.from);
            let diffuse_factor = material.diffuse * light_direction.dot(&hit.normal.direction);
            //specular
            let diffuse_contribution: Pixel = light_colour.scale(diffuse_factor);
            let specular_factor = material.specular;
            let specular_contribution: Pixel = light_colour.scale(specular_factor);
            light_contributions += specular_contribution + diffuse_contribution;
        }
        ambient_light + light_contributions.scale(1.0 / self.lights.len() as f64)
    }
}
