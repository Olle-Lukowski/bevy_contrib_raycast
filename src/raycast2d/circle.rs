use crate::{RayCast2d, RayIntersection2d};

use bevy::math::{primitives::Circle, Ray2d};

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
                Some(RayIntersection2d {
                    normal: (ray.origin + t * *ray.direction).normalize(),
                    distance: t,
                })
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::Vec2;

    use super::*;

    #[test]
    fn test_circle() {
        let circle = Circle::new(1.0);
        let ray = Ray2d::new(Vec2::ZERO, Vec2::X);
        let intersection = circle.cast_ray(Vec2::new(3.0, 0.8), 0.0, ray, 100.0);
        assert!(intersection.is_some());
    }
}
