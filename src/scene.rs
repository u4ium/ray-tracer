use crate::image::{Pixel, BLACK};
use crate::object::Object;
use crate::ray::{Hit, Ray};

pub struct Scene {
    objects: Vec<Object>,
}
impl Scene {
    pub fn new() -> Scene {
        Scene { objects: vec![] }
    }
    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }
    pub fn trace(&self, ray: &Ray, depth: u8) -> Pixel {
        let mut best: Option<(f64, &Object, Hit)> = None;
        for object in self.objects.iter() {
            match object.intersect(ray) {
                Some(hit) => {
                    let distance = (hit.normal.from.clone() - ray.from.clone()).magnitude();
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
            Some((_, object, hit)) => object.get_colour(&ray.direction, &hit), // TODO: recurse
            None => BLACK,
        }
    }
}
