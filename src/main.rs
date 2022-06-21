//! Anything-Goes Bevy Playground

use bevy::{math::const_vec3, prelude::*};

const MOON_STARTING_POSITION: Vec3 = const_vec3!([0.0, 150.0, 1.0]);
const MOON_SIZE: Vec3 = const_vec3!([30.0, 30.0, 0.0]);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(SystemSet::new().with_system(update_position))
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Moon;

// Add the game's entities to our world
fn setup(mut commands: Commands) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Moon
    commands.spawn().insert(Moon).insert_bundle(SpriteBundle {
        transform: Transform {
            scale: MOON_SIZE,
            translation: MOON_STARTING_POSITION,
            ..default()
        },
        sprite: Sprite { ..default() },
        ..default()
    });
}

// NOTES:
//  * Without `With<Moon>` -- no complaints, no errors, no odd output, no warnings, just: moon does not move.
fn update_position(mut query: Query<&mut Transform, With<Moon>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.translation.x = (time.seconds_since_startup() as f32).sin() * 150.;
        transform.translation.y = (time.seconds_since_startup() as f32).cos() * 150.;
    }
}

fn update_velocity(mut moon_query: Query<(&mut Velocity, &Transform), With<Moon>>) {
    let (mut velocity, transform) = moon_query.single_mut();
    //force_vector = transform.translation.x, transform.translation.y;
    let force_magnitude = transform.translation.x.powf(2.) + transform.translation.y.powf(2.);
}

fn get_distance(position: &Vec<f32>) -> f32 {
    ((position[0] as f32).powf(2.) + (position[1] as f32).powf(2.)).powf(0.5)
}

fn get_force(position: &Vec<f32>) -> f32 {
    let distance = get_distance(position);
    1. / distance.powf(2.)
}

fn get_displacement(position: &Vec<f32>) -> Vec<f32> {
    let force = get_force(&position);
    let x_component = position[0] / (position[0] + position[1]);
    let y_component = position[1] / (position[0] + position[1]);
    vec![x_component * force, y_component * force]
}
