use crate::*;
use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;
use voxel::{CHUNK_SIZE_I32, CHUNK_DIM, Voxels};
use voxel::mesh_data::*;

#[derive(Component)]
pub(super) struct LoadedChunk;

#[derive(Component)]
pub struct ChunkLoader {
    pub radius: i32,
}

#[derive(Event)]
pub(super) struct ConstructChunkMesh {
    x: i32,
    y: i32,
    z: i32,
}

impl ConstructChunkMesh {
    pub(super) fn new(x: i32, y: i32, z: i32) -> ConstructChunkMesh {
        ConstructChunkMesh { x, y, z }
    }
}

#[derive(Event)]
pub(super) struct ChunkMeshUpdate {
    x: i32,
    y: i32,
    z: i32,
    vertices: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    indices: Vec<u32>,
}

pub(super) fn handle_chunk_constructions(
    mut rx: EventReader<ConstructChunkMesh>,
    mut tx: EventWriter<ChunkMeshUpdate>,
    voxels: Res<Voxels>,
) {
    let Some(&ConstructChunkMesh { x: chunk_x, y: chunk_y, z: chunk_z }) = rx.read().next()
    else {
        return;
    };

    let Some(chunk) = voxels.get_chunk(chunk_x, chunk_y, chunk_z)
    else {
        return;
    };

    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    for x in 0..CHUNK_SIZE_I32 {
        for y in 0..CHUNK_SIZE_I32 {
            for z in 0..CHUNK_SIZE_I32 {
                let voxel = chunk.get_block(x, y, z);
                let config = &voxels.configs[&voxel];
                if !config.render {
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
                    let neighbor = chunk.get_block(x + x_off, y + y_off, z + z_off);
                    let neighbor_config = &voxels.configs[&neighbor];

                    if neighbor_config.render {
                        continue;
                    }

                    indices.extend(CUBE_INDICES.iter().map(|i| i + vertices.len() as u32));
                    normals.extend(&CUBE_NORMALS[face]);
                    vertices.extend(CUBE_VERTICES[face].iter().map(|t| [
                        t[0] + chunk_x as f32 * CHUNK_DIM + x as f32,
                        t[1] + chunk_y as f32 * CHUNK_DIM + y as f32,
                        t[2] + chunk_z as f32 * CHUNK_DIM + z as f32,
                    ]));
                }
            }
        }
    }

    tx.send(ChunkMeshUpdate {
        x: chunk_x,
        y: chunk_y,
        z: chunk_z,
        vertices,
        normals,
        indices,
    });
}

pub(super) fn handle_chunk_mesh_update(
    mut commands: Commands,
    mut rx: EventReader<ChunkMeshUpdate>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    voxels: Res<Voxels>,
) {
    for ChunkMeshUpdate { x, y, z, vertices, normals, indices } in rx.read() {
        let (x, y, z) = (*x, *y, *z);
        if let Some(chunk) = voxels.get_chunk(x, y, z) {
            let mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
                .with_inserted_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    vertices.clone(),
                )
                .with_inserted_attribute(
                    Mesh::ATTRIBUTE_NORMAL,
                    normals.clone(),
                )
                .with_inserted_indices(Indices::U32(indices.clone()));
            let material = StandardMaterial {
                base_color: Color::rgb_u8(255, 255, 0),
                ..Default::default()
            };
            let mut entity = commands.entity(chunk.entity);
            entity.insert(meshes.add(mesh));
            entity.insert(materials.add(material));
        }
    }
}
