use crate::{RayCast2d, RayIntersection2d};

use bevy::math::{
    primitives::{Direction2d, Segment2d, Triangle2d},
    Ray2d,
};

impl RayCast2d for Triangle2d {
    fn cast_ray_local(&self, ray: Ray2d, max_distance: f32) -> Option<RayIntersection2d> {
        let mut closest_intersection: Option<RayIntersection2d> = None;

        let a = self.vertices[0];
        let b = self.vertices[1];
        let c = self.vertices[2];

        let edges = [(a, b), (b, c), (c, a)];

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
