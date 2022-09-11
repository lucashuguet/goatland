use bevy::prelude::*;

use client::mesh::genchunk;
use client::orientation::{parse_normal, parse_orientation, Orientation};

#[derive(Component)]
pub struct Chunk {
    #[allow(dead_code)]
    position: Vec2,
}

#[derive(Component)]
pub struct TerrainGen;

fn squaregen(
    size: Vec2,
    minimum: Vec3,
    orientation: &Orientation,
    meshes: &mut ResMut<Assets<Mesh>>,
    material: Handle<StandardMaterial>,
) -> PbrBundle {
    let (position, rotation) = parse_orientation(minimum, size, orientation);

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
    let buffer = genchunk(coord.x as i32, coord.y as i32, 2);

    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(coord.x * 16., 0., coord.y * 16.),
            ..default()
        })
        .with_children(|parent| {
            for (index, quadbuffer) in buffer.groups.iter().enumerate() {
                let orientation = parse_normal(index);

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
