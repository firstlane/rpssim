//! Shows how to render simple primitive shapes with a single color.
//! Renders a 2D scene containing a single, moving sprite.

use bevy::sprite::collide_aabb::collide;
use bevy::window::{WindowCreated, WindowResized};
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{CursorGrabMode, PresentMode},
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

#[derive(Resource)]
struct WindowSize {
    x: f32,
    y: f32,
}

#[derive(Component)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

// TODO: Enforce that this is a unit vector
#[derive(Component)]
struct Direction {
    x: f32,
    y: f32,
}

// #[derive(Component)]
// struct Pos {
//     x: f32,
//     y: f32,
// }

#[derive(Component)]
struct Velocity {
    vel: f32,
}

fn main() {
    App::new()
        .insert_resource(WindowSize { x: 100.0, y: 100.0 })
        // .add_plugins(DefaultPlugins.set(WindowPlugin {
        //     primary_window: Some(Window {
        //         title: "I am a window!".into(),
        //         resolution: (100., 100.).into(),
        //         present_mode: PresentMode::AutoVsync,
        //         // Tells wasm to resize the window according to the available canvas
        //         fit_canvas_to_parent: true,
        //         ..default()
        //     }),
        //     ..default()
        // }))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .add_system(access_window_system)
        .add_system(on_resize_system)
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

    for i in 0..1 {
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
            Velocity { vel: 100.0 },
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
fn sprite_movement(
    time: Res<Time>,
    window_size: Res<WindowSize>,
    mut sprite_position: Query<(&mut Direction, &mut Transform, &mut Velocity)>,
) {
    for (mut direction, mut transform, mut velocity) in &mut sprite_position {
        transform.translation.x += velocity.vel * time.delta_seconds() + direction.x;
        transform.translation.y += velocity.vel * time.delta_seconds() + direction.y;

        println!("{:.1} x {:.1}", transform.translation.x, transform.translation.y);

        if transform.translation.x >= window_size.x || transform.translation.x <= 0. {
            direction.x -= direction.x;
        }

        if transform.translation.y >= window_size.y || transform.translation.y <= 0. {
            direction.y -= direction.y;
        }

        // if transform.translation.y > 200. {
        //     *logo = Direction::Down;
        // } else if transform.translation.y < -200. {
        //     *logo = Direction::Up;
        // }
    }

    // let ship_size = Vec2::new(80.0, 80.0);
    // let comet_size = Vec2::new(0.0, 0.0);
    // for comet in comets.iter() {
    //     for (ship, mut status) in ships.iter_mut() {
    //         if collide(ship.translation, ship_size, comet.translation, comet_size).is_some() {
    //             status.dead = true;
    //         }
    //     }
    // }
}

// fn resize_notificator(resize_event: Res<Events<WindowResized>>) {
//     let mut reader = resize_event.get_reader();
//     for e in reader.iter(&resize_event) {
//         println!("width = {} height = {}", e.width, e.height);
//     }
// }

fn access_window_system(mut windows: ResMut<Windows>, mut window_size: ResMut<WindowSize>) {
    for window in windows.iter() {
        window_size.x = window.width();
        window_size.y = window.height();
        //println!("{:.1} x {:.1}", window.width(), window.height());
        //window.set_title(String::from("Yay, I'm a window!"));
    }
}

// fn update_resolution(mut created_reader: EventReader<WindowCreated>, window_size: Res<WindowSize>) {
//     for e in created_reader.iter() {
//         let window_id = e.id;

//     }

//     // if keys.just_pressed(KeyCode::Key1) {
//     //     let res = resolution.small;
//     //     window.resolution.set(res.x, res.y);
//     // }
//     // if keys.just_pressed(KeyCode::Key2) {
//     //     let res = resolution.medium;
//     //     window.resolution.set(res.x, res.y);
//     // }
//     // if keys.just_pressed(KeyCode::Key3) {
//     //     let res = resolution.large;
//     //     window.resolution.set(res.x, res.y);
//     // }
// }

fn on_resize_system(
    mut resize_reader: EventReader<WindowResized>,
    mut window_size: ResMut<WindowSize>,
) {
    for e in resize_reader.iter() {
        // When resolution is being changed
        window_size.x = e.width;
        window_size.y = e.height;
        //text.sections[0].value = format!("{:.1} x {:.1}", e.width, e.height);
    }
}
