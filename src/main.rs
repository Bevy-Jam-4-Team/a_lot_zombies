use bevy::prelude::*;

pub mod setup;
pub mod player;
use setup::*;
use player::*;

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins,
                SetupPlugin,
                PlayerPlugin,
            )
        )
        .run();
}