use crate::{RayCast2d, RayIntersection2d};

use bevy::math::{
    primitives::{Rhombus, Segment2d},
    Dir2, Ray2d, Vec2,
};

impl RayCast2d for Rhombus {
    fn cast_ray_local(&self, ray: Ray2d, max_distance: f32) -> Option<RayIntersection2d> {
        // first, we check if the ray starts inside the rhombus.

        if ray.origin.x.abs() / self.half_diagonals.x + ray.origin.y.abs() / self.half_diagonals.y <= 1.0 {
            return Some(RayIntersection2d {
                normal: -ray.direction,
                position: ray.origin,
                distance: 0.0,
            });
        }


        let mut closest: Option<RayIntersection2d> = None;

        let top = Vec2::new(0.0, self.half_diagonals.y);
        let bottom = Vec2::new(0.0, -self.half_diagonals.y);
        let left = Vec2::new(-self.half_diagonals.x, 0.0);
        let right = Vec2::new(self.half_diagonals.x, 0.0);

        let edges = [(bottom, right), (right, top), (top, left), (left, bottom)];

        for (start, end) in edges {
            let segment = Segment2d::new(Dir2::new(end - start).unwrap(), start.distance(end));

            if let Some(intersection) =
                segment.cast_ray((start + end) / 2.0, 0.0, ray, max_distance)
            {
                if closest.is_none() || intersection.distance < closest.unwrap().distance {
                    closest = Some(intersection);
                }
            }
        }

        closest
    }
}
