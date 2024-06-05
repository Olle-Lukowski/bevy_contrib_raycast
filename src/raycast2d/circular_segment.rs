use crate::{RayCast2d, RayIntersection2d};

use bevy::math::{
    primitives::{CircularSegment, Segment2d},
    Dir2, Mat2, Ray2d, Vec2,
};

impl RayCast2d for CircularSegment {
    fn cast_ray_local(&self, ray: Ray2d, max_distance: f32) -> Option<RayIntersection2d> {
        // first, check if we are inside the segment

        let start = self.arc.radius * Mat2::from_angle(self.arc.half_angle) * Vec2::Y;
        let end = self.arc.radius * Mat2::from_angle(-self.arc.half_angle) * Vec2::Y;

        let oc = ray.origin;
        let length_squared = oc.length_squared();
        if length_squared < self.arc.radius * self.arc.radius {
            // we also need to check if the origin is above the segment
            if Vec2::Y.angle_between(oc).abs() < self.arc.half_angle && oc.y >= start.y.min(end.y) {
                return Some(RayIntersection2d {
                    normal: -ray.direction,
                    position: ray.origin,
                    distance: 0.0,
                });
            }
        }

        let mut closest = None;
        if let Some(intersection) = self.arc.cast_ray_local(ray, max_distance) {
            closest = Some(intersection);
        }

        let segment = Segment2d::new(Dir2::new(end - start).unwrap(), self.arc.radius * 2.0);

        // Check if the segment connecting the two ends of the arc is intersecting the ray
        if let Some(intersection) = segment.cast_ray((start + end) / 2.0, 0.0, ray, max_distance) {
            if closest.is_none() || intersection.distance < closest.unwrap().distance {
                closest = Some(intersection);
            }
        }

        closest
    }
}
