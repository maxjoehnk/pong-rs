use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::{Vector3, Matrix4};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Event, PngFormat, Projection, Sprite, Texture, TextureHandle,
    VirtualKeyCode, WithSpriteRender,
};
use config::ArenaConfig;
use Axis;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

const SPRITESHEET_SIZE: (f32, f32) = (8.0, 16.0);

pub struct Pong;

impl<'a, 'b> State<GameData<'a, 'b>> for Pong {
    fn handle_event(&mut self, _: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            Trans::Quit
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }

    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        world.register::<Paddle>();

        let spritesheet = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "texture/pong_spritesheet.png",
                PngFormat,
                Default::default(),
                (),
                &texture_storage
            )
        };

        initialise_paddles(world, spritesheet);
        initialise_camera(world);
    }
}

fn initialise_camera(world: &mut World) {
    let (arena_height, arena_width) = {
        let config = &world.read_resource::<ArenaConfig>();
        (config.height, config.width)
    };

    world.create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            arena_width,
            arena_height,
            0.0,
        )))
        .with(GlobalTransform(
            Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into()
        ))
        .build();
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

impl<'a> From<&'a Side> for Axis {
    fn from(side: &'a Side) -> Axis {
        match side {
            Side::Left => Axis::LeftPaddle,
            Side::Right => Axis::RightPaddle,
        }
    }
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: 1.0,
            height: 1.0,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_paddles(world: &mut World, spritesheet: TextureHandle) {
    let (arena_height, arena_width) = {
        let config = &world.read_resource::<ArenaConfig>();
        (config.height, config.width)
    };

    let sprite = Sprite {
        left: 0.0,
        right: PADDLE_WIDTH,
        top: 0.0,
        bottom: PADDLE_HEIGHT
    };

    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // Correctly position the paddles.
    let y = arena_height / 2.0;
    left_transform.translation = Vector3::new(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.translation = Vector3::new(arena_width - PADDLE_WIDTH * 0.5, y, 0.0);

    // Create a left plank entity.
    world
        .create_entity()
        .with_sprite(&sprite, spritesheet.clone(), SPRITESHEET_SIZE)
        .expect("Failed to add sprite render on paddle")
        .with(Paddle::new(Side::Left))
        .with(GlobalTransform::default())
        .with(left_transform)
        .build();

    // Create right plank entity.
    world
        .create_entity()
        .with_sprite(&sprite, spritesheet.clone(), SPRITESHEET_SIZE)
        .expect("Failed to add sprite render on paddle")
        .with(Paddle::new(Side::Right))
        .with(GlobalTransform::default())
        .with(right_transform)
        .build();
}

