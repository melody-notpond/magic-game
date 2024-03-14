use bevy::utils::{HashMap, hashbrown::hash_map::Entry};
use components::*;

use crate::*;

pub mod components;
mod mesh_data;

pub const VOXEL_SIZE: f32 = 0.5;
pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_SIZE_I32: i32 = CHUNK_SIZE as i32;
pub const CHUNK_DIM: f32 = CHUNK_SIZE as f32 * VOXEL_SIZE;
pub const CHUNK_SIZE_SQ: usize = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_SIZE_CB: usize = CHUNK_SIZE_SQ * CHUNK_SIZE;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct VoxelId(u32);

impl VoxelId {
    pub fn air() -> VoxelId {
        VoxelId(0)
    }

    pub fn config(self, voxels: &Voxels) -> &VoxelConfigEntry {
        &voxels.configs[&self]
    }
}

#[derive(Resource)]
pub struct Voxels {
    chunks: HashMap<(i32, i32, i32), ChunkVoxels>,
    configs: HashMap<VoxelId, VoxelConfigEntry>,
    voxel_names: HashMap<String, VoxelId>,
    next_id: VoxelId,
    loaded_chunk_mark: bool,
}

impl Default for Voxels {
    fn default() -> Self {
        Voxels {
            chunks: HashMap::new(),
            configs: {
                let mut map = HashMap::new();
                map.insert(VoxelId(0), VoxelConfigEntry {
                    debug_name: "air".to_owned(),
                    render: false,
                    solid: false,
                    color: Color::rgba_u8(0, 0, 0, 0),
                });
                map
            },
            voxel_names: {
                let mut map = HashMap::new();
                map.insert("air".to_owned(), VoxelId(0));
                map
            },
            next_id: VoxelId(1),
            loaded_chunk_mark: false,
        }
    }
}

impl Voxels {
    pub fn add_chunk(
        &mut self,
        mut commands: Commands,
        x: i32,
        y: i32,
        z: i32,
    ) -> bool {
        let Entry::Vacant(v) = self.chunks.entry((x, y, z))
        else {
            return false;
        };

        let transform = Transform::from_xyz(
            x as f32 * CHUNK_DIM,
            y as f32 * CHUNK_DIM,
            z as f32 * CHUNK_DIM,
        );

        let entity = commands.spawn((SpatialBundle {
            transform,
            visibility: Visibility::Hidden,
            ..Default::default()
        }, Chunk {
            loaded: false,
            mark: false,
        })).id();

        let voxels: Box<[VoxelId; CHUNK_SIZE_CB]> =
            vec![VoxelId::air(); CHUNK_SIZE_CB]
            .into_boxed_slice()
            .try_into()
            .unwrap();

        let chunk = ChunkVoxels {
            voxels,
            entity,
        };

        v.insert(chunk);
        true
    }

    pub fn has_chunk(&self, x: i32, y: i32, z: i32) -> bool {
        self.chunks.contains_key(&(x, y, z))
    }

    pub fn get_chunk(&self, x: i32, y: i32, z: i32) -> Option<&ChunkVoxels> {
        self.chunks.get(&(x, y, z))
    }

    pub fn get_chunk_mut(&mut self, x: i32, y: i32, z: i32) -> Option<&mut ChunkVoxels> {
        self.chunks.get_mut(&(x, y, z))
    }

    pub fn set_block(&mut self, x: i32, y: i32, z: i32, id: VoxelId) {
        let (i, j, k) = (
            x.div_euclid(CHUNK_SIZE as i32),
            y.div_euclid(CHUNK_SIZE as i32),
            z.div_euclid(CHUNK_SIZE as i32),
        );
        if let Some(chunk) = self.chunks.get_mut(&(i, j, k)) {
            let (i, j, k) = (
                x.rem_euclid(CHUNK_SIZE as i32) as usize,
                y.rem_euclid(CHUNK_SIZE as i32) as usize,
                z.rem_euclid(CHUNK_SIZE as i32) as usize,
            );

            chunk.set_block(i, j, k, id);
        }
    }

    pub fn get_block(&self, x: i32, y: i32, z: i32) -> VoxelId {
        let (i, j, k) = (
            x.div_euclid(CHUNK_SIZE as i32),
            y.div_euclid(CHUNK_SIZE as i32),
            z.div_euclid(CHUNK_SIZE as i32),
        );

        if let Some(chunk) = self.chunks.get(&(i, j, k)) {
            let (i, j, k) = (
                x.rem_euclid(CHUNK_SIZE as i32),
                y.rem_euclid(CHUNK_SIZE as i32),
                z.rem_euclid(CHUNK_SIZE as i32),
            );

            chunk.get_block(i, j, k)
        } else {
            VoxelId::air()
        }
    }

