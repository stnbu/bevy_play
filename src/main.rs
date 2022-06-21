//! Anything-Goes Bevy Playground

use bevy::{math::const_vec3, prelude::*};

const BALL_STARTING_POSITION: Vec3 = const_vec3!([0.0, 150.0, 1.0]);
const BALL_SIZE: Vec3 = const_vec3!([30.0, 30.0, 0.0]);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(SystemSet::new().with_system(update_position))
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

#[derive(Component)]
struct Ball;

// Add the game's entities to our world
fn setup(mut commands: Commands) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Ball
    commands.spawn().insert(Ball).insert_bundle(SpriteBundle {
        transform: Transform {
            scale: BALL_SIZE,
            translation: BALL_STARTING_POSITION,
            ..default()
        },
        sprite: Sprite { ..default() },
        ..default()
    });
}

// NOTES:
//  * Without `With<Ball>` -- no complaints, no errors, no odd output, no warnings, just: ball does not move.
fn update_position(mut query: Query<&mut Transform, With<Ball>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.translation.x = (time.seconds_since_startup() as f32).sin() * 150.;
        transform.translation.y = (time.seconds_since_startup() as f32).cos() * 150.;
    }
}
