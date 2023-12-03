use bevy::prelude::*;
mod systems;
use systems::*;


pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                setup_camera,
                spawn_floor,
                setup_lights,
            ));  
    }
}


#[derive(Component)]
pub struct MyGroundPlane;
