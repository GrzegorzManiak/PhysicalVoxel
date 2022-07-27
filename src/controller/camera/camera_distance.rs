use bevy::{prelude::{KeyCode, Res, With, Query}, input::Input};
use crate::components::{OrbitCamera, CameraMode, CameraInputs};

pub fn manager(
    mut camera: Query<(&mut OrbitCamera, &mut CameraInputs), With<OrbitCamera>>,
    input: Res<Input<KeyCode>>,
) {
    for (mut orbit_camera, inputs) in camera.iter_mut() {

        // -- Camera distance increment
        if input.just_pressed(inputs.zoom_out) {

            if orbit_camera.camera_step < orbit_camera.camera_step_max {
                // -- Step the camera distance up
                orbit_camera.camera_step += 1;

                // -- Camera is now in orbit mode
                orbit_camera.camera_mode = CameraMode::Orbit;
            } else {

                // -- Step the camera distance to the max
                orbit_camera.camera_step = orbit_camera.camera_step_max;
            }
        }

        // -- Camera distance decrement 
        if input.just_pressed(inputs.zoom_in) 
        {
            if orbit_camera.camera_step < 1 {
                // -- Dont decrement the camera distance below 0
                orbit_camera.camera_step = 0;

                // -- Camera is now in first person mode
                orbit_camera.camera_mode = CameraMode::FirstPerson;
            } else {

                // -- Step the camera distance down
                orbit_camera.camera_step -= 1;
            }
        }

        // -- Lerp camera distance
        orbit_camera.camera_distance = bevy_easings::Lerp::lerp(
            &orbit_camera.camera_distance,
            &((orbit_camera.camera_step * orbit_camera.camera_step_distance) as f32),
            &orbit_camera.camera_step_lerp,
        );

    }
}