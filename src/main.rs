//! A simplified implementation of the classic game "Breakout"
//! Gradually morphed into something else (mburr)

use bevy::{
    core::FixedTimestep,
    math::{const_vec2, const_vec3},
    prelude::*,
};

const TIME_STEP: f32 = 0.25 / 60.0;

// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.
// The `const_vec3!` macros are needed as functions that operate on floats cannot be constant in Rust.

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const BALL_STARTING_POSITION: Vec3 = const_vec3!([0.0, -50.0, 1.0]);
const BALL_SIZE: Vec3 = const_vec3!([30.0, 30.0, 0.0]);
const BALL_SPEED: f32 = 400.0;
const INITIAL_BALL_DIRECTION: Vec2 = const_vec2!([0.5, -0.5]);

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(apply_velocity),
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

// Add the game's entities to our world
fn setup(mut commands: Commands) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Ball
    commands
        .spawn()
        .insert(Ball)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: BALL_SIZE,
                translation: BALL_STARTING_POSITION,
                ..default()
            },
            sprite: Sprite {
                color: BALL_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED));
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}
