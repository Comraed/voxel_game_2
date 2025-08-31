use std::collections::{HashMap, VecDeque};
use bevy::ecs::component::{ComponentHook, ComponentHooks, HookContext, Mutable, StorageType};
use bevy::ecs::component::StorageType::Table;
use bevy::ecs::world::DeferredWorld;
use bevy::math::{IVec3, Vec3};
use bevy::prelude::{Commands, Component, Entity, Mesh, Query, Res, Resource, World};
use crate::block::{Block, BlockType};
use crate::{CHUNK_SIZE, CUBE_SIZE};
use crate::terrain::TerrainGenerator2d;
use crate::voxel_directions::VoxelDir;

#[derive(Resource, Default)]
pub struct ChunkPositionMap(HashMap<IVec3, Entity>);
impl ChunkPositionMap{
    pub(crate) fn get(&self, offset: IVec3) -> Option<Entity>{
        self.0.get(&offset).cloned()
    }

    pub(crate) fn offsets(&self) -> Vec<IVec3>{
        self.0.keys().cloned().collect::<Vec<_>>()
    }
}

#[derive(PartialEq)]
pub enum ChunkState{
    Populated,
    Loaded,
    Drawn,
}

pub struct Chunk{
    pub offset: IVec3,
    pub block_data: [[[BlockType; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    pub state: ChunkState,
}

impl Chunk{
    pub fn new(offset: IVec3, generator: &Res<TerrainGenerator2d>) -> Self{

        let mut block_data = [[[BlockType::Air ;CHUNK_SIZE];CHUNK_SIZE];CHUNK_SIZE];
        let chunk_size_i32 = CHUNK_SIZE as i32;
        for x in 0..chunk_size_i32{
            for y in 0..chunk_size_i32{
                for z in 0..chunk_size_i32{

                    let world_x = ((offset.x * chunk_size_i32) + x) as f32 * CUBE_SIZE;
                    let world_y = ((offset.y * chunk_size_i32) + y) as f32 * CUBE_SIZE;
                    let world_z = ((offset.z * chunk_size_i32) + z) as f32 * CUBE_SIZE;

                    if (world_y) < (generator.generate(world_x, world_z) - 1.5) {
                        block_data[x as usize][y as usize][z as usize] = BlockType::Stone;
                    }
                    else if (world_y) < generator.generate(world_x, world_z) {
                        block_data[x as usize][y as usize][z as usize] = BlockType::Grass;
                    }
                }
            }
        }

        Self{ offset, block_data, state: ChunkState::Populated }
    }
}

impl Component for Chunk{
    const STORAGE_TYPE: StorageType = Table;
    type Mutability = Mutable;
    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world :DeferredWorld, context: HookContext|{
            //add to position map
            let chunk_entity = context.entity;

            let offset = world.get::<Chunk>(chunk_entity).expect("being added").offset;
            let mut map = world.resource_mut::<ChunkPositionMap>();
            map.0.insert(offset, chunk_entity);

            world.get_mut::<Chunk>(chunk_entity).expect("being added").state = ChunkState::Loaded
        });
        hooks.on_remove(|mut world :DeferredWorld, context:HookContext|{
            //remove from position map
            let offset = world.get::<Chunk>(context.entity).expect("being removed").offset;
            let mut map = world.resource_mut::<ChunkPositionMap>();
            map.0.remove(&offset);
        });
    }
}

pub const CHUNK_SIZE_PAD: usize = CHUNK_SIZE + 2;
pub struct ChunkRenderData{
    pub solid_face_axes: [[[u64; CHUNK_SIZE_PAD]; CHUNK_SIZE_PAD];3],
    pub texture_face_axes: Vec<[[[u64; CHUNK_SIZE_PAD]; CHUNK_SIZE_PAD];3]>,
}
