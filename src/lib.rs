use bevy::math::{
    primitives::{Primitive2d, Primitive3d},
    Dir2, Dir3, Mat2, Quat, Ray2d, Ray3d, Vec2, Vec3,
};

/// The implementation for [RayCast2d] for many types.
pub mod raycast2d;

/// The implementation for [RayCast3d] for many types.
pub mod raycast3d;

/// The default output used for [RayCast3d]
#[derive(Debug, Clone, Copy)]
pub struct RayIntersection3d {
    pub normal: Dir3,
    pub position: Vec3,
    pub distance: f32,
}

/// The default output used for [RayCast2d]
#[derive(Debug, Clone, Copy)]
pub struct RayIntersection2d {
    pub normal: Dir2,
    pub position: Vec2,
    pub distance: f32,
}

/// An extension trait for bevy's primitive shapes, to provide raycasting functionality.
pub trait RayCast3d: Primitive3d {
    fn cast_ray_local(&self, ray: Ray3d, max_distance: f32) -> Option<RayIntersection3d>;

    fn cast_ray(
        &self,
        position: Vec3,
        rotation: Quat,
        ray: Ray3d,
        max_distance: f32,
    ) -> Option<RayIntersection3d> {
        let inv_rotation = rotation.inverse();
        let local_origin = inv_rotation * (ray.origin - position);
        let local_direction = inv_rotation * *ray.direction;
        let local_ray = Ray3d::new(local_origin, local_direction);

        if let Some(mut intersection) = self.cast_ray_local(local_ray, max_distance) {
            intersection.position = rotation * intersection.position + position;
            intersection.normal = Dir3::new(rotation * *intersection.normal).unwrap();
            Some(intersection)
        } else {
            None
        }
    }
}

/// An extension trait for bevy's primitive shapes, to provide raycasting functionality.
pub trait RayCast2d: Primitive2d {
    fn cast_ray_local(&self, ray: Ray2d, max_distance: f32) -> Option<RayIntersection2d>;

    fn cast_ray(
        &self,
        position: Vec2,
        angle: f32,
        ray: Ray2d,
        max_distance: f32,
    ) -> Option<RayIntersection2d> {
        let inv_rotation = Mat2::from_angle(-angle);

        let local_origin = inv_rotation * (ray.origin - position);
        let local_direction = inv_rotation * *ray.direction;
        let local_ray = Ray2d::new(local_origin, local_direction);

        if let Some(mut intersection) = self.cast_ray_local(local_ray, max_distance) {
            let rotation = Mat2::from_angle(angle);
            intersection.normal = Dir2::new(rotation * *intersection.normal).unwrap();
            intersection.position = rotation * intersection.position + position;
            Some(intersection)
        } else {
            None
        }
    }
}
