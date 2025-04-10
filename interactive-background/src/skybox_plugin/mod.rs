mod render;
mod shader;

use bevy::{
    core_pipeline::core_3d::graph::{Core3d, Node3d},
    prelude::*,
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_graph::{RenderGraphApp, RenderLabel},
        render_resource::{Buffer, BufferDescriptor, ShaderType},
        renderer::{RenderDevice, RenderQueue},
        Render, RenderApp, RenderSet,
    },
};
use wgpu_types::BufferUsages;

#[derive(Clone, Deref, Resource, ExtractResource)]
pub struct SkyboxImage(pub Handle<Image>);

#[derive(Clone, Resource, Deref, ExtractResource)]
struct SkyboxPipelineUniformBuffers {
    buffer: Buffer,
}

pub struct SkyboxPlugin;

fn init_resources(mut cmd: Commands) {
    cmd.insert_resource(shader::Uniforms::default());
}

fn update_uniforms(mut uniforms: ResMut<shader::Uniforms>, time: Res<Time>) {
    uniforms.time += time.elapsed_secs();
}

fn init_uniform_buffers(mut cmd: Commands, render_device: Res<RenderDevice>) {
    let buffer = render_device.create_buffer(&BufferDescriptor {
        label: Some("SkyboxPipelineUniformBuffer"),
        size: shader::Uniforms::min_size().into(),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    cmd.insert_resource(SkyboxPipelineUniformBuffers { buffer });
}

fn update_uniform_buffers(
    uniforms: Res<shader::Uniforms>,
    bufs: Res<SkyboxPipelineUniformBuffers>,
    render_queue: Res<RenderQueue>,
) {
    render_queue.write_buffer(&bufs, 0, uniforms.as_byte_slice());
}

#[derive(Debug, Clone, RenderLabel, PartialEq, Eq, Hash)]
struct SkyboxPipelineLabel;

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractResourcePlugin::<SkyboxImage>::default(),
            ExtractResourcePlugin::<shader::Uniforms>::default(),
            ExtractResourcePlugin::<SkyboxPipelineUniformBuffers>::default(),
        ))
        .add_systems(Startup, (init_resources, init_uniform_buffers))
        .add_systems(Update, (update_uniforms, update_uniform_buffers));

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_systems(Render, render::queue_bind_group.in_set(RenderSet::Queue))
            .add_render_graph_node::<render::SkyboxPipelineNode>(Core3d, SkyboxPipelineLabel)
            .add_render_graph_edges(
                Core3d,
                (
                    Node3d::Tonemapping,
                    SkyboxPipelineLabel,
                    Node3d::EndMainPassPostProcessing,
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<render::SkyboxPipeline>();
    }
}
