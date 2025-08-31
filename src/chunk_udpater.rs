use std::collections::{HashMap, VecDeque};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::ecs::relationship::RelationshipSourceCollection;
use bevy::math::{IVec3, Quat, Vec2, Vec3};
use bevy::math::ops::{ceil, floor};
use bevy::pbr::StandardMaterial;
use bevy::platform::collections::Equivalent;
use bevy::prelude::{Camera, Commands, Component, Entity, Mesh, Mesh3d, MeshMaterial3d, Query, Rectangle, Res, ResMut, Single, Transform, With};
use bevy::reflect::Array;
use crate::chunk::{Chunk, ChunkPositionMap, ChunkRenderData, ChunkState, CHUNK_SIZE_PAD};
use crate::{Player, CHUNK_SIZE, CUBE_SIZE};
use crate::block::BlockType;
use crate::block::BlockType::Air;
use crate::terrain::TerrainGenerator2d;
use crate::voxel_directions::VoxelDir;

const MAX_LOAD_CHUNKS_PER_UPDATE: usize = 4;
const MAX_DRAW_CHUNKS_PER_UPDATE: usize = 4;
const CHUNK_RENDER_DIST: f32 = 2f32;

pub fn load_chunks(
    mut commands: Commands,
    pos_map: Res<ChunkPositionMap>,
    terrain_gen: Res<TerrainGenerator2d>,
    player_trans: Single<&Transform, With<Player>>
){
    let max_chunk = ceil(CHUNK_RENDER_DIST) as i32;
    let min_chunk = floor(-CHUNK_RENDER_DIST) as i32;

    let mut chunk_to_dist = Vec::with_capacity(ceil(CHUNK_RENDER_DIST * 2.0) as usize ^ 3);
    let world_render_dist = CHUNK_RENDER_DIST * CHUNK_SIZE as f32 * CUBE_SIZE;
    let player_pos = player_trans.translation;

    for x in min_chunk..max_chunk {
        for y in min_chunk..max_chunk {
            for z in min_chunk..max_chunk {
                let x = x + (player_pos.x/(CHUNK_SIZE as f32)) as i32;
                let y = y + (player_pos.y/(CHUNK_SIZE as f32)) as i32;
                let z = z + (player_pos.z/(CHUNK_SIZE as f32)) as i32;

                let check_offset = IVec3::new(x, y, z);
                let centre = Vec3::new(
                    ((z as f32 + 0.5) * CHUNK_SIZE as f32) * CUBE_SIZE,
                    ((z as f32 + 0.5) * CHUNK_SIZE as f32) * CUBE_SIZE,
                    ((z as f32 + 0.5) * CHUNK_SIZE as f32) * CUBE_SIZE,
                );

                let dist = centre.distance_squared(player_pos);
                if dist > (world_render_dist * world_render_dist) {continue}

                chunk_to_dist.push((check_offset, (dist * 100.0) as u32));
            }
        }
    }

    chunk_to_dist.sort_by(|(_, dist_a), (_, dist_b)| dist_a.cmp(dist_b));
    let chunk_queue = chunk_to_dist.into_iter().map(|(offset, _)| offset).collect::<Vec<_>>();
    let mut to_remove = pos_map.offsets();
    let mut spawned = 0;

    for offset in chunk_queue.into_iter() {
        if let Some(i) = to_remove.iter().position(|key| key == &offset) {
            to_remove.remove(i);
            continue
        }

        if spawned >= MAX_LOAD_CHUNKS_PER_UPDATE { continue }

        println!("spawn chunk: {}", offset);
        commands.spawn((
            Chunk::new(offset, &terrain_gen),
            Transform::from_translation(offset.as_vec3() * CHUNK_SIZE as f32)
        ));
        spawned += 1;
    }

    for offset in to_remove {
        if let Some(remove_ent) = pos_map.get(offset){
            if let Ok(mut ent_commands) =commands.get_entity(remove_ent) {
                ent_commands.despawn();
                println!("despawn chunk: {}", offset);
            }
        }
    }
}

