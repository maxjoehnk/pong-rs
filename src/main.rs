extern crate amethyst;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat, Pipeline,
                         PosTex, RenderBundle, Stage};
use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::config::ConfigError;

use config::GameConfig;

mod pong;
mod systems;
mod config;

const BACKGROUND_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum Axis {
    LeftPaddle,
    RightPaddle
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    use pong::Pong;

    let path = "./resources/display.ron";

    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target(BACKGROUND_COLOR, 1.0)
            .with_pass(DrawFlat::<PosTex>::new()),
    );

    let render_bundle = RenderBundle::new(pipe, Some(config));

    let game_config = build_game_config();

    let game_data = GameDataBuilder::default()
        .with_bundle(render_bundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(build_input_bundle()?)?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"]);

    let mut game = Application::build("./", Pong)?
        .with_resource(game_config.arena)
        .build(game_data)?;

    game.run();

    Ok(())
}

fn build_input_bundle() -> Result<InputBundle<Axis, String>, ConfigError> {
    let binding_path = "./resources/input.ron";

    InputBundle::<Axis, String>::new().with_bindings_from_file(binding_path)
}

fn build_game_config() -> GameConfig {
    let config_path = "./resources/config.ron";

    GameConfig::load(&config_path)
}