mod debug_utils;
mod block;

use bevy::app::{App, Startup};
use bevy::color::Color;
use bevy::DefaultPlugins;
use bevy::math::Vec3;
use bevy::pbr::{AmbientLight, PointLight};
use bevy::prelude::{Camera3d, Commands, Transform};
use bevy::utils::default;
use crate::debug_utils::fly_cam::{FlyCam, NoCameraPlayerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NoCameraPlayerPlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

pub const CHUNK_SIZE : u32 = 32;
pub const CUBE_SIZE : f32 = 1.5;

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
        FlyCam,
        Camera3d::default(),
        Transform::from_xyz(16.0, 0.0, 16.0).looking_at(target, Vec3::Y),
    ));
}