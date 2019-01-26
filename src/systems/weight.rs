use amethyst::core::Transform;
use amethyst::core::timing::Time;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use crate::components::Movement;
use nalgebra::Vector2;
use amethyst::renderer::VirtualKeyCode;

// You'll have to mark PADDLE_HEIGHT as public in pong.rs
use crate::seeking_warmth::{CAMERA_HEIGHT};
use crate::components::Player;
use crate::components::Weight;
//use crate::components::

pub struct WeightSystem;

impl<'s> System<'s> for WeightSystem {
    type SystemData = (
        WriteStorage<'s, Movement>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Weight>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut movements, players, weights, time): Self::SystemData) {
        for (player, movement, weight) in (&players, &mut movements, &weights).join() {
            //TODO: check for grounded player
            if true {
                movement.acceleration += weight.acceleration;
            }
        }
    }
}
