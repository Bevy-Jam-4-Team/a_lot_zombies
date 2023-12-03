use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*, render::camera::ScalingMode};


pub fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn(
        Camera3dBundle {
            projection: OrthographicProjection {
                scale: 3.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                ..default()
            }
            .into(),
            transform: Transform::from_xyz(0.1, 0.5, 0.01)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ); 
}

pub fn setup_lights (
    mut commands: Commands,
) {
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
    /*commands.spawn((
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
    ));*/
}

#[derive(Component)]
pub struct GroundPlane;

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

    commands.spawn((floor, GroundPlane));
}