pub fn draw_chunks(
    q_chunks: Query<(&mut Chunk, Entity)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
){
    let plane_mesh = meshes.add(Mesh::from(Rectangle::from_size(Vec2::splat(1.0))));
    let white_material = materials.add(StandardMaterial::from_color(Color::WHITE));
    let mut drawn_chunks = 0;

    for (mut chunk, chunk_ent) in q_chunks.into_iter().filter(|chunk| chunk.0.state == ChunkState::Loaded) {
        if drawn_chunks >= MAX_DRAW_CHUNKS_PER_UPDATE { continue; }
        let mut solid_face_axes: [[[u64; CHUNK_SIZE_PAD]; CHUNK_SIZE_PAD];3] = [[[0u64; CHUNK_SIZE_PAD]; CHUNK_SIZE_PAD];3];
        let texture_face_axes: Vec<[[[u64; CHUNK_SIZE_PAD]; CHUNK_SIZE_PAD];3]> = Vec::new();

        for x in 0..CHUNK_SIZE_PAD {
            for y in 0..CHUNK_SIZE_PAD {
                for z in 0..CHUNK_SIZE_PAD {
                    if  x == 0 || x == CHUNK_SIZE_PAD-1 ||
                        y == 0 || y == CHUNK_SIZE_PAD-1 ||
                        z == 0 || z == CHUNK_SIZE_PAD-1
                    {continue}

                    let x = x - 1;
                    let y = y - 1;
                    let z = z - 1;

                    let block_type = chunk.block_data[x][y][z];
                    if !block_type.is_solid() {continue}

                    let mut texture_faces = *texture_face_axes.get(block_type.idx()).unwrap_or(&[[[0u64; CHUNK_SIZE_PAD]; CHUNK_SIZE_PAD];3]);

                    let x_bit = 1u64 << x;
                    let y_bit = 1u64 << y;
                    let z_bit = 1u64 << z;

                    texture_faces[0][x][z] |= y_bit;
                    solid_face_axes[0][x][z] |= y_bit;

                    texture_faces[1][x][y] |= z_bit;
                    solid_face_axes[1][x][y] |= z_bit;

                    texture_faces[2][y][z] |= x_bit;
                    solid_face_axes[2][y][z] |= x_bit;
                }
            }
        }

        let chunk_data = ChunkRenderData {
            solid_face_axes,
            texture_face_axes
        };
        let dirs = [VoxelDir::Top, VoxelDir::Fnt, VoxelDir::Rgt];


        let mut lst_planes = Vec::new();
        let offset = chunk.offset * CHUNK_SIZE as i32;

        if let Some(grass_data) = chunk_data.texture_face_axes.get(BlockType::Grass.idx()) {
            let axis = grass_data[0];

            for x in 0..CHUNK_SIZE_PAD {
                for y in 0..CHUNK_SIZE_PAD {
                    for z in 0..CHUNK_SIZE_PAD {
                        let draw = (1u64 << y) & axis[x][z] > 0;
                        if !draw { continue }

                        let norm = Vec3::from(VoxelDir::Top);
                        let cube_cnt_pos = Vec3::new(x as f32, y as f32, z as f32) + 0.5;
                        let plane_pos = cube_cnt_pos + norm / 2.0;

                        let transform = Transform::from_translation(plane_pos)
                            .with_scale(Vec3::splat(1.0))
                            .with_rotation(Quat::from(VoxelDir::Top));

                        let plane_ent = commands.spawn((
                            Mesh3d(plane_mesh.clone()),
                            MeshMaterial3d(white_material.clone()),
                            transform
                        )).id();
                        lst_planes.add(plane_ent);
                    }
                }
            }
        }
        commands.get_entity(chunk_ent).unwrap().add_children(&*lst_planes);

        chunk.state = ChunkState::Drawn;
        drawn_chunks += 1;
    }
}