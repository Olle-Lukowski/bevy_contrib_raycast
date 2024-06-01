use crate::{RayCast2d, RayIntersection2d};

use bevy::math::{
    primitives::{CircularSegment, Segment2d},
    Dir2, Mat2, Ray2d, Vec2,
};

impl RayCast2d for CircularSegment {
    fn cast_ray_local(&self, ray: Ray2d, max_distance: f32) -> Option<RayIntersection2d> {
        let mut closest = None;
        if let Some(intersection) = self.arc.cast_ray_local(ray, max_distance) {
            closest = Some(intersection);
        }

        // Check if the segment connecting the two ends of the arc is intersecting the ray

        let start = self.arc.radius * Mat2::from_angle(self.arc.half_angle) * Vec2::Y;
        let end = self.arc.radius * Mat2::from_angle(-self.arc.half_angle) * Vec2::Y;

        let segment = Segment2d::new(Dir2::new(end - start).unwrap(), self.arc.radius);
        if let Some(intersection) = segment.cast_ray((start + end) / 2.0, 0.0, ray, max_distance) {
            if closest.is_none() || intersection.distance < closest.unwrap().distance {
                closest = Some(intersection);
            }
        }

        closest
    }
}
