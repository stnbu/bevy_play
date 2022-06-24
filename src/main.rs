//! Anything-Goes Bevy Playground

use bevy::{
    core::FixedTimestep,
    math::{const_vec2, const_vec3},
    prelude::*,
};

const TIME_STEP: f32 = 1.0 / 60.0;
const TURTLE_STARTING_POSITION: Vec3 = const_vec3!([0.0, 150.0, 1.0]);
const TURTLE_SIZE: Vec3 = const_vec3!([30.0, 30.0, 0.0]);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update_velocity)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(apply_velocity),
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

const TURTLE_SPEED: f32 = 400.0;
const INITIAL_TURTLE_DIRECTION: Vec2 = const_vec2!([0.5, -0.5]);

// Could be a component; but there is only one
#[derive(Default)]
struct Velocity(Vec2);

#[derive(Component)]
struct Turtle;

// Add the game's entities to our world
fn setup(mut commands: Commands) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Turtle
    commands.spawn().insert(Turtle).insert_bundle(SpriteBundle {
        transform: Transform {
            scale: TURTLE_SIZE,
            translation: TURTLE_STARTING_POSITION,
            ..default()
        },
        sprite: Sprite { ..default() },
        ..default()
    });
}

fn apply_velocity(mut query: Query<&mut Transform>, velocity: ResMut<Velocity>) {
    for mut transform in query.iter_mut() {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}

fn update_velocity(keyboard_input: Res<Input<KeyCode>>, mut velocity: ResMut<Velocity>) {
    if keyboard_input.just_pressed(KeyCode::Up) {
        velocity.y += 1.;
    }
    if keyboard_input.just_pressed(KeyCode::Down) {
        velocity.y -= 1.;
    }
    if keyboard_input.just_pressed(KeyCode::Left) {
        velocity.x -= 1.;
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        velocity.x += 1.;
    }
}
