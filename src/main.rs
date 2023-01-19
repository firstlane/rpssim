//! Shows how to render simple primitive shapes with a single color.
//! Renders a 2D scene containing a single, moving sprite.

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy::window::{WindowResized};
use rand::Rng;

#[derive(Component)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

#[derive(Component)]
struct Direction {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Pos {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Velocity {
    vel: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Create 10 of each rock, paper, and scissors
    // with random starting position, random directions,
    // and fixed velocity.

    let mut rng = rand::thread_rng();

    for i in 0..10 {
        // Paper
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                ..default()
            },
            Throw::Paper,
            // Velocity { vel: 1.0 },
            Direction {
                x: rng.gen_range(0.0..1.0),
                y: rng.gen_range(0.0..1.0),
            },
            // Pos {
            //     x: rng.gen_range(0.0..1.0),
            //     y: rng.gen_range(0.0..1.0),
            // },
        ));

        // // Circle (scissors)
        // commands.spawn(MaterialMesh2dBundle {
        //     mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        //     material: materials.add(ColorMaterial::from(Color::PURPLE)),
        //     transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
        //     ..default()
        // });

        // // Hexagon (rock)
        // commands.spawn(MaterialMesh2dBundle {
        //     mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
        //     material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        //     transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
        //     ..default()
        // });
    }
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
        }

        if transform.translation.y > 200. {
            *logo = Direction::Down;
        } else if transform.translation.y < -200. {
            *logo = Direction::Up;
        }
    }
}

fn resize_notificator(resize_event: Res<Events<WindowResized>>) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        println!("width = {} height = {}", e.width, e.height);
    }
}
