use bevy::{prelude::{With, Query, Camera, Transform, EventReader, ParamSet }, input::mouse::MouseMotion};
use crate::components::{OrbitCamera, CameraMode, Player};

pub mod orbital_camera;
pub mod camera_distance;

pub fn manager(
    mut query: ParamSet<(
        Query<(&mut Camera, &mut OrbitCamera, &mut Transform), With<OrbitCamera>>,
        Query<&mut Transform, With<Player>>,    
    )>,
    mut motion_evr: EventReader<MouseMotion>,
) {
    let mut player_transform = Transform::default();    

    for transform in query.p1().iter_mut() {
        player_transform = *transform;
    }

    for (_, mut orbital_camera, mut transform) in query.p0().iter_mut() {

        // -- Get the mouse position
        for ev in motion_evr.iter() {
            let x = invert(ev.delta.x / 100.0, orbital_camera.inverted_x);
            let y = invert(ev.delta.y / 100.0, orbital_camera.inverted_y);

            // -- X should be able to loop an infinite amount of times.
            orbital_camera.horizontal_angle = 
                clamp_overflow(orbital_camera.horizontal_angle + x * orbital_camera.mouse_sensitivity, 0.0, 360.0);

            // -- Y should be clamped to a minimum of -90.0 and a maximum of 90.0 due to the camera going upside down --
            orbital_camera.vertical_angle = 
                (orbital_camera.vertical_angle - y * orbital_camera.mouse_sensitivity).min(89.0).max(-89.0);
        }

        match orbital_camera.camera_mode {
            CameraMode::Orbit => {
                orbital_camera::manager(
                    &mut orbital_camera,
                    &mut transform,
                    player_transform
                );
            },

            CameraMode::FirstPerson => {

            },  
        }
    }
}

// -- EG: Value = 400, max = 360, min = 0, result = 0.0 -- 
fn clamp_overflow(value: f32, min: f32, max: f32) -> f32 {
    if value > max { return min; }
    if value < min { return max; }
    return value;
}

fn invert(x: f32, trig: bool) -> f32 { 
    if trig { return -x; }
    return x;
}