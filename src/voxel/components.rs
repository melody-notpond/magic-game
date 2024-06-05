use std::sync::{Arc, RwLock};

use crate::*;
use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::tasks::futures_lite::future;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use voxel::{CHUNK_SIZE_I32, Voxels, VOXEL_SIZE};
use voxel::mesh_data::*;

use super::VoxelRes;

#[derive(Component)]
pub(super) struct Chunk {
    pub(super) loaded: bool,
    pub(super) mark: bool,
}

#[derive(Component)]
pub struct ChunkLoader {
    pub x_radius: i32,
    pub y_radius: i32,
    pub z_radius: i32,
}

#[derive(Event)]
pub struct GenerateChunk {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl GenerateChunk {
    pub fn new(x: i32, y: i32, z: i32) -> GenerateChunk {
        GenerateChunk { x, y, z }
    }
}

#[derive(Event)]
pub struct ConstructChunkMesh {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ConstructChunkMesh {
    pub fn new(x: i32, y: i32, z: i32) -> ConstructChunkMesh {
        ConstructChunkMesh { x, y, z }
    }
}

#[derive(Component)]
pub(super) struct ChunkMeshWaiter(Task<(Collider, Mesh)>);

pub(super) fn init_chunk_construction(
    mut commands: Commands,
    voxels: Res<VoxelRes>,
    mut rx: EventReader<ConstructChunkMesh>,
) {
    let v = voxels.clone();
    let Ok(voxels) = voxels.try_read()
    else {
        return;
    };

    let pool = AsyncComputeTaskPool::get();
    for &ConstructChunkMesh { x, y, z } in rx.read() {
        let Some(chunk) = voxels.get_chunk(x, y, z)
        else {
            continue;
        };

        let task = pool.spawn(construct_chunk(x, y, z, v.clone()));
        commands.entity(chunk.entity).insert(ChunkMeshWaiter(task));
    }
}

pub(super) fn handle_chunk_mesh_update(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut waiting_chunks: Query<(Entity, &mut ChunkMeshWaiter)>,
) {
    for (entity, mut waiter) in waiting_chunks.iter_mut() {
        let Some((col, mesh)) =
            future::block_on(future::poll_once(&mut waiter.0))
        else {
            continue;
        };

        let material = StandardMaterial {
            base_color: Color::rgb_u8(255, 255, 0),
            ..Default::default()
        };

        commands.entity(entity)
            .insert(col)
            .insert(meshes.add(mesh))
            .insert(materials.add(material))
            .remove::<ChunkMeshWaiter>();
    }
}

async fn construct_chunk(
    chunk_x: i32,
    chunk_y: i32,
    chunk_z: i32,
    voxels: Arc<RwLock<Voxels>>,
) -> (Collider, Mesh) {
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let chx = CHUNK_SIZE_I32 * chunk_x;
    let chy = CHUNK_SIZE_I32 * chunk_y;
    let chz = CHUNK_SIZE_I32 * chunk_z;
    for x in 0..CHUNK_SIZE_I32 {
        for y in 0..CHUNK_SIZE_I32 {
            for z in 0..CHUNK_SIZE_I32 {
                let voxel = voxels.read().unwrap()
                    .get_block(chx + x, chy + y, chz + z);

                if !voxel.config(&*voxels.read().unwrap()).render {
                    continue;
                }

                for face in 0..6 {
                    let (x_off, y_off, z_off) = [
                        (1, 0, 0),
                        (0, 1, 0),
                        (0, 0, 1),
                        (-1, 0, 0),
                        (0, -1, 0),
                        (0, 0, -1),
                    ][face];
                    let neighbor = voxels.read().unwrap()
                        .get_block(
                            chx + x + x_off, chy + y + y_off, chz + z + z_off);

                    if neighbor.config(&*voxels.read().unwrap()).render {
                        continue;
                    }

                    indices.extend(CUBE_INDICES.iter().map(|i| i + vertices.len() as u32));
                    normals.extend(&CUBE_NORMALS[face]);
                    vertices.extend(CUBE_VERTICES[face].iter().map(|v| [
                        (v[0] + x as f32) * VOXEL_SIZE,
                        (v[1] + y as f32) * VOXEL_SIZE,
                        (v[2] + z as f32) * VOXEL_SIZE,
                    ]));
                }
            }
        }
    }

    (Collider::trimesh(
        vertices.iter().map(|&[x, y, z]| Vec3::new(x, y, z)).collect(),
        indices.chunks(3).map(|x| TryInto::<[u32; 3]>::try_into(x).unwrap())
            .collect()),
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vertices,
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            normals,
        )
        .with_inserted_indices(Indices::U32(indices)))
}
