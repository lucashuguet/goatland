use block_mesh::ndshape::{ConstShape, ConstShape3u32};
use block_mesh::{
    visible_block_faces, MergeVoxel, UnitQuadBuffer, Voxel, VoxelVisibility,
    RIGHT_HANDED_Y_UP_CONFIG,
};

use noise::{NoiseFn, OpenSimplex};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct BoolVoxel(pub bool);

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

pub fn genchunk(chunkx: i32, chunkz: i32, seed: u32) -> UnitQuadBuffer {
    let noise = OpenSimplex::new(seed);

    type ChunkShape = ConstShape3u32<18, 130, 18>;

    let mut voxels = [FULL; ChunkShape::SIZE as usize];

    for i in 0..ChunkShape::SIZE {
        let [x, y, z] = ChunkShape::delinearize(i);

        let noisey = (noise.get([
            (x as i32 + chunkx * 16) as f64 / 10.,
            (z as i32 + chunkz * 16) as f64 / 10.,
        ]) * 10.) as u32
            + 10;

        voxels[i as usize] = if y <= noisey { FULL } else { EMPTY }
    }

    let mut buffer = UnitQuadBuffer::new();
    visible_block_faces(
        &voxels,
        &ChunkShape {},
        [0; 3],
        [17, 129, 17],
        &RIGHT_HANDED_Y_UP_CONFIG.faces,
        &mut buffer,
    );

    buffer
}
