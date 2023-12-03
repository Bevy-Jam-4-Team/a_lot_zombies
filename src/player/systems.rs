use bevy::prelude::*;

use super::components::*;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //asset_server: Res<AssetServer>,
) {
    /*let player = PbrBundle {
        mesh: asset_server.load("models/Player.gltf#Scene0"),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    };*/
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            material: materials.add(Color::RED.into()),
            ..default()
        },
        Player
    ));
}