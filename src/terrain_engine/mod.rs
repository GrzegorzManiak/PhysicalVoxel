use bevy::prelude::{Plugin, App, StartupStage};

pub mod chunk;  

pub struct VoxelEnginePlugin;   

impl Plugin for VoxelEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, chunk::new);
    }
}