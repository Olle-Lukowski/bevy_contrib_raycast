use crate::{RayCast3d, RayIntersection3d};

use bevy::math::{primitives::Sphere, Dir3, Ray3d};

impl RayCast3d for Sphere {
    fn cast_ray_local(&self, ray: Ray3d, max_distance: f32) -> Option<RayIntersection3d> {
        let oc = ray.origin;

        let length_squared = oc.length_squared();

        // Check if the ray origin is inside the circle
        if length_squared < self.radius * self.radius {
            return Some(RayIntersection3d {
                normal: -ray.direction,
                position: ray.origin,
                distance: 0.0,
            });
        }

        let a = ray.direction.length_squared();
        let b = 2.0 * oc.dot(*ray.direction);
        let c = length_squared - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            None
        } else {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t > 0.0 && t < max_distance {
                let intersection = ray.origin + t * *ray.direction;
                Some(RayIntersection3d {
                    // TODO: Check if unwrapping is fine
                    normal: Dir3::new(intersection).unwrap(),
                    position: intersection,
                    distance: t,
                })
            } else {
                None
            }
        }
    }
}
