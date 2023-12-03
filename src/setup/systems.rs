use bevy::prelude::*;
use std::f32::consts::*;
use bevy::pbr::{CascadeShadowConfigBuilder, NotShadowCaster};

pub fn setup_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 22.0, 0.0)
                .looking_at(Vec3::new(0.5, 0.0, 0.0), Vec3::ZERO),
            ..default()
        }
    );
    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 0.3,
        maximum_distance: 3.0,
        ..default()
    }
    .build();
    // Sun
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0) 
            .looking_at(Vec3::new(0.0, -45.0, 0.0), Vec3::Y),
        cascade_shadow_config,
        ..default()
    });
    // Sky
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(10.0, 10.0, 10.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("888888").unwrap(),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(20.0)),
            ..default()
        },
        NotShadowCaster,
    ));
}

pub fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.0))),
        material: materials.add(Color::DARK_GREEN.into()),
        ..default()
    };

    commands.spawn(floor);

}

pub fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.elapsed_seconds() * PI / 5.0,
            -FRAC_PI_4,
        );
    }
}

/*
fn cursor_position( q_windows: Query<&Window, With<PrimaryWindow>>, ) {
    if let Some(position) = q_windows.single().cursor_position() {
        println!("Cursor is inside the primary window, at {:?}", position);
    } else {
        println!("Cursor is not in the game window.");
    }
}
fn cursor_events(mut cursor_evr: EventReader<CursorMoved>, ) {
    for ev in cursor_evr.read() {
        println!(
            "New cursor position: X: {}, Y: {}, in Window ID: {:?}",
            ev.position.x, ev.position.y, ev.window
        );
    }
}
*/
pub fn move_camera (
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if let Ok(mut transform) = camera_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Q) {
            direction += Vec3::new(0.0, 0.0, 1.0);
        }
        if keyboard_input.pressed(KeyCode::E) {
            direction += Vec3::new(0.0, 0.0, -1.0);
        }
        
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        
        transform.translation += direction;

        if keyboard_input.just_pressed(KeyCode::Z) {
            transform.rotation = Quat::from_rotation_z(90.0) + Quat::from_rotation_x(90.0)
        }
        if keyboard_input.just_pressed(KeyCode::C) {
            transform.rotation = Quat::from_rotation_z(0.0)
        }

        if keyboard_input.just_pressed(KeyCode::R) {
            println!("(x: {0}, y: {1}, z: {2})", transform.translation.x, transform.translation.y, transform.translation.z);
            println!("{:?}", transform.rotation)
        }
    }
}