use bevy::prelude::ReflectResource;
use std::hash::{DefaultHasher, Hasher};
use bevy::prelude::{Resource};
use bevy::reflect::Reflect;
use noise::{NoiseFn, Simplex};

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct TerrainConfig2d {
    octaves: usize,
    scale_width: f64,
    scale_height: f64,
    persistence: f64,
    lacunarity: f64,
}

impl TerrainConfig2d {
    pub fn new(octaves: usize, scale_width: f64, scale_height: f64, persistence: f64, lacunarity: f64) -> Self{
        Self{ octaves, scale_width, scale_height, persistence, lacunarity}
    }
}

impl Default for TerrainConfig2d {
    fn default() -> Self {
        Self::new(1, 20., 10., 0.5, 0.5)
    }
}

#[derive(Resource)]
pub struct TerrainGenerator2d {
    layers: Vec<TerrainLayer2d>,
}

impl Default for TerrainGenerator2d{
    fn default() -> Self {
        Self::new(1, &TerrainConfig2d::default())
    }
}

impl TerrainGenerator2d {
    pub fn new(seed: u32, properties: &TerrainConfig2d) -> Self {
        let mut hasher = DefaultHasher::default();
        hasher.write_u32(seed);
        let seed = hasher.finish();
        let mut layers = Vec::with_capacity(properties.octaves);

        let mut scale_width = properties.scale_width;
        let mut scale_height = properties.scale_height;

        for _i in 0..properties.octaves {
            layers.push(TerrainLayer2d::new(seed, scale_width, scale_height));
            scale_height *= properties.persistence;
            scale_width *= properties.lacunarity;
        }

        Self{ layers }
    }
    pub fn generate(&self, x: f32, y:f32, ) -> f32 {
        self.layers.iter()
            .map(|layer|{layer.generate(x,y)})
            .sum()
    }
}

static mut LAYER_ITER: u64 = 0;
struct TerrainLayer2d {
    scale_width : f64,
    scale_height : f64,
    algorithm: Simplex,
}
impl TerrainLayer2d {
    fn new(seed: u64, scale_width : f64, scale_height : f64) -> Self{
        let mut hasher = DefaultHasher::default();
        let seed = seed + unsafe { LAYER_ITER };
        unsafe{LAYER_ITER += 1}
        hasher.write_u64(seed);
        let seed = hasher.finish();

        Self {
            scale_width,
            scale_height,
            algorithm: Simplex::new(seed as u32),
        }
    }
    fn generate(&self, x: f32, z: f32) -> f32{
        let pt = [
            (x as f64) / self.scale_width,
            (z as f64) / self.scale_width,
        ];
        (self.algorithm.get(pt) * self.scale_height) as f32
    }
}