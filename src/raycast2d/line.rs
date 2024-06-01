use crate::{RayCast2d, RayIntersection2d};

use bevy::math::{primitives::Line2d, Dir2, Ray2d};

impl RayCast2d for Line2d {
    fn cast_ray_local(&self, ray: Ray2d, max_distance: f32) -> Option<RayIntersection2d> {
        let perpendicular = -self.direction.perp();
        let denominator = perpendicular.dot(*ray.direction);

        if denominator.abs() < f32::EPSILON {
            return None;
        }

        let t = perpendicular.dot(-ray.origin) / denominator;

        if t < 0.0 || t > max_distance {
            return None;
        }

        Some(RayIntersection2d {
            normal: Dir2::new(perpendicular).unwrap(),
            position: ray.origin + *ray.direction * t,
            distance: t,
        })
    }
}
