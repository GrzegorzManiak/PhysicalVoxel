use bevy::prelude::*;
use bevy_inspector_egui::*;
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier3d::prelude::{Collider, Velocity, GravityScale, Sleeping, Ccd, RigidBody, LockedAxes};
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
        Query<&mut OrbitCamera, With<OrbitCamera>>,
    )>,
    mut windows: ResMut<Windows>,
    mut lines: ResMut<DebugLines>,
    input: Res<Input<KeyCode>>,
) {
    let mut camera = OrbitCamera::default();

    for orbit_camera in query.p1().iter_mut() {
        camera = orbit_camera.clone();
    }       


    for (mut velocity, mut transform) in query.p0().iter_mut() {
        
        let mut translation = transform.translation;

        // -- START DEBUG --
        let mut orbit = camera::orbital_camera::calculate_orbit(
            transform.rotation.x.to_degrees() + 90.0,
            0.0,
            10.0,
        );

        orbit.x += translation.x;
        orbit.y += translation.y;
        orbit.z += translation.z;

        lines.line_colored(
            translation,
            orbit, 
            0.0,
            Color::CYAN
        );
        // -- END DEBUG --


        if input.pressed(KeyCode::W) {
            velocity.linvel.x += 1.5;
        }

        if input.pressed(KeyCode::S) {
            velocity.linvel.x = -1.0;
        }

        if input.pressed(KeyCode::A) {
            velocity.linvel.z = 1.5;
        }

        if input.pressed(KeyCode::D) {
            velocity.linvel.z = -1.5;
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
        ..default()
    })
    .insert(OrbitCamera::default())      
    .insert(Rotation::zero())
    .insert(CameraInputs::default());

    // -- Player
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule { 
            radius: 0.5, 
            depth: 1.0, 
            ..Default::default()
        })),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        material: materials.add(Color::rgb(0.1, 0.2, 0.6).into()),
        ..default()
    })
    .insert(Player)
    .insert(Collider::capsule(
        Vec3::new(0.0, -0.5, 0.0),
        Vec3::new(0.0, 0.5, 0.0),
        0.5,
    ))
    .insert(RigidBody::Dynamic)
    .insert(Velocity {
        linvel: Vec3::new(1.0, 2.0, 3.0),
        angvel: Vec3::new(0.0, 0.0, 0.0),
    })
    .insert(GravityScale(0.7))
    .insert(Sleeping::disabled())
    .insert(Ccd::enabled())
    .insert(LockedAxes::ROTATION_LOCKED);
}   