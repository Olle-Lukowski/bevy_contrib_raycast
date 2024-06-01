use crate::{RayCast2d, RayIntersection2d};

use bevy::math::{primitives::Arc2d, Dir2, Ray2d, Vec2};

impl RayCast2d for Arc2d {
    fn cast_ray_local(&self, ray: Ray2d, max_distance: f32) -> Option<RayIntersection2d> {
        let oc = ray.origin;

        let a = ray.direction.length_squared();
        let b = 2.0 * ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        if t1 < 0.0 && t2 < 0.0 {
            return None;
        }

        if t1 > max_distance && t2 > max_distance {
            return None;
        }

        let p1 = ray.direction * t1 + ray.origin;

        if p1.angle_between(Vec2::Y).abs() < self.half_angle {
            return Some(RayIntersection2d {
                distance: t1,
                normal: Dir2::new(p1).unwrap(),
                position: p1,
            });
        }

        let p2 = ray.direction * t2 + ray.origin;
        if p2.angle_between(Vec2::Y).abs() < self.half_angle {
            return Some(RayIntersection2d {
                distance: t2,
                normal: Dir2::new(p2).unwrap(),
                position: p2,
            });
        }

        None
    }
}
