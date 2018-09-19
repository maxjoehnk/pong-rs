use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use pong::{Paddle, Side, PADDLE_HEIGHT};
use config::ArenaConfig;
use Axis;

pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<Axis, String>>,
        Read<'s, ArenaConfig>
    );

    fn run(&mut self, (mut transforms, paddles, input, arena_config): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = input.axis_value(&Axis::from(&paddle.side));
            if let Some(mv_amount) = movement {
                let scaled_amount = 1.2 * mv_amount as f32;
                transform.translation[1] = (transform.translation[1] + scaled_amount)
                    .min(arena_config.height - PADDLE_HEIGHT * 0.5)
                    .max(PADDLE_HEIGHT * 0.5);
            }
        }
    }
}