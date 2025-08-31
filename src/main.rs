mod debug_utils;
mod block;
mod chunk;
mod voxel_directions;
mod terrain;
mod chunk_udpater;

use bevy::app::{App, Startup, Update};
use bevy::color::Color;
use bevy::DefaultPlugins;
use bevy::math::Vec3;
use bevy::pbr::{AmbientLight, PointLight};
use bevy::pbr::wireframe::WireframePlugin;
use bevy::prelude::{Camera3d, Commands, Component, Transform};
use bevy::render::RenderDebugFlags;
use bevy::utils::default;
use crate::chunk::{ChunkPositionMap};
use crate::chunk_udpater::{draw_chunks, load_chunks};
use crate::debug_utils::fly_cam::{FlyCam, NoCameraPlayerPlugin};
use crate::terrain::TerrainGenerator2d;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WireframePlugin { debug_flags: RenderDebugFlags::default() })
        .add_plugins(NoCameraPlayerPlugin)
        .init_resource::<ChunkPositionMap>()
        .init_resource::<TerrainGenerator2d>()
        .add_systems(Startup, (setup_camera))
        .add_systems(Update, (load_chunks, draw_chunks))
        .run();
}

pub const CHUNK_SIZE : usize = 32;
pub const CUBE_SIZE : f32 = 1.5;

#[derive(Component)]
struct Player;

fn setup_camera(
    mut commands: Commands,
) {
    commands.insert_resource(AmbientLight{
        color: Color::WHITE,
        brightness: 200.0,
        affects_lightmapped_meshes: true,
    });

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            radius: 1000.0,
            range: 1000.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));

    let target =  Vec3::new(0., 1., 0.);
    commands.spawn((
        FlyCam, Player,
        Camera3d::default(),
        Transform::from_xyz(16.0, 0.0, 16.0).looking_at(target, Vec3::Y),
    ));
}