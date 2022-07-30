use bevy::{
    prelude::*,
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_asset::RenderAssets,
        render_graph::{self, RenderGraph},
        render_resource::*,
        renderer::{RenderContext, RenderDevice},
        RenderApp, RenderStage,
    },
    window::WindowDescriptor,
};
use std::borrow::Cow;

use crate::components::*;

pub struct ComputePlugin;

impl Plugin for ComputePlugin {
    fn build(&self, app: &mut App) {
        // Extract the game of life image resource from the main world into the render world
        // for operation on by the compute shader and display on the sprite.

        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<MarchingCubePipeline>();
    }
}

pub struct MarchingCubePipeline {
    // sim_pipeline: ComputePipeline,
    init_pipeline: CachedComputePipelineId,
}

impl FromWorld for MarchingCubePipeline {
    fn from_world(world: &mut World) -> Self {

        let bind_group_layout = world.resource::<RenderDevice>()
        .create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::StorageTexture {
                    access: StorageTextureAccess::ReadWrite,
                    format: TextureFormat::Rgba8Unorm,
                    view_dimension: TextureViewDimension::D2,
                },
                count: None,
            }],
        });

        let shader = world.resource::<AssetServer>().load("./shaders/marching_cube.wgsl");

        let mut pipeline_cache = world.resource_mut::<PipelineCache>();

        let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: Some(vec![bind_group_layout.clone()]),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
        });

        // let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        //     label: None,
        //     layout: Some(vec![texture_bind_group_layout.clone()]),
        //     shader,
        //     shader_defs: vec![],
        //     entry_point: Cow::from("update"),
        // });

        MarchingCubePipeline {
            init_pipeline,
        }
    }
}


const SIZE: (u32, u32) = (1280, 720);
const WORKGROUP_SIZE: u32 = 8;

impl render_graph::Node for MarchingCubeNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<MarchingCubePipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // if the corresponding pipeline has loaded, transition to the next stage
        match self.state {
            Initialized::Loading => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline)
                { self.state = Initialized::Init; }
            }
            Initialized::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline)
                { self.state = Initialized::Update; }
            }
            Initialized::Update => {}
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {

        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<MarchingCubePipeline>();

        let mut pass = render_context.command_encoder
            .begin_compute_pass(&ComputePassDescriptor::default());

        // select the pipeline based on the current state
        match self.state {
            Initialized::Loading => {}
            Initialized::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
            Initialized::Update => {}
        }

        Ok(())
    }
}