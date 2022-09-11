use std::f32::consts::PI;

use block_mesh::RIGHT_HANDED_Y_UP_CONFIG;

use bevy_math::{Quat, Vec2, Vec3};

#[derive(PartialEq, Debug)]
pub enum Orientation {
    Top,
    Bottom,
    Left,
    Right,
    Back,
    Front,
}

pub fn deg2rand(deg: i32) -> f32 {
    deg as f32 * PI / 180.
}

pub fn parse_orientation(minimum: Vec3, size: Vec2, orientation: &Orientation) -> (Vec3, Quat) {
    let putorigin2corner: Vec3 = Vec3::new(size.x / 2. - 0.5, size.y / 2. - 0.5, 0.5);

    match orientation {
        Orientation::Top => (
            minimum + Vec3::new(0., size.y / 2., -size.y / 2.) + putorigin2corner,
            Quat::from_rotation_x(deg2rand(-90)),
        ),
        Orientation::Bottom => (
            minimum + Vec3::new(0., -size.y / 2., size.y / 2. - 1.) + putorigin2corner,
            Quat::from_rotation_x(deg2rand(90)),
        ),
        Orientation::Left => (
            minimum + Vec3::new(-size.x / 2., 0., size.x / 2. - 1.) + putorigin2corner,
            Quat::from_rotation_y(deg2rand(-90)),
        ),
        Orientation::Right => (
            minimum + Vec3::new(-size.x / 2. + 1., 0., -size.x / 2.) + putorigin2corner,
            Quat::from_rotation_y(deg2rand(90)),
        ),
        Orientation::Front => (
            minimum + Vec3::new(0., 0., 0.) + putorigin2corner,
            Quat::IDENTITY,
        ),
        Orientation::Back => (
            minimum + Vec3::new(-size.x + 1., 0., -1.) + putorigin2corner,
            Quat::from_rotation_y(deg2rand(180)),
        ),
    }
}

pub fn parse_normal(index: usize, custom: Option<[i32; 3]>) -> Orientation {
    let normals = RIGHT_HANDED_Y_UP_CONFIG.faces[index].quad_mesh_normals()[0];

    let normal = if let Some(c) = custom {
        c
    } else {
        [normals[0] as i32, normals[1] as i32, normals[2] as i32]
    };

    match normal {
        [0, 1, 0] => Orientation::Top,
        [0, -1, 0] => Orientation::Bottom,
        [-1, 0, 0] => Orientation::Left,
        [1, 0, 0] => Orientation::Right,
        [0, 0, 1] => Orientation::Front,
        [0, 0, -1] => Orientation::Back,
        _ => panic!("Invalid normal"),
    }
}
