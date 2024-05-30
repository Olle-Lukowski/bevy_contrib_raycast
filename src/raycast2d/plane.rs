use crate::{RayCast2d, RayIntersection2d};

use bevy::math::{primitives::Plane2d, Ray2d};

impl RayCast2d for Plane2d {
    fn cast_ray_local(&self, ray: Ray2d, max_distance: f32) -> Option<RayIntersection2d> {
        let normal_dot_direction = self.normal.dot(*ray.direction);

        // Check if the ray is parallel to the plane
        if normal_dot_direction.abs() < f32::EPSILON {
            return None;
        }

        let t = -self.normal.dot(ray.origin) / normal_dot_direction;

        if t < 0.0 || t > max_distance {
            return None;
        }

        Some(RayIntersection2d {
            normal: self.normal,
            position: ray.origin + *ray.direction * t,
            distance: t,
        })
    }
}
