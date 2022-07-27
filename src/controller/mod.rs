use bevy::{prelude::*, input::mouse::MouseMotion};
use bevy_inspector_egui::*;
use crate::components::*;
 
mod camera;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, instantiate_character_controller);

        app.add_system(character_controller);
        app.add_system(camera::camera_distance::manager);
        app.add_system(camera::manager);

        app.register_type::<OrbitCamera>();
        app.register_type::<Player>();

        app.init_resource::<CameraMode>();  

        app.add_plugin(InspectorPlugin::<CameraMode>::new());
        app.register_inspectable::<CameraMode>();
    }
}


pub fn character_controller(
    mut query: ParamSet<(
        Query<(&mut Velocity, &mut Transform), With<Player>>,
        Query<(&mut Camera, &mut OrbitCamera, &mut Transform), With<OrbitCamera>>
    )>,
    mut windows: ResMut<Windows>,
    input: Res<Input<KeyCode>>,
    mut motion_evr: EventReader<MouseMotion>,
) {
    let mut position = Vec3::default();

    for (Velocity, mut Transform) in query.p0().iter_mut() {
        let Translation = &mut Transform.translation;

        if input.pressed(KeyCode::W) {
            Translation.y += 1.0;
        }

        if input.pressed(KeyCode::S) {
            Translation.y -= 1.0;
        }

        if input.pressed(KeyCode::A) {
            Translation.x -= 1.0;
        }

        if input.pressed(KeyCode::D) {
            Translation.x += 1.0;
        }

        if input.pressed(KeyCode::Q) {
            Translation.z += 1.0;
        }

        if input.pressed(KeyCode::E) {
            Translation.z -= 1.0;
        }

        // -- Toggle mouse cursor 
        let window = windows.get_primary_mut().unwrap();

        if input.pressed(KeyCode::B) {
            window.set_cursor_lock_mode(true);
            window.set_cursor_visibility(false);
        }

        if input.pressed(KeyCode::N) {
            window.set_cursor_lock_mode(false);
            window.set_cursor_visibility(true);
        }


        position.x = Translation.x;
        position.y = Translation.y;
        position.z = Translation.z; 
    }
}

// -- Spawn in the nesecary components --   
pub fn instantiate_character_controller(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    
    // -- Camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
    .insert(OrbitCamera::default())      
    .insert(Velocity::new(1.0, 0.0))
    .insert(Rotation::zero())
    .insert(CameraInputs::default());

    // -- Player
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule { 
            radius: 0.5, 
            ..Default::default()
        })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    })
    .insert(Player);
}   