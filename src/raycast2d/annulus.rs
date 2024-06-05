use crate::{RayCast2d, RayIntersection2d};

use bevy::math::{primitives::Annulus, Dir2, Ray2d};

impl RayCast2d for Annulus {
    fn cast_ray_local(&self, ray: Ray2d, max_distance: f32) -> Option<RayIntersection2d> {
        let oc = ray.origin;
        let length_squared = oc.length_squared();

        if length_squared < self.inner_circle.radius * self.inner_circle.radius {
            // we are in the inner circle, find the intersection point with the border of it
            // based on ray origin and direction
            let a = ray.direction.length_squared();
            let b = 2.0 * oc.dot(*ray.direction);
            let c = length_squared - self.inner_circle.radius * self.inner_circle.radius;
            let discriminant = b * b - 4.0 * a * c;
            let mut t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t > 0.0 && t < max_distance {
                let intersection = ray.origin + t * *ray.direction;
                return Some(RayIntersection2d {
                    normal: Dir2::new(-intersection).unwrap(),
                    position: intersection,
                    distance: t,
                });
            }

            // try the second t
            t = (-b + discriminant.sqrt()) / (2.0 * a);
            if t > 0.0 && t < max_distance {
                let intersection = ray.origin + t * *ray.direction;
                return Some(RayIntersection2d {
                    normal: Dir2::new(-intersection).unwrap(),
                    position: intersection,
                    distance: t,
                });
            }
        }

        self.outer_circle.cast_ray_local(ray, max_distance)
    }
}
