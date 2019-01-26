use amethyst::ecs::prelude::*;

pub const PLAYER_HEIGHT: f32 = 34.0;
pub const PLAYER_WIDTH: f32 = 19.0;
pub const PLAYER_VELOCITY: f32 = 50.0;

pub struct Player {
    pub velocity: f32,
    pub width: f32,
    pub height: f32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            velocity: 1.0,
            width: 1.0,
            height: 1.0,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
