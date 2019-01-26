use amethyst::core::Transform;
use amethyst::core::timing::Time;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use crate::components::Movement;
use nalgebra::Vector2;

// You'll have to mark PADDLE_HEIGHT as public in pong.rs
use crate::seeking_warmth::{CAMERA_HEIGHT};
use crate::components::Player;
//use crate::components::

pub struct PlayerControlSystem;

impl<'s> System<'s> for PlayerControlSystem {
    type SystemData = (
        WriteStorage<'s, Movement>,
        ReadStorage<'s, Player>,
        Read<'s, Time>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut movements, players, time, input): Self::SystemData) {
        for (player, movement) in (&players, &mut movements).join() {
            let opt_movement_val = input.axis_value("player_x");
            if let Some(movement_val) = opt_movement_val {
                // TODO: use player velocity again
                movement.acceleration += Vector2::new(player.velocity * movement_val as f32, 0.0);
                if movement.acceleration.y != 0.0 {
                    println!("Adding to acceleration {}", player.velocity * movement_val as f32);
                    println!("New accelleration {}", movement.acceleration);
                }
            }
        }
    }
}
