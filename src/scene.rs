pub mod light;
pub mod object;
use self::{
    light::Light,
    object::{intersection::find_closest_intersection, Object},
};
use crate::{
    image::Colour,
    ray::{Hit, Ray},
    vector::HVector,
};

pub struct Scene {
    objects: Vec<Object>,
    lights: Vec<Light>,
}

impl Scene {
    pub fn new(objects: Vec<Object>, lights: Vec<Light>) -> Scene {
        Scene { objects, lights }
    }
    pub fn trace(&self, ray: &Ray, depth: u8) -> Colour {
        match find_closest_intersection(&self.objects, ray) {
            Some(hit) => self.get_colour(&ray.direction, &hit, depth),
            None => Colour::BLACK,
        }
    }
    pub fn get_colour(&self, direction: &HVector, hit: &Hit, depth: u8) -> Colour {
        //let reflection = if depth == 0 {
        //    Colour::BLACK
        //} else {
        //    Colour::BLACK // TODO: recurse
        //};
        //let refraction = Colour::BLACK; // TODO
        let material = hit.material.unwrap(); // PANIC if hit has no material

        // ambient
        let ambient_light = material.colour.scale(material.ambient);

        let incident_reversed = direction.reverse();
        let mut light_contributions = Colour::BLACK;
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
