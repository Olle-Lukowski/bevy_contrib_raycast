use crate::{RayCast2d, RayIntersection2d};

use bevy::math::{
    primitives::{Segment2d, Triangle2d},
    Dir2, Ray2d,
};

impl RayCast2d for Triangle2d {
    fn cast_ray_local(&self, ray: Ray2d, max_distance: f32) -> Option<RayIntersection2d> {
        // first, we check if the ray starts inside the triangle.
        let a = self.vertices[0];
        let b = self.vertices[1];
        let c = self.vertices[2];

        let v0 = c - a;
        let v1 = b - a;
        let v2 = ray.origin - a;

        let dot00 = v0.dot(v0);
        let dot01 = v0.dot(v1);
        let dot02 = v0.dot(v2);
        let dot11 = v1.dot(v1);
        let dot12 = v1.dot(v2);

        let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
        let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
        let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

        if u >= 0.0 && v >= 0.0 && u + v < 1.0 {
            return Some(RayIntersection2d {
                normal: -ray.direction,
                position: ray.origin,
                distance: 0.0,
            });
        }

        let mut closest_intersection: Option<RayIntersection2d> = None;

        let edges = [(a, b), (b, c), (c, a)];

        for (start, end) in edges {
            let segment = Segment2d::new(Dir2::new(end - start).unwrap(), start.distance(end));

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
