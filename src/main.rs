//! Anything-Goes Bevy Playground

use bevy::{
    math::{const_vec2, const_vec3},
    prelude::*,
};

const TIME_STEP: f32 = 0.00001 / 60.0;
const INITIAL_MOON_DIRECTION: Vec2 = const_vec2!([0.5, -0.5]);
const MOON_STARTING_POSITION: Vec3 = const_vec3!([0.0, 150.0, 1.0]);
const MOON_SIZE: Vec3 = const_vec3!([30.0, 30.0, 0.0]);
const MOON_SPEED: f32 = 400.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_system(update_velocity)
                .with_system(update_position)
                .with_system(update_velocity.before(update_position)),
        )
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
    commands
        .spawn()
        .insert(Moon)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: MOON_SIZE,
                translation: MOON_STARTING_POSITION,
                ..default()
            },
            sprite: Sprite { ..default() },
            ..default()
        })
        .insert(Velocity(INITIAL_MOON_DIRECTION.normalize() * MOON_SPEED));
}

// NOTES:
//  * Without `With<Moon>` -- no complaints, no errors, no odd output, no warnings, just: moon does not move.
fn update_position(mut query: Query<(&mut Transform, &Velocity), With<Moon>>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}

fn update_velocity(mut moon_query: Query<(&mut Velocity, &Transform)>) {
    let (mut velocity, transform) = moon_query.single_mut();
    let delta_v = get_delta_v(&vec![
        transform.translation.x as f32,
        transform.translation.y as f32,
    ]);
    velocity.x += delta_v[0];
    velocity.y += delta_v[1];
    //println!("{:?}", delta_v);
    //println!("({}, {})", velocity.x, velocity.y)
}

fn get_distance(position: &Vec<f32>) -> f32 {
    ((position[0] as f32).powf(2.) + (position[1] as f32).powf(2.)).powf(0.5)
}

fn get_force(position: &Vec<f32>) -> f32 {
    let distance = get_distance(position);
    let f = 1. / distance.powf(2.);
    //println!("{}", &f);
    f.abs()
}

fn get_delta_v(position: &Vec<f32>) -> Vec<f32> {
    let force = get_force(position);
    let x_component = position[0] / (position[0] + position[1]);
    let y_component = position[1] / (position[0] + position[1]);
    vec![
        -x_component * force * 100000000.,
        -y_component * force * 100000000.,
    ]
}
