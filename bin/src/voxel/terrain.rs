use bevy::prelude::*;
use block_mesh::ndshape::{ConstShape, ConstShape3u32};
use block_mesh::{
    greedy_quads, GreedyQuadsBuffer, MergeVoxel, Voxel, VoxelVisibility, RIGHT_HANDED_Y_UP_CONFIG,
};
use noise::{NoiseFn, OpenSimplex};

#[derive(Component)]
pub struct TerrainGen;

#[derive(Clone, Copy, Eq, PartialEq)]
struct BoolVoxel(bool);

const EMPTY: BoolVoxel = BoolVoxel(false);
const FULL: BoolVoxel = BoolVoxel(true);

impl Voxel for BoolVoxel {
    fn get_visibility(&self) -> VoxelVisibility {
        if *self == EMPTY {
            VoxelVisibility::Empty
        } else {
            VoxelVisibility::Opaque
        }
    }
}

impl MergeVoxel for BoolVoxel {
    type MergeValue = Self;

    fn merge_value(&self) -> Self::MergeValue {
        *self
    }
}

pub fn generation(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let noise = OpenSimplex::new();

    type ChunkShape = ConstShape3u32<16, 16, 16>;

    let mut voxels = [EMPTY; ChunkShape::SIZE as usize];

    for i in 0..ChunkShape::SIZE {
        let [x, y, z] = ChunkShape::delinearize(i);

        voxels[i as usize] = if y < noise.get([x as f64, z as f64]) as u32 {
            FULL
        } else {
            EMPTY
        }
    }

    let mut buffer = GreedyQuadsBuffer::new(voxels.len());
    greedy_quads(
        &voxels,
        &ChunkShape {},
        [0; 3],
        [16; 3],
        &RIGHT_HANDED_Y_UP_CONFIG.faces,
        &mut buffer,
    );

    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        })
        .with_children(|parent| {});
}

impl Plugin for TerrainGen {
    fn build(&self, app: &mut App) {
        app.add_system(generation);
    }
}
