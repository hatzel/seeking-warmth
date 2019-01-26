use amethyst::ecs::prelude::*;
use amethyst::core::Transform;

pub struct AABB {
    pub width: f32,
    pub height: f32,
}

impl AABB {
    fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
        }
    }

    fn collides(&self, transform: &Transform, other: &Self, other_transform: &Transform) -> bool {
        let delta_x = (transform.translation().x - other_transform.translation().x).abs();
        let delta_y = (transform.translation().y - other_transform.translation().y).abs();
        if delta_x <= (self.height + other.height) * 0.5 {
            true
        } else if delta_y <= (self.width + other.width) * 0.5 {
            true
        } else {
            false
        }
    }
}

impl Component for AABB {
    type Storage = DenseVecStorage<Self>;
}
