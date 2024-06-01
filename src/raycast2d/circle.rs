use crate::{RayCast2d, RayIntersection2d};

use bevy::math::{primitives::Circle, Dir2, Ray2d};

impl RayCast2d for Circle {
    fn cast_ray_local(&self, ray: Ray2d, max_distance: f32) -> Option<RayIntersection2d> {
        let oc = ray.origin;

        let a = ray.direction.length_squared();
        let b = 2.0 * oc.dot(*ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            None
        } else {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t > 0.0 && t < max_distance {
                let intersection = ray.origin + t * *ray.direction;
                Some(RayIntersection2d {
                    // TODO: Check if unwrapping is fine
                    normal: Dir2::new(intersection).unwrap(),
                    position: intersection,
                    distance: t,
                })
            } else {
                None
            }
        }
    }
}