    pub fn get_voxel_config(&self, id: VoxelId) -> Option<&VoxelConfigEntry> {
        self.configs.get(&id)
    }

    pub fn get_voxel_config_mut(&mut self, id: VoxelId) -> Option<&mut VoxelConfigEntry> {
        self.configs.get_mut(&id)
    }

    pub fn id_from_name(&self, name: &str) -> Option<VoxelId> {
        self.voxel_names.get(name).cloned()
    }

    pub fn add_voxel(&mut self, name: &str, data: VoxelConfigEntry) -> VoxelId {
        let id = self.next_id;
        self.next_id = VoxelId(id.0 + 1);
        self.configs.insert(id, data);
        self.voxel_names.insert(name.to_owned(), id);
        id
    }
}

pub struct ChunkVoxels {
    voxels: Box<[VoxelId; CHUNK_SIZE_CB]>,
    entity: Entity,
}

impl ChunkVoxels {
    fn set_block(&mut self, x: usize, y: usize, z: usize, id: VoxelId) {
        let i = x * CHUNK_SIZE_SQ + y * CHUNK_SIZE + z;
        self.voxels[i] = id;
    }

    fn get_block(&self, x: i32, y: i32, z: i32) -> VoxelId {
        let x = x as usize;
        let y = y as usize;
        let z = z as usize;
        if x < CHUNK_SIZE && y < CHUNK_SIZE && z < CHUNK_SIZE {
            self.voxels[x * CHUNK_SIZE_SQ + y * CHUNK_SIZE + z]
        } else {
            VoxelId::air()
        }
    }
}

pub struct VoxelConfigEntry {
    pub debug_name: String,
    pub render: bool,
    pub solid: bool,
    pub color: Color,
}

fn setup_voxels(mut commands: Commands) {
    commands.init_resource::<Voxels>();
    commands.init_resource::<Events<GenerateChunk>>();
    commands.init_resource::<Events<ConstructChunkMesh>>();
    commands.init_resource::<Events<ChunkMeshUpdate>>();
}

fn load_chunks(
    mut commands: Commands,
    mut voxels: ResMut<Voxels>,
    mut tx_gen: EventWriter<GenerateChunk>,
    mut tx_cons: EventWriter<ConstructChunkMesh>,
    loaders: Query<(&ChunkLoader, &Transform)>,
    mut chunks: Query<(&mut Chunk, &mut Visibility)>,
) {
    for (loader, trans) in loaders.iter() {
        let (chunk_x, chunk_y, chunk_z) = (
            trans.translation.x.div_euclid(CHUNK_DIM) as i32,
            trans.translation.y.div_euclid(CHUNK_DIM) as i32,
            trans.translation.z.div_euclid(CHUNK_DIM) as i32,
        );

        for x in chunk_x - loader.x_radius ..= chunk_x + loader.x_radius {
            for y in chunk_y - loader.y_radius ..= chunk_y + loader.y_radius {
                for z in chunk_z - loader.z_radius ..= chunk_z + loader.z_radius {
                    if voxels.add_chunk(
                        commands.reborrow(),
                        x, y, z,
                    ) {
                        tx_gen.send(GenerateChunk::new(x, y, z));
                        tx_cons.send(ConstructChunkMesh::new(x + 1, y + 0, z + 0));
                        tx_cons.send(ConstructChunkMesh::new(x + 0, y + 1, z + 0));
                        tx_cons.send(ConstructChunkMesh::new(x + 0, y + 0, z + 1));
                        tx_cons.send(ConstructChunkMesh::new(x - 1, y + 0, z + 0));
                        tx_cons.send(ConstructChunkMesh::new(x + 0, y - 1, z + 0));
                        tx_cons.send(ConstructChunkMesh::new(x + 0, y + 0, z - 1));
                    }

                    if let Some(chunk) = voxels.get_chunk(x, y, z) {
                        commands.entity(chunk.entity)
                            .insert(Visibility::Visible)
                            .insert(Chunk {
                                loaded: true,
                                mark: voxels.loaded_chunk_mark,
                            });
                    }
                }
            }
        }
    }

    for (mut chunk, mut vis) in chunks.iter_mut() {
        if chunk.mark != voxels.loaded_chunk_mark {
            *vis = Visibility::Hidden;
            chunk.loaded = false;
        }
    }

    voxels.loaded_chunk_mark ^= true;
}

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, setup_voxels)
            .add_systems(Update, (
                load_chunks,
                handle_chunk_constructions,
                handle_chunk_mesh_update
            ))
        ;
    }
}
