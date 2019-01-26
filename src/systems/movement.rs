use log::debug;
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

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Movement>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        );

    fn run(&mut self, (mut movements, mut transforms, time): Self::SystemData) {
        for (movement, transform) in (&mut movements, &mut transforms).join() {
            movement.velocity += movement.acceleration;
            movement.velocity.x = nalgebra::clamp(movement.velocity.x, -movement.max_velocity.x, movement.max_velocity.x);
            movement.velocity.y = nalgebra::clamp(movement.velocity.y, -movement.max_velocity.y, movement.max_velocity.y);
            let delta_pos = movement.velocity * time.delta_seconds();
            // debug!("Acceleration: {}", movement.acceleration);
            // debug!("Velocity: {}", movement.velocity);
            // debug!("Applying delta_pos {}", delta_pos);
            transform.translate_x(delta_pos.x);
            transform.translate_y(delta_pos.y);

            movement.acceleration = Vector2::new(0.0, 0.0);
        }
    }
}
