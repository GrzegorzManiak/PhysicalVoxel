use bevy::{prelude::*, math::vec3};
use bevy_easings::EasingsPlugin;

mod components;
mod controller;
mod terrain_engine;   

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
        transform: Transform { translation: vec3(0.0, -1.0, 0.0), ..Default::default() },
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    })
    // .insert(Collider::cuboid(100.0, 0.1, 100.0))
    .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -1.0, 0.0)));


    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 15500.0,
            shadows_enabled: true,
            range: 500.0,
            radius: 40.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 50.0, 14.0),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())

        .add_plugin(controller::CharacterControllerPlugin)
        .add_plugin(terrain_engine::VoxelEnginePlugin)
        .add_plugin(terrain_engine::chunk::marching_cube::ComputePlugin)
        .insert_resource(Msaa { samples: 4 })
        .add_plugin(EasingsPlugin)
        .add_startup_system(setup)  
        // .add_plugin(DebugLinesPlugin::default())  
        // .add_plugin(EditorPlugin)
        .run();
}
