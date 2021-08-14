use crate::{
    ray::{Hit, Ray},
    scene::object::Intersectable,
};

pub fn find_closest_intersection<'a, O>(
    objects: impl IntoIterator<Item = &'a O>,
    ray: &Ray,
) -> Option<Hit<'a>>
where
    O: 'a + Intersectable,
{
    let mut best = None;
    for object in objects {
        best = object.intersect(ray).and_then(|hit| {
            let distance = (hit.normal.from.clone() - ray.from.clone()).magnitude_squared();
            match best {
                None => Some((distance, object, hit)), // first hit, update best
                Some((d, o, h)) => {
                    if d > distance {
                        Some((distance, object, hit)) // closer hit, update best
                    } else {
                        Some((d, o, h)) // further hit, preserve previous best
                    }
                }
            }
        });
    }
    best.and_then(|(_, _, hit)| Some(hit))
}
