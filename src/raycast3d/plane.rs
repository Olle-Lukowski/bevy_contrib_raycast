use crate::{RayCast3d, RayIntersection3d};

use bevy::math::{primitives::{Plane3d, InfinitePlane3d}, Ray3d};

impl RayCast3d for Plane3d {
    fn cast_ray_local(&self, ray: Ray3d, max_distance: f32) -> Option<RayIntersection3d> {
        let intersection = InfinitePlane3d::new(self.normal).cast_ray_local(ray, max_distance)?;

        // check if we are within self.half_size
        if intersection.position.x.abs() > self.half_size.x || intersection.position.y.abs() > self.half_size.y {
            return None;
        }

        Some(intersection)
    }
}

impl RayCast3d for InfinitePlane3d {
    fn cast_ray_local(&self, ray: Ray3d, max_distance: f32) -> Option<RayIntersection3d> {
        let denominator = self.normal.dot(*ray.direction);

        if denominator.abs() < f32::EPSILON {
            return None;
        }

        let t = self.normal.dot(-ray.origin) / denominator;

        if t < 0.0 || t > max_distance {
            return None;
        }

        // check if we are within self.half_size
        let intersection = ray.origin + *ray.direction * t;

        Some(RayIntersection3d {
            normal: self.normal,
            position: intersection,
            distance: t,
        })
    }
}
