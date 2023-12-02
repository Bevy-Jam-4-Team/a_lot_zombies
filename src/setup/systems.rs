use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::gltf::Gltf;

use super::components::*;
use super::resources::*;

pub fn setup_camera(
    mut commands: Commands
) {
    commands.spawn((
        Camera3dBundle {
            projection: OrthographicProjection {
                // For this example, let's make the screen/window height
                // correspond to 16.0 world units.
                scaling_mode: ScalingMode::FixedVertical(16.0),
                ..default()
            }.into(),
            // the distance doesn't really matter for orthographic,
            // it should look the same (though it might affect
            // shadows and clipping / culling)
            transform: Transform::from_xyz(10.0, 12.0, 16.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MyCameraMarker,
    ));
}

pub fn load_gltf(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    let gltf = ass.load("models/landscape.gltf");
    commands.insert_resource(MyAssetPack(gltf));
}

pub fn spawn_gltf_objects(
    mut commands: Commands,
    my: Res<MyAssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    // if the GLTF has loaded, we can navigate its contents
    if let Some(gltf) = assets_gltf.get(&my.0) {
        // spawn the first scene in the file
        commands.spawn(SceneBundle {
            scene: gltf.scenes[0].clone(),
            ..Default::default()
        });

        /*// spawn the scene named "YellowCar"
        commands.spawn(SceneBundle {
            scene: gltf.named_scenes["YellowCar"].clone(),
            transform: Transform::from_xyz(1.0, 2.0, 3.0),
            ..Default::default()
        });

        // PERF: the `.clone()`s are just for asset handles, don't worry :)
        */
    }
}