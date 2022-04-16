use alloc::vec::Vec;
use bitflags::bitflags;
use core::ops::Deref;

use crate::primitive::{Extent2D, PowerPreference, TextureDimension, TextureFormat};

bitflags! {
    /// WebGPU features.
    #[repr(transparent)]
    pub struct WGPUFeatures: u32 {
    }
}

/// WebGPU device descriptor.
#[derive(Clone, Copy, Debug, Default)]
pub struct WGPUDeviceDescriptor {
    pub power_preference: PowerPreference,
    pub force_fallback_adapter: bool,
}

/// WebGPU surface descriptor.
#[derive(Clone, Copy, Debug)]
pub struct WGPUSurfaceDescriptor {
    pub depth_stencil_format: Option<TextureFormat>,
    pub sample_count: u32,
    pub size: Extent2D,
}

impl Default for WGPUSurfaceDescriptor {
    fn default() -> Self {
        Self {
            depth_stencil_format: Some(TextureFormat::DEPTH24STENCIL8),
            sample_count: 1,
            size: Extent2D::default(),
        }
    }
}

/// WebGPU buffer.
#[derive(Debug)]
pub struct WGPUBuffer {
    pub(super) buffer: wgpu::Buffer,
}

/// Readonly WebGPU buffer view.
#[derive(Debug)]
pub struct WGPUBufferView<'a> {
    pub(super) buffer: &'a wgpu::Buffer,
    pub(super) view: Option<wgpu::BufferView<'a>>,
}

impl<'a> Deref for WGPUBufferView<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        if let Some(ref view) = self.view {
            view
        } else {
            &[]
        }
    }
}

impl<'a> Drop for WGPUBufferView<'a> {
    fn drop(&mut self) {
        self.buffer.unmap();
    }
}

/// WebGPU texture.
#[derive(Debug)]
pub struct WGPUTexture {
    pub(super) texture: wgpu::Texture,
    pub(super) view: wgpu::TextureView,
    pub(super) msaa_texture: Option<wgpu::Texture>,
    pub(super) format: TextureFormat,
    pub(super) dimension: TextureDimension,
}

/// WebGPU sampler.
#[derive(Debug)]
pub struct WGPUSampler {
    pub(super) sampler: wgpu::Sampler,
}

/// WebGPU shader.
#[derive(Debug)]
pub struct WGPUShader {
    pub(super) shader: wgpu::ShaderModule,
}

/// WebGPU render pipeline.
#[derive(Debug)]
pub struct WGPURenderPipeline {
    pub(super) pipeline: wgpu::RenderPipeline,
    pub(super) index_format: wgpu::IndexFormat,
}

/// WebGPU render pass.
#[derive(Debug)]
pub struct WGPURenderPass {
    pub(super) color_views: Vec<wgpu::TextureView>,
    pub(super) resolve_targets: Vec<Option<wgpu::TextureView>>,
    pub(super) color_ops: Vec<wgpu::Operations<wgpu::Color>>,
    pub(super) depth_view: Option<wgpu::TextureView>,
    pub(super) depth_ops: Option<wgpu::Operations<f32>>,
    pub(super) stencil_ops: Option<wgpu::Operations<u32>>,
}

/// WebGPU bind group.
#[derive(Debug)]
pub struct WGPUBindGroup {
    pub(super) bind_group: wgpu::BindGroup,
}

/// WebGPU bind group layout.
#[derive(Debug)]
pub struct WGPUBindGroupLayout {
    pub(super) layout: wgpu::BindGroupLayout,
}
