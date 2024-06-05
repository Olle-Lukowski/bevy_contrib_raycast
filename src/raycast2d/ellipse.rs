use crate::{RayCast2d, RayIntersection2d};

use bevy::math::{primitives::Ellipse, Dir2, Ray2d, Vec2};

impl RayCast2d for Ellipse {
    fn cast_ray_local(&self, ray: Ray2d, max_distance: f32) -> Option<RayIntersection2d> {
        let inv_half_size = Vec2::ONE / self.half_size;

        let origin = ray.origin * inv_half_size;
        let direction = *ray.direction * inv_half_size;

        if origin.length_squared() < 1.0 {
            return Some(RayIntersection2d {
                normal: -ray.direction,
                position: ray.origin,
                distance: 0.0,
            });
        }

        let a = direction.length_squared();
        let b = 2.0 * origin.dot(direction);
        let c = origin.length_squared() - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt = discriminant.sqrt();
        let t = (-b - sqrt) / (2.0 * a);
        if t < 0.0 || t > max_distance {
            return None;
        }

        let position = ray.origin + t * *ray.direction;

        let local_normal = position * inv_half_size;
        let normal = Dir2::new(local_normal).unwrap();

        Some(RayIntersection2d {
            normal,
            position,
            distance: t,
        })
    }
}
