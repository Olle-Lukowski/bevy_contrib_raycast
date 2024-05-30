use crate::{RayCast2d, RayIntersection2d};

use bevy::math::{
    primitives::{Direction2d, Rectangle, Segment2d},
    Ray2d, Vec2,
};

impl RayCast2d for Rectangle {
    fn cast_ray_local(&self, ray: Ray2d, max_distance: f32) -> Option<RayIntersection2d> {
        let mut closest_intersection: Option<RayIntersection2d> = None;

        let bottom_left = -self.half_size;
        let bottom_right = Vec2::new(self.half_size.x, -self.half_size.y);
        let top_left = Vec2::new(-self.half_size.x, self.half_size.y);
        let top_right = self.half_size;

        let edges = [
            (bottom_left, bottom_right),
            (bottom_right, top_right),
            (top_right, top_left),
            (top_left, bottom_left),
        ];

        for (start, end) in edges {
            let segment =
                Segment2d::new(Direction2d::new(end - start).unwrap(), start.distance(end));

            if let Some(intersection) =
                segment.cast_ray((start + end) / 2.0, 0.0, ray, max_distance)
            {
                if let Some(ref closest) = closest_intersection {
                    if intersection.distance < closest.distance {
                        closest_intersection = Some(intersection);
                    }
                } else {
                    closest_intersection = Some(intersection);
                }
            }
        }

        closest_intersection
    }
}