use bevy::prelude::*;
// use bevy_inspector_egui::Inspectable;

// region: --Common Components--

#[derive(Component, Default, Clone, Debug)]
pub struct Rotation {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Rotation {
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, }
    }
}

// endregion: --Common Components--

// region: --Character controller--

#[derive(Default, Clone, Reflect)]
#[reflect(name = "Camera Mode")]
pub enum CameraMode {
    #[default]
    Orbit,
    FirstPerson,
}

#[derive(Default, Reflect, Clone, Component)]
#[reflect(name = "Orbital Camera", Component)]   
pub struct OrbitCamera {
    pub mouse_sensitivity: f32,

    pub horizontal_angle: f32,
    pub vertical_angle: f32,

    pub inverted_x: bool,   
    pub inverted_y: bool,

    pub camera_mode: CameraMode,
    pub camera_distance: f32,

    pub camera_step: u32,
    pub camera_step_distance: u32,
    pub camera_step_lerp: f32,  
    pub camera_step_max: u32,

    pub camera_position: Vec3,  
}

impl OrbitCamera {
    pub fn default() -> Self {
        Self {
            mouse_sensitivity: 20.0,  
            camera_distance: 0.0,
            horizontal_angle: 0.0,  
            vertical_angle: 0.0,
            inverted_x: false,
            inverted_y: false,
            camera_mode: CameraMode::Orbit, 
            camera_step: 3,
            camera_step_distance: 5,
            camera_step_lerp: 0.25,
            camera_step_max: 5,       
            camera_position: Vec3::default(),
        }
    }
}


#[derive(Component, Default, Clone, Debug, Reflect)]
#[reflect(name = "Player", Component)]    
pub struct Player;

// endregion: --Character controller--


// region: --Inputs--  

#[derive(Component)] 
pub struct CameraInputs {
    pub zoom_in: KeyCode,
    pub zoom_out: KeyCode,
}

impl CameraInputs {
    pub fn default() -> Self {
        Self {
            zoom_in: KeyCode::X,
            zoom_out: KeyCode::Z,
        }
    }
}

// endregion: --Inputs--

pub enum Initialized {
    Loading,
    Init,
    Update,
}

pub struct MarchingCubeNode {
    pub state: Initialized,
}

impl Default for MarchingCubeNode {
    fn default() -> Self {
        Self {
            state: Initialized::Loading,
        }
    }
}