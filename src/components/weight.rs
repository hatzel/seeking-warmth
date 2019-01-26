use nalgebra::Vector2;
use amethyst::ecs::prelude::*;

pub struct Weight {
    pub acceleration: Vector2<f32>,
}

impl Weight {
    pub fn new() -> Self {
        Self {
            acceleration: Vector2::new(0.0, -9.81),
        }
    }
}

impl Component for Weight {
    type Storage = DenseVecStorage<Self>;
}
