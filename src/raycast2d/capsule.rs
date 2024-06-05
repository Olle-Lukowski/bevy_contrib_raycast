use crate::{RayCast2d, RayIntersection2d};

use bevy::math::{
    primitives::{Capsule2d, Circle, Rectangle},
    Ray2d, Vec2,
};

impl RayCast2d for Capsule2d {
    fn cast_ray_local(&self, ray: Ray2d, max_distance: f32) -> Option<RayIntersection2d> {


        // check intersection with the two half-circles, and the the rectangle
        let bottom_circle = Circle::new(self.radius);
        let top_circle = Circle::new(self.radius);
        let rectangle = Rectangle::new(self.radius * 2.0, self.half_length * 2.0);
            
        let mut closest_intersection: Option<RayIntersection2d> = None;

        let shapes: [(&dyn RayCast2d, Vec2); 3] = [
            (&bottom_circle, Vec2::new(0.0, self.half_length)),
            (&top_circle, Vec2::new(0.0, -self.half_length)),
            (&rectangle, Vec2::new(0.0, 0.0)),
        ];

        for (shape, pos) in shapes {
            if let Some(intersection) = shape.cast_ray(pos, 0.0, ray, max_distance) {
                if intersection.distance == 0.0 {
                    return Some(intersection);
                }
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
