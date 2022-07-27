use bevy::{prelude::Transform, math::Vec3};

use crate::components::OrbitCamera;

pub fn manager(
    orbital_camera: &OrbitCamera,
    transform: &mut Transform,
    player_transform: Transform,
) {
    // -- Get the positions
    let camera_translation = &mut transform.translation;
    let player_translation = &player_transform.translation;

    // -- Calculate the camera oribit position --
    let orbit = calculate_orbit(
        orbital_camera.horizontal_angle, 
        orbital_camera.vertical_angle,
        orbital_camera.camera_distance,
    );

    // -- Set the camera position with offsets --
    camera_translation.x = orbit.x + player_translation.x;
    camera_translation.y = orbit.y + player_translation.y;
    camera_translation.z = orbit.z + player_translation.z;

    // -- Look at the player --
    transform.look_at(*player_translation, Vec3::Y);
}


// -- THis function is used to determine the position of the camera on
// the xz and xy planes based on the given angle and distance.
// https://math.stackexchange.com/questions/989900/calculate-x-y-z-from-two-specific-degrees-on-a-sphere
// (cosϕ×sinθ,cosϕ×cosθ,sinϕ)
fn calculate_orbit(degrees_x: f32, degrees_y: f32, distance: f32) -> Vec3 {
    let radians_y = degrees_x.to_radians();
    let radians_x = degrees_y.to_radians();
    
    let x = distance * radians_x.cos() * radians_y.sin();
    let y = distance * radians_x.sin();
    let z = distance * radians_x.cos() * radians_y.cos();
    
    Vec3::new(x, y, z)
}
