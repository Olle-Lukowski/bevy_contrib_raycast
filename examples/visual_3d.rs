use bevy::prelude::*;
use bevy_contrib_raycast::RayCast3d;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Shape {
    Sphere,
    Plane,
}

impl Shape {
    fn next(self) -> Self {
        match self {
            Shape::Sphere => Shape::Plane,
            Shape::Plane => Shape::Sphere,
        }
    }
}

const RED: Color = Color::linear_rgb(1.0, 0.0, 0.0);
const GREEN: Color = Color::linear_rgb(0.0, 1.0, 0.0);
const BLUE: Color = Color::linear_rgb(0.0, 0.0, 1.0);

#[derive(Component)]
struct Position(Vec3);

#[derive(Component)]
struct Rotation(Quat);

#[derive(Resource)]
struct CurrentShape(Shape);

#[derive(Resource)]
struct CurrentOrigin(Vec3);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(CurrentShape(Shape::Sphere))
        .insert_resource(CurrentOrigin(Vec3::ZERO))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn((Position(Vec3::ONE), Rotation(Quat::IDENTITY)));

    // Text for keybinds
    commands.spawn(TextBundle::from_section(
        "WASD to move shape,\nARROWS to move ray origin,\nQE to move up and down,\nSPACE to cycle shapes\nIJKL to rotate",
        TextStyle {
            font_size: 30.0,
            color: Color::WHITE,
            ..default()
        },
    ));
}

fn draw_normal(
    gizmos: &mut Gizmos,
    shape: &dyn RayCast3d,
    position: Vec3,
    rotation: Quat,
    origin: Vec3,
) {
    if let Some(hit) = shape.cast_ray(position, rotation, Ray3d::new(origin, Vec3::Y), f32::MAX) {
        gizmos.arrow(hit.position, hit.position + *hit.normal * 5.0, BLUE);
    }
}

fn update(
    keys: Res<ButtonInput<KeyCode>>,
    mut shapes: Query<(&mut Position, &mut Rotation)>,
    mut current_shape: ResMut<CurrentShape>,
    mut current_origin: ResMut<CurrentOrigin>,
    mut gizmos: Gizmos,
    time: Res<Time>,
) {
    if keys.just_pressed(KeyCode::Space) {
        current_shape.0 = current_shape.0.next();
    }

    let (mut position, mut rotation) = shapes.single_mut();

    let mut movement = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        movement.z += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        movement.z -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        movement.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        movement.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyQ) {
        movement.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyE) {
        movement.y -= 1.0;
    }

    position.0 += movement * time.delta_seconds();

    let mut rotate = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyI) {
        rotate.y += 0.5;
    }
    if keys.pressed(KeyCode::KeyK) {
        rotate.y -= 0.5;
    }
    if keys.pressed(KeyCode::KeyJ) {
        rotate.x -= 0.5;
    }
    if keys.pressed(KeyCode::KeyL) {
        rotate.x += 0.5;
    }

    rotation.0 = Quat::from_rotation_x(rotate.x * time.delta_seconds()) * rotation.0;
    rotation.0 = Quat::from_rotation_y(rotate.y * time.delta_seconds()) * rotation.0;

    let mut origin_movement = Vec3::ZERO;
    if keys.pressed(KeyCode::ArrowUp) {
        origin_movement.z += 1.0;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        origin_movement.z -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowLeft) {
        origin_movement.x -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        origin_movement.x += 1.0;
    }

    current_origin.0 += origin_movement * time.delta_seconds();

    gizmos.arrow(current_origin.0, current_origin.0 + Vec3::Y * 10.0, GREEN);

    match current_shape.0 {
        Shape::Sphere => {
            let sphere = Sphere { radius: 0.5 };
            gizmos.primitive_3d(&sphere, position.0, rotation.0, RED);

            draw_normal(
                &mut gizmos,
                &sphere,
                position.0,
                rotation.0,
                current_origin.0,
            );
        }
        Shape::Plane => {
            let plane = Plane3d {
                normal: Dir3::new_unchecked(Vec3::Y),
                half_size: Vec2::ONE * 5.0,
            };
            gizmos.primitive_3d(&plane, position.0, rotation.0, RED);

            draw_normal(
                &mut gizmos,
                &plane,
                position.0,
                rotation.0,
                current_origin.0,
            );
        }
    }
}
