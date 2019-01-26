use amethyst::core::Transform;
use amethyst::core::timing::Time;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

// You'll have to mark PADDLE_HEIGHT as public in pong.rs
use crate::seeking_warmth::{CAMERA_HEIGHT};
use crate::components::Player;
//use crate::components::

pub struct PlayerMovementSystem;

impl<'s> System<'s> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, Time>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, players, time, input): Self::SystemData) {
        for (player, transform) in (&players, &mut transforms).join() {
            let opt_movement = input.axis_value("player_x");
            if let Some(movement) = opt_movement {
                transform.translate_x(player.velocity * time.delta_seconds() * movement as f32);
            }
        }
    }
}