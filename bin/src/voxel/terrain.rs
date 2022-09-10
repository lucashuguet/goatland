use std::f32::consts::PI;

use bevy::prelude::*;
use block_mesh::RIGHT_HANDED_Y_UP_CONFIG;

use super::chunk;

#[derive(Component)]
pub struct Chunk {
    #[allow(dead_code)]
    position: Vec2,
}

#[derive(Component)]
pub struct TerrainGen;

#[derive(Debug)]
enum Orientation {
    Top,
    Bottom,
    Left,
    Right,
    Back,
    Front,
}

fn deg2rand(deg: i32) -> f32 {
    deg as f32 * PI / 180.
}

fn squaregen(
    size: Vec2,
    minimum: Vec3,
    orientation: &Orientation,
    meshes: &mut ResMut<Assets<Mesh>>,
    material: Handle<StandardMaterial>,
) -> PbrBundle {
    let putorigin2corner: Vec3 = Vec3::new(size.x / 2. - 0.5, size.y / 2. - 0.5, 0.5);

    let (position, rotation) = match orientation {
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
    };

    PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Quad { size, ..default() })),
        material,
        transform: Transform {
            translation: position,
            rotation,
            ..default()
        },
        ..default()
    }
}

pub fn spawnchunk(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    coord: Vec2,
) {
    let buffer = chunk::genchunk(coord.x as i32, coord.y as i32);

    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(coord.x * 16., 0., coord.y * 16.),
            ..default()
        })
        .with_children(|parent| {
            for (index, quadbuffer) in buffer.groups.iter().enumerate() {
                let normal = RIGHT_HANDED_Y_UP_CONFIG.faces[index].quad_mesh_normals()[0];

                let orientation = match [normal[0] as i32, normal[1] as i32, normal[2] as i32] {
                    [1, 0, 0] => Orientation::Right,
                    [0, 1, 0] => Orientation::Top,
                    [0, 0, 1] => Orientation::Front,
                    [-1, 0, 0] => Orientation::Left,
                    [0, -1, 0] => Orientation::Bottom,
                    [0, 0, -1] => Orientation::Back,
                    _ => panic!("Invalid normal"),
                };

                for quad in quadbuffer.iter() {
                    parent.spawn_bundle(squaregen(
                        Vec2::new(1., 1.),
                        Vec3::new(
                            quad.minimum[0] as f32,
                            quad.minimum[1] as f32,
                            quad.minimum[2] as f32,
                        ),
                        &orientation,
                        meshes,
                        materials.add(Color::GREEN.into()),
                    ));
                }
            }
        })
        .insert(Chunk { position: coord });
}

pub fn generation(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1. })),
        material: materials.add(Color::YELLOW.into()),
        transform: Transform::from_xyz(0.5, 0.5, 0.5),
        ..default()
    });

    // commands.spawn_bundle(squaregen(
    //     Vec2::ONE,
    //     Vec3::ZERO,
    //     &Orientation::Front,
    //     &mut meshes,
    //     materials.add(Color::GREEN.into()),
    // ));

    // commands.spawn_bundle(squaregen(
    //     Vec2::ONE,
    //     Vec3::ZERO,
    //     &Orientation::Back,
    //     &mut meshes,
    //     materials.add(Color::BLACK.into()),
    // ));

    // commands.spawn_bundle(squaregen(
    //     Vec2::ONE,
    //     Vec3::ZERO,
    //     &Orientation::Left,
    //     &mut meshes,
    //     materials.add(Color::BLUE.into()),
    // ));

    // commands.spawn_bundle(squaregen(
    //     Vec2::ONE,
    //     Vec3::ZERO,
    //     &Orientation::Right,
    //     &mut meshes,
    //     materials.add(Color::PINK.into()),
    // ));

    // commands.spawn_bundle(squaregen(
    //     Vec2::ONE,
    //     Vec3::ZERO,
    //     &Orientation::Top,
    //     &mut meshes,
    //     materials.add(Color::RED.into()),
    // ));

    // commands.spawn_bundle(squaregen(
    //     Vec2::ONE,
    //     Vec3::ZERO,
    //     &Orientation::Bottom,
    //     &mut meshes,
    //     materials.add(Color::PURPLE.into()),
    // ));

    spawnchunk(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec2::new(0., 0.),
    );

    spawnchunk(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec2::new(-1., 0.),
    );

    spawnchunk(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec2::new(-1., -1.),
    );

    spawnchunk(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec2::new(0., -1.),
    )
}

impl Plugin for TerrainGen {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generation);
    }
}
