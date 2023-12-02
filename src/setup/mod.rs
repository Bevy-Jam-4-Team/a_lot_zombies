use bevy::prelude::*;
pub mod components;
pub mod resources;
mod systems;

use systems::*;
use resources::*;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource(MyAssetPack)
            .add_systems(Startup, (
                setup_camera,
                load_gltf,
                spawn_gltf_objects
            ));
    }
}