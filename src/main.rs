use bevy::prelude::*;

pub mod setup;
use setup::*;

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins,
                SetupPlugin
            )
        )
        .run();
}