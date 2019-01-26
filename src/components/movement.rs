use nalgebra::Vector2;
use amethyst::ecs::prelude::*;

pub struct Movement {
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub max_velocity: Vector2<f32>,
    pub drag: f32,
}

impl Movement {
    pub fn new() -> Self {
        Self {
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            max_velocity: Vector2::new(200.0, 1000.0),
            drag: 0.2
        }
    }
}

impl Component for Movement {
    type Storage = DenseVecStorage<Self>;
}
