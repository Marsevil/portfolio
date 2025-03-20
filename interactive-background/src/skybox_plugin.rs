use bevy::{
    core_pipeline::{
        core_3d::graph::{Core3d, Node3d},
        fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    },
    prelude::*,
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_asset::RenderAssets,
        render_graph::{self, RenderGraphApp, RenderLabel},
        render_resource::{
            BindGroup, BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, Buffer,
            BufferDescriptor, CachedPipelineState, CachedRenderPipelineId, ColorTargetState,
            ColorWrites, FragmentState, LoadOp, MultisampleState, Operations, PipelineCache,
            PrimitiveState, RenderPassColorAttachment, RenderPassDescriptor,
            RenderPipelineDescriptor, ShaderType, StoreOp, TextureFormat, TextureViewDescriptor,
        },
        renderer::{RenderDevice, RenderQueue},
        texture::GpuImage,
        Render, RenderApp, RenderSet,
    },
};
use wgpu_types::{BindingType, BufferBindingType, BufferUsages, ShaderStages};

const PIPELINE_INIT_LABEL: &str = "skybox_pipeline_init";
const SHADER_PATH: &str = "skybox.wgsl";

#[derive(Clone, Deref, Resource, ExtractResource)]
pub struct SkyboxImage(pub Handle<Image>);

#[derive(Resource)]
struct SkyboxPipelineBindGroup {
    bind_group: BindGroup,
    buffer: Buffer,
}

#[derive(Default, Debug, Clone, Resource, ExtractResource, ShaderType)]
pub struct SkyboxPipelineParameters {
    time: f32,
    _webgl2_padding: Vec3,
}
impl SkyboxPipelineParameters {
    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(
            Some("SkyboxPipelineParametersLayout"),
            &BindGroupLayoutEntries::single(
                ShaderStages::FRAGMENT,
                BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: Some(Self::min_size()),
                },
            ),
        )
    }

    fn as_byte_slice(&self) -> &[u8] {
        // Safe because lock immutable reference and the struct can be Copy
        unsafe {
            core::slice::from_raw_parts(
                core::ptr::from_ref(self) as *const u8,
                core::mem::size_of::<Self>(),
            )
        }
    }
}

#[derive(Resource)]
struct SkyboxPipeline {
    bind_group_layout: BindGroupLayout,
    init_pipeline: CachedRenderPipelineId,
}
impl FromWorld for SkyboxPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let bind_group_layout = SkyboxPipelineParameters::bind_group_layout(render_device);
        let shader = world.resource::<AssetServer>().load(SHADER_PATH);
        let pipeline_cache = world.resource_mut::<PipelineCache>();
        let init_pipeline = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
            label: Some(PIPELINE_INIT_LABEL.into()),
            layout: vec![bind_group_layout.clone()],
            push_constant_ranges: vec![],
            vertex: fullscreen_shader_vertex_state(),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            fragment: Some(FragmentState {
                shader,
                shader_defs: vec![],
                entry_point: "init".into(),
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

#[derive(Debug, Clone, Copy)]
enum SkyboxPipelineState {
    Loading,
    Init(u32),
}
impl SkyboxPipelineState {
    pub fn next(self) -> Self {
        match self {
            Self::Loading => Self::Init(0),
            Self::Init(idx) => Self::Init((idx + 1) % 6),
        }
    }
}

struct SkyboxPipelineNode {
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

        let render_queue = world.resource::<RenderQueue>();
        let uniforms = world.resource::<SkyboxPipelineParameters>();
        let bindings = world.resource::<SkyboxPipelineBindGroup>();
        render_queue.write_buffer(&bindings.buffer, 0, uniforms.as_byte_slice());

        match self.state {
            SkyboxPipelineState::Loading => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_render_pipeline_state(pipeline.init_pipeline)
                {
                    self.state = self.state.next();
                }
            }
            SkyboxPipelineState::Init(_) => self.state = self.state.next(),
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

        let SkyboxPipelineState::Init(face_idx) = self.state else {
            return Ok(());
        };

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
                    load: LoadOp::Clear(wgpu_types::Color::GREEN),
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

        Ok(())
    }
}

pub struct SkyboxPlugin;

fn queue_bind_group(
    mut commands: Commands,
    pipeline: Res<SkyboxPipeline>,
    render_device: Res<RenderDevice>,
) {
    let buffer = render_device.create_buffer(&BufferDescriptor {
        label: Some("SkyboxPipelineUniformBuffer"),
        size: SkyboxPipelineParameters::min_size().try_into().unwrap(),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let bind_group = render_device.create_bind_group(
        Some("SkyboxPipelineBindGroup"),
        &pipeline.bind_group_layout,
        &BindGroupEntries::single(buffer.as_entire_binding()),
    );

    commands.insert_resource(SkyboxPipelineBindGroup { bind_group, buffer });
}

fn update_uniforms(mut uniforms: ResMut<SkyboxPipelineParameters>, time: Res<Time>) {
    uniforms.time += time.elapsed_secs();
}

#[derive(Debug, Clone, RenderLabel, PartialEq, Eq, Hash)]
struct SkyboxPipelineLabel;

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractResourcePlugin::<SkyboxImage>::default(),
            ExtractResourcePlugin::<SkyboxPipelineParameters>::default(),
        ));
        app.add_systems(Update, update_uniforms);

        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(Render, queue_bind_group.in_set(RenderSet::Queue));

        render_app
            .add_render_graph_node::<SkyboxPipelineNode>(Core3d, SkyboxPipelineLabel)
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
        render_app.init_resource::<SkyboxPipeline>();
    }
}
