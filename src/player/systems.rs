use bevy::prelude::*;
use super::components::*;

pub fn spawn_player(
    mut commands: Commands,
    //mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let player = 
        SceneBundle {
            scene: asset_server.load("models/player.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            visibility: Visibility::Visible,
            ..Default::default()
        };
    commands.spawn((player, Player));
}

pub fn player_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<&mut Transform, With<Player>>,
    mut camera_q: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    if let Ok(mut player_transform) = player_q.get_single_mut() {
        let mut direction = Vec3::ZERO;

        // forward
        if keys.pressed(KeyCode::W) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keys.pressed(KeyCode::S) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keys.pressed(KeyCode::A) {
            direction += Vec3::new(0.0, 0.0, 1.0);
        }
        if keys.pressed(KeyCode::D) {
            direction += Vec3::new(0.0, 0.0, -1.0);
        }
        // make a variable for speed so that it might be modified.

        let movement = direction.normalize_or_zero() * 2.0 * time.delta_seconds();
        player_transform.translation += movement;

        let mut camera_transform = camera_q.get_single_mut().expect("Could not collect Camera3D entity");
        camera_transform.translation = Vec3::from_array([player_transform.translation.x, 10.0, player_transform.translation.z]);
    };

}