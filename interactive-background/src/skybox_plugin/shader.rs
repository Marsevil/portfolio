use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResource,
        render_resource::{BindGroupLayout, BindGroupLayoutEntries, ShaderType},
        renderer::RenderDevice,
    },
};
use wgpu_types::{BindingType, BufferBindingType, ShaderStages};

pub const SHADER_PATH: &str = "skybox.wgsl";
pub const FRAGMENT_ENTRY_POINT: &str = "init";

#[derive(Default, Debug, Clone, Resource, ExtractResource, ShaderType)]
pub struct Uniforms {
    pub time: f32,
    _webgl2_padding: Vec3,
}
impl Uniforms {
    pub fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
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

    pub fn as_byte_slice(&self) -> &[u8] {
        // Safe because lock immutable reference and the struct can be Copy
        unsafe {
            core::slice::from_raw_parts(
                core::ptr::from_ref(self) as *const u8,
                core::mem::size_of::<Self>(),
            )
        }
    }
}
