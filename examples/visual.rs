use bevy::prelude::*;
use bevy_contrib_raycast::RayCast2d;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Shape {
    Circle,
    Ellipse,
    Plane,
    Line,
    Segment,
    Polyline,
    Triangle,
    Rectangle,
    Polygon,
    RegularPolygon,
    Capsule,
}

impl Shape {
    fn next(self) -> Self {
        match self {
            Shape::Circle => Shape::Ellipse,
            Shape::Ellipse => Shape::Plane,
            Shape::Plane => Shape::Line,
            Shape::Line => Shape::Segment,
            Shape::Segment => Shape::Polyline,
            Shape::Polyline => Shape::Triangle,
            Shape::Triangle => Shape::Rectangle,
            Shape::Rectangle => Shape::Polygon,
            Shape::Polygon => Shape::RegularPolygon,
            Shape::RegularPolygon => Shape::Capsule,
            Shape::Capsule => Shape::Circle,
        }
    }
}

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Rotation(f32);

#[derive(Resource)]
struct CurrentShape(Shape);

#[derive(Resource)]
struct CurrentOrigin(Vec2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(CurrentShape(Shape::Circle))
        .insert_resource(CurrentOrigin(Vec2::ZERO))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((Position(Vec2::ONE), Rotation(0.0)));

    // Text for keybinds
    commands.spawn(TextBundle::from_section(
        "WASD to move shape, ARROWS to move ray origin, QE to rotate, SPACE to cycle shapes",
        TextStyle {
            font_size: 30.0,
            color: Color::WHITE,
            ..default()
        },
    ));
}

fn draw_normal(
    gizmos: &mut Gizmos,
    shape: &dyn RayCast2d,
    position: Vec2,
    rotation: f32,
    origin: Vec2,
) {
    if let Some(hit) = shape.cast_ray(position, rotation, Ray2d::new(origin, Vec2::Y), f32::MAX) {
        gizmos.arrow_2d(
            hit.position,
            hit.position + *hit.normal * 250.0,
            Color::BLUE,
        );
    }
}

fn update(
    keys: Res<ButtonInput<KeyCode>>,
    mut current_shape: ResMut<CurrentShape>,
    mut current_origin: ResMut<CurrentOrigin>,
    mut shapes: Query<(&mut Position, &mut Rotation)>,
    mut gizmos: Gizmos,
) {
    if keys.just_pressed(KeyCode::Space) {
        current_shape.0 = current_shape.0.next();
    }

    let (mut position, mut rotation) = shapes.single_mut();

    let mut movement = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        movement.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        movement.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        movement.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        movement.x += 1.0;
    }

    position.0 += movement;

    let mut rotate = 0.0;
    if keys.just_pressed(KeyCode::KeyQ) {
        rotate += 0.5;
    }
    if keys.just_pressed(KeyCode::KeyE) {
        rotate -= 0.5;
    }
    rotation.0 += rotate;

    let mut origin_movement = Vec2::ZERO;
    if keys.pressed(KeyCode::ArrowUp) {
        origin_movement.y += 1.0;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        origin_movement.y -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowLeft) {
        origin_movement.x -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        origin_movement.x += 1.0;
    }
    current_origin.0 += origin_movement;

    // now we draw the ray, cast it, and draw the normal.
    gizmos.arrow_2d(
        current_origin.0,
        current_origin.0 + Vec2::Y * 25000.0,
        Color::GREEN,
    );

    match current_shape.0 {
        Shape::Circle => {
            let circle = Circle { radius: 25.0 };
            gizmos.primitive_2d(circle, position.0, rotation.0, Color::RED);

            draw_normal(
                &mut gizmos,
                &circle,
                position.0,
                rotation.0,
                current_origin.0,
            );
        }
        Shape::Ellipse => {
            let ellipse = Ellipse {
                half_size: Vec2::new(25.0, 15.0),
            };
            gizmos.primitive_2d(ellipse, position.0, rotation.0, Color::RED);

            draw_normal(
                &mut gizmos,
                &ellipse,
                position.0,
                rotation.0,
                current_origin.0,
            );
        }
        Shape::Plane => {
            let plane = Plane2d {
                normal: Direction2d::new(Vec2::Y).unwrap(),
            };
            gizmos.primitive_2d(plane, position.0, rotation.0, Color::RED);

            draw_normal(
                &mut gizmos,
                &plane,
                position.0,
                rotation.0,
                current_origin.0,
            );
        }
        Shape::Line => {
            let line = Line2d {
                direction: Direction2d::new(Vec2::Y).unwrap(),
            };
            gizmos.primitive_2d(line, position.0, rotation.0, Color::RED);

            draw_normal(&mut gizmos, &line, position.0, rotation.0, current_origin.0);
        }
        Shape::Segment => {
            let segment = Segment2d {
                direction: Direction2d::new(Vec2::Y).unwrap(),
                half_length: 25.0,
            };
            gizmos.primitive_2d(segment, position.0, rotation.0, Color::RED);

            draw_normal(
                &mut gizmos,
                &segment,
                position.0,
                rotation.0,
                current_origin.0,
            );
        }
        Shape::Polyline => {
            let polyline = Polyline2d::<3>::from_iter([
                Vec2::new(0.0, 0.0),
                Vec2::new(25.0, 0.0),
                Vec2::new(25.0, 25.0),
            ]);
            gizmos.primitive_2d(polyline.clone(), position.0, rotation.0, Color::RED);

            draw_normal(
                &mut gizmos,
                &polyline,
                position.0,
                rotation.0,
                current_origin.0,
            );
        }
        Shape::Triangle => {
            let triangle = Triangle2d::new(
                Vec2::new(0.0, 0.0),
                Vec2::new(25.0, 0.0),
                Vec2::new(25.0, 25.0),
            );
            gizmos.primitive_2d(triangle, position.0, rotation.0, Color::RED);

            draw_normal(
                &mut gizmos,
                &triangle,
                position.0,
                rotation.0,
                current_origin.0,
            );
        }
        Shape::Rectangle => {
            let rectangle = Rectangle {
                half_size: Vec2::new(25.0, 15.0),
            };
            gizmos.primitive_2d(rectangle, position.0, rotation.0, Color::RED);

            draw_normal(
                &mut gizmos,
                &rectangle,
                position.0,
                rotation.0,
                current_origin.0,
            );
        }
        Shape::Polygon => {
            let poly = Polygon::<5>::from_iter([
                Vec2::new(-12.5, 0.0),
                Vec2::new(12.5, 0.0),
                Vec2::new(25.0, 25.0),
                Vec2::new(0.0, 50.0),
                Vec2::new(-25.0, 25.0),
            ]);
            gizmos.primitive_2d(poly.clone(), position.0, rotation.0, Color::RED);

            draw_normal(&mut gizmos, &poly, position.0, rotation.0, current_origin.0);
        }
        Shape::RegularPolygon => {
            let polygon = RegularPolygon::new(25.0, 10);
            gizmos.primitive_2d(polygon, position.0, rotation.0, Color::RED);

            draw_normal(
                &mut gizmos,
                &polygon,
                position.0,
                rotation.0,
                current_origin.0,
            );
        }
        Shape::Capsule => {
            let capsule = Capsule2d::new(25.0, 100.0);
            gizmos.primitive_2d(capsule, position.0, rotation.0, Color::RED);

            draw_normal(
                &mut gizmos,
                &capsule,
                position.0,
                rotation.0,
                current_origin.0,
            );
        }
    };
}
