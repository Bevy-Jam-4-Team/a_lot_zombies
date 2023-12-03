use bevy::prelude::*;
pub mod components;
mod systems;
use systems::*;


pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                setup_camera,
                spawn_floor,
            ))
            .add_systems(Update, (
                animate_light_direction,
                move_camera,
            ));
    }
}