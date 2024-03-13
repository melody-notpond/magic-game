use bevy::utils::{HashMap, hashbrown::hash_map::Entry};
use components::*;

use crate::*;

pub mod components;
mod mesh_data;

pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_SIZE_I32: i32 = CHUNK_SIZE as i32;
pub const CHUNK_DIM: f32 = CHUNK_SIZE as f32;
pub const CHUNK_SIZE_SQ: usize = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_SIZE_CB: usize = CHUNK_SIZE_SQ * CHUNK_SIZE;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct VoxelId(u32);

impl VoxelId {
    pub fn air() -> VoxelId {
        VoxelId(0)
    }
}

#[derive(Resource)]
pub struct Voxels {
    chunks: HashMap<(i32, i32, i32), ChunkVoxels>,
    configs: HashMap<VoxelId, VoxelConfigEntry>,
    voxel_names: HashMap<String, VoxelId>,
    next_id: VoxelId,
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
                    stay_cube: false,
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
        should_load: bool,
    ) -> bool {
        let Entry::Vacant(v) = self.chunks.entry((x, y, z))
        else {
            return false;
        };

        let transform = Transform::from_xyz(
            x as f32 * CHUNK_SIZE as f32,
            y as f32 * CHUNK_SIZE as f32,
            z as f32 * CHUNK_SIZE as f32,
        );

        let entity = if should_load {
            commands.spawn((SpatialBundle {
                transform,
                ..Default::default()
            }, LoadedChunk))
        } else {
            commands.spawn(SpatialBundle {
                transform,
                visibility: Visibility::Hidden,
                ..Default::default()
            })
        }.id();

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

    pub fn get_block(&mut self, x: i32, y: i32, z: i32) -> Option<VoxelId> {
        let (i, j, k) = (
            x.div_euclid(CHUNK_SIZE as i32),
            y.div_euclid(CHUNK_SIZE as i32),
            z.div_euclid(CHUNK_SIZE as i32),
        );

        if let Some(chunk) = self.chunks.get_mut(&(i, j, k)) {
            let (i, j, k) = (
                x.rem_euclid(CHUNK_SIZE as i32),
                y.rem_euclid(CHUNK_SIZE as i32),
                z.rem_euclid(CHUNK_SIZE as i32),
            );

            Some(chunk.get_block(i, j, k))
        } else {
            None
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
    pub stay_cube: bool,
    pub solid: bool,
    pub color: Color,
}

fn setup_voxels(mut commands: Commands) {
    commands.init_resource::<Voxels>();
    commands.init_resource::<Events<ConstructChunkMesh>>();
    commands.init_resource::<Events<ChunkMeshUpdate>>();
}

fn load_chunks(
    mut commands: Commands,
    mut voxels: ResMut<Voxels>,
    mut queue: EventWriter<ConstructChunkMesh>,
    loaders: Query<(&ChunkLoader, &Transform)>,
    // _loaded: Query<Entity, With<LoadedChunk>>,
) {
    for (loader, trans) in loaders.iter() {
        let (chunk_x, chunk_y, chunk_z) = (
            trans.translation.x.div_euclid(CHUNK_SIZE as f32) as i32,
            trans.translation.y.div_euclid(CHUNK_SIZE as f32) as i32,
            trans.translation.z.div_euclid(CHUNK_SIZE as f32) as i32,
        );

        for x in chunk_x - loader.radius ..= chunk_x + loader.radius {
            for y in chunk_y - loader.radius ..= chunk_y + loader.radius {
                for z in chunk_z - loader.radius ..= chunk_z + loader.radius {
                    if voxels.add_chunk(
                        commands.reborrow(),
                        x, y, z,
                        true,
                    ) {
                        queue.send(ConstructChunkMesh::new(x, y, z));
                    }
                }
            }
        }
    }
}

fn post_setup_voxels_test(
    mut commands: Commands,
    mut voxels: ResMut<Voxels>,
    mut queue: EventWriter<ConstructChunkMesh>,
) {
    let solid = voxels.add_voxel("solid", VoxelConfigEntry {
        debug_name: "solid".to_owned(),
        render: true,
        stay_cube: true,
        solid: true,
        color: Color::rgb_u8(255, 255, 0),
    });

    if voxels.add_chunk(
        commands.reborrow(),
        0, 0, 0,
        true,
    ) {
        for x in 0..CHUNK_SIZE_I32 {
            for y in 0..CHUNK_SIZE_I32 {
                for z in 0..CHUNK_SIZE_I32 {
                    voxels.set_block(x, y, z, solid);
                }
            }
        }

        queue.send(ConstructChunkMesh::new(0, 0, 0));
    }
}

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (setup_voxels, post_setup_voxels_test).chain())
            .add_systems(Update, (
                load_chunks,
                handle_chunk_constructions,
                handle_chunk_mesh_update
            ))
        ;
    }
}
