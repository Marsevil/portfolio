use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_graph,
        render_resource::{
            BindGroup, BindGroupEntries, BindGroupLayout, CachedPipelineState,
            CachedRenderPipelineId, FragmentState, LoadOp, Operations, PipelineCache,
            RenderPassColorAttachment, RenderPassDescriptor, RenderPipelineDescriptor, StoreOp,
            TextureViewDescriptor,
        },
        renderer::RenderDevice,
        texture::GpuImage,
    },
};
use wgpu_types::{ColorTargetState, ColorWrites, MultisampleState, PrimitiveState, TextureFormat};

use super::{shader, SkyboxImage, SkyboxPipelineUniformBuffers};

const PIPELINE_LABEL: &str = "skybox_pipeline_init";

#[derive(Resource)]
pub struct SkyboxPipelineBindGroup {
    bind_group: BindGroup,
}

#[derive(Debug, Clone, Copy)]
enum SkyboxPipelineState {
    Loading,
    Init,
}
impl SkyboxPipelineState {
    pub fn next(self) -> Self {
        match self {
            Self::Loading => Self::Init,
            Self::Init => Self::Init,
        }
    }
}

#[derive(Resource)]
pub struct SkyboxPipeline {
    bind_group_layout: BindGroupLayout,
    init_pipeline: CachedRenderPipelineId,
}
impl FromWorld for SkyboxPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let bind_group_layout = shader::Uniforms::bind_group_layout(render_device);
        let shader = world.resource::<AssetServer>().load(shader::SHADER_PATH);
        let pipeline_cache = world.resource_mut::<PipelineCache>();
        let init_pipeline = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
            label: Some(PIPELINE_LABEL.into()),
            layout: vec![bind_group_layout.clone()],
            push_constant_ranges: vec![],
            vertex: bevy::core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state(),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            fragment: Some(FragmentState {
                shader,
                shader_defs: vec![],
                entry_point: shader::FRAGMENT_ENTRY_POINT.into(),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::Rgba8Unorm,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
            }),
            zero_initialize_workgroup_memory: false,
        });

        Self {
            bind_group_layout,
            init_pipeline,
        }
    }
}

pub struct SkyboxPipelineNode {
    state: SkyboxPipelineState,
}
impl Default for SkyboxPipelineNode {
    fn default() -> Self {
        Self {
            state: SkyboxPipelineState::Loading,
        }
    }
}
impl render_graph::Node for SkyboxPipelineNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<SkyboxPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        match self.state {
            SkyboxPipelineState::Loading => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_render_pipeline_state(pipeline.init_pipeline)
                {
                    self.state = self.state.next();
                }
            }
            SkyboxPipelineState::Init => self.state = self.state.next(),
        }
    }

    fn run<'w>(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext<'w>,
        world: &'w World,
    ) -> Result<(), render_graph::NodeRunError> {
        let pipeline = world.resource::<SkyboxPipeline>();
        let skybox_image = world.resource::<SkyboxImage>();
        let pipeline_bind_group = &world.resource::<SkyboxPipelineBindGroup>();

        let gpu_images = world.resource::<RenderAssets<GpuImage>>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let image = gpu_images.get(&skybox_image.0).unwrap();

        let SkyboxPipelineState::Init = self.state else {
            return Ok(());
        };

        for face_idx in 0..6 {
            let view = image.texture.create_view(&TextureViewDescriptor {
                base_array_layer: face_idx,
                array_layer_count: Some(1),
                ..Default::default()
            });

            let mut pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
                color_attachments: &[RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(wgpu_types::Color::BLACK),
                        store: StoreOp::Store,
                    },
                }
                .into()],
                ..Default::default()
            });

            pass.set_bind_group(0, &pipeline_bind_group.bind_group, &[]);

            let init_pipeline = pipeline_cache
                .get_render_pipeline(pipeline.init_pipeline)
                .unwrap();
            pass.set_render_pipeline(init_pipeline);

            pass.draw(0..3, 0..1);
        }

        Ok(())
    }
}

pub fn queue_bind_group(
    mut commands: Commands,
    pipeline: Res<SkyboxPipeline>,
    pipeline_bufs: Res<SkyboxPipelineUniformBuffers>,
    render_device: Res<RenderDevice>,
) {
    let bind_group = render_device.create_bind_group(
        Some("SkyboxPipelineBindGroup"),
        &pipeline.bind_group_layout,
        &BindGroupEntries::single(pipeline_bufs.as_entire_binding()),
    );

    commands.insert_resource(SkyboxPipelineBindGroup { bind_group });
}
