use crate::{RayCast2d, RayIntersection2d};

use bevy::math::{primitives::Annulus, Ray2d};

impl RayCast2d for Annulus {
    fn cast_ray_local(&self, ray: Ray2d, max_distance: f32) -> Option<RayIntersection2d> {
        let mut closest = None;

        if let Some(intersection) = self.inner_circle.cast_ray_local(ray, max_distance) {
            closest = Some(intersection);
        }

        if let Some(intersection) = self.outer_circle.cast_ray_local(ray, max_distance) {
            if closest.is_none() || intersection.distance < closest.unwrap().distance {
                closest = Some(intersection);
            }
        }

        closest
    }
}
