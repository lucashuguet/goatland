use bevy::{input::mouse::MouseMotion, prelude::*};

use std::f32::consts::PI;

const SPEED: f32 = 20.;
const RADIANT: f32 = PI / 180.;

#[derive(Component)]
pub struct PlayerController;

pub fn print_xyz(mut query: Query<(&Transform, With<PlayerController>)>) {
    let (transform, _) = query.single_mut();

    println!("{:?}", transform.translation);
}

pub fn camera_rotation(
    mut mouse: EventReader<MouseMotion>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, With<PlayerController>)>,
) {
    let (mut transform, _) = query.single_mut();

    let mut delta = Vec2::ZERO;

    for event in mouse.iter() {
        delta += event.delta;
    }

    transform.rotate_local_x(delta.y * RADIANT * SPEED * time.delta_seconds() * -1.);
    transform.rotate_y(delta.x * RADIANT * SPEED * time.delta_seconds() * -1.);
}

pub fn player_movement(
    kbd: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, With<PlayerController>)>,
    time: Res<Time>,
) {
    let (mut transform, _) = query.single_mut();

    let x: Vec3 = if kbd.pressed(KeyCode::Q) {
        let left = transform.left();
        Vec3::new(left.x, 0., left.z)
    } else if kbd.pressed(KeyCode::D) {
        let right = transform.right();
        Vec3::new(right.x, 0., right.z)
    } else {
        Vec3::ZERO
    };

    let y: Vec3 = if kbd.pressed(KeyCode::LShift) {
        -Vec3::Y
    } else if kbd.pressed(KeyCode::Space) {
        Vec3::Y
    } else {
        Vec3::ZERO
    };

    let z: Vec3 = if kbd.pressed(KeyCode::Z) {
        let forward = transform.forward();
        Vec3::new(forward.x, 0., forward.z)
    } else if kbd.pressed(KeyCode::S) {
        let back = transform.back();
        Vec3::new(back.x, 0., back.z)
    } else {
        Vec3::ZERO
    };

    let translation = x + y + z;

    transform.translation += translation * SPEED * time.delta_seconds();
}

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_movement)
            .add_system(camera_rotation)
            .add_system(print_xyz);
    }
}
