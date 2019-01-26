use log::debug;
use amethyst::core::Transform;
use amethyst::core::timing::Time;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use crate::components::Movement;
use nalgebra::Vector2;

use crate::components::AABB;

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        ReadStorage<'s, AABB>,
        ReadStorage<'s, Transform>,
        );

    fn run(&mut self, (bounding_boxes, transforms): Self::SystemData) {
        for (bb, trans) in (&bounding_boxes, &transforms).join() {
            for (other_bb, other_trans) in (&bounding_boxes, &transforms).join() {
                bb.collides(trans, other_bb, other_trans);
            }
        }
    }
}
