use std::f32::consts::{FRAC_PI_2, PI};
use bevy::asset::AssetId;
use bevy::math::{IVec3, Quat, Vec3};
use bevy::pbr::StandardMaterial;
use VoxelDir::{Bck, Bot, Fnt, Lft, Rgt, Top};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum VoxelDir { Fnt = 0, Top = 1, Rgt = 2, Bck = 3,  Bot = 4,  Lft = 5, }


impl VoxelDir {
    pub(crate) fn dirs<T: From<VoxelDir>>() -> [T; 6] {
        [
            T::from(Fnt),
            T::from(Top),
            T::from(Rgt),
            T::from(Bck),
            T::from(Bot),
            T::from(Lft)
        ]
    }

    pub(crate) fn opp(&self) -> VoxelDir{
        match self {
            Fnt => { Bck }
            Bck => { Fnt }
            Rgt => { Lft }
            Lft => { Rgt }
            Top => { Bot }
            Bot => { Top }
        }
    }

    pub(crate) fn idx(&self) -> usize { *self as usize }

    pub(crate) fn get(idx: usize) -> VoxelDir {
        *VoxelDir::dirs::<VoxelDir>().get(idx).unwrap()
    }
}

impl From<VoxelDir> for Vec3{
    fn from(value: VoxelDir) -> Self {
        match value {
            Fnt => {Vec3::new(0., 0., 1.)}
            Bck => {Vec3::new(0., 0., -1.)}
            Rgt => {Vec3::new(1., 0., 0.)}
            Lft => {Vec3::new(-1., 0., 0.)}
            Top => {Vec3::new(0., 1., 0.)}
            Bot => {Vec3::new(0., -1., 0.)}
        }
    }
}

impl From<VoxelDir> for IVec3{
    fn from(value: VoxelDir) -> Self {
        match value {
            Fnt => {IVec3::new(0, 0, 1)}
            Bck => {IVec3::new(0, 0, -1)}
            Rgt => {IVec3::new(1, 0, 0)}
            Lft => {IVec3::new(-1, 0, 0)}
            Top => {IVec3::new(0, 1, 0)}
            Bot => {IVec3::new(0, -1, 0)}
        }
    }
}

impl From<VoxelDir> for Quat{
    fn from(value: VoxelDir) -> Self {
        match value {
            Fnt => {Quat::from_axis_angle(Vec3::Y, 0.0)}
            Bck => {Quat::from_axis_angle(Vec3::Y, PI)}
            Rgt => {Quat::from_axis_angle(Vec3::Y, FRAC_PI_2)}
            Lft => {Quat::from_axis_angle(Vec3::Y, -FRAC_PI_2)}
            Top => {Quat::from_axis_angle(Vec3::X, -FRAC_PI_2)}
            Bot => {Quat::from_axis_angle(Vec3::X, FRAC_PI_2)}
        }
    }
}