use alloc::vec::Vec;
use core::marker::PhantomData;

use super::gpu::WebGL;
use crate::descriptor::{
    BindGroupEntry, BindGroupLayoutEntry, BindingResource, BindingType, BlendState,
    ColorAttachment, ColorTargetState, SamplerDescriptor, StencilFaceState, VertexAttribute,
};
use crate::primitive::{
    AddressMode, BufferSize, Color, ColorWrite, CompareFunction, CullMode, FilterMode, FrontFace,
    PrimitiveTopology, ShaderStage, TextureFormat, VertexStepMode,
};

/// The context ID.
static mut CONTEXT_ID: ContextId = ContextId::new(0.);

/// A resource ID type for interop in WASM environment.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Id<const T: usize>(pub f64);

impl<const T: usize> Id<T> {
    /// Creates a null Id.
    #[inline]
    pub fn null() -> Self {
        Self(0.)
    }

    /// Returns if Id is null
    #[inline]
    pub fn is_null(&self) -> bool {
        self.0 == 0.
    }
}

/// A raw slice of data for interop in WASM environment.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Slice<T = u8> {
    pub ptr: BufferSize,
    pub len: BufferSize,
    phantom: PhantomData<*const T>,
}

impl<T> Slice<T> {
    /// Creates a slice from raw parts.
    pub fn from_raw_parts(ptr: *const T, len: usize) -> Self {
        Slice {
            ptr: ptr as BufferSize,
            len: len as BufferSize,
            phantom: PhantomData,
        }
    }

    /// Creates an empty slice.
    #[inline]
    pub fn empty() -> Self {
        Self {
            ptr: 0,
            len: 0,
            phantom: PhantomData,
        }
    }
}

impl<'a> From<&'a str> for Slice {
    #[inline]
    fn from(data: &'a str) -> Self {
        Self::from_raw_parts(data.as_ptr(), data.len())
    }
}

impl<'a, T> From<&'a [T]> for Slice<T> {
    #[inline]
    fn from(data: &'a [T]) -> Self {
        Self::from_raw_parts(data.as_ptr(), data.len())
    }
}

impl<'a, T> From<&'a Vec<T>> for Slice<T> {
    #[inline]
    fn from(data: &'a Vec<T>) -> Self {
        Self::from_raw_parts(data.as_ptr(), data.len())
    }
}

#[non_exhaustive]
pub struct ResourceType;

impl ResourceType {
    pub const CONTEXT: usize = 0;
    pub const FUTURE: usize = 1;
    pub const CANVAS: usize = 2;
    pub const IMAGE_SOURCE: usize = 3;
    pub const DEVICE: usize = 4;
    pub const BUFFER: usize = 5;
    pub const TEXTURE: usize = 6;
    pub const SAMPLER: usize = 7;
    pub const SHADER: usize = 8;
    pub const BIND_GROUP_LAYOUT: usize = 9;
    pub const BIND_GROUP: usize = 10;
    pub const RENDER_PIPELINE: usize = 11;
    pub const RENDER_PASS: usize = 12;
}

/// App context Id.
pub type ContextId = Id<{ ResourceType::CONTEXT }>;

pub type FutureId = Id<{ ResourceType::FUTURE }>;
pub type CanvasId = Id<{ ResourceType::CANVAS }>;
pub type ImageSourceId = Id<{ ResourceType::IMAGE_SOURCE }>;
pub type DeviceId = Id<{ ResourceType::DEVICE }>;
pub type BufferId = Id<{ ResourceType::BUFFER }>;
pub type TextureId = Id<{ ResourceType::TEXTURE }>;
pub type SamplerId = Id<{ ResourceType::SAMPLER }>;
pub type ShaderId = Id<{ ResourceType::SHADER }>;
pub type BindGroupLayoutId = Id<{ ResourceType::BIND_GROUP_LAYOUT }>;
pub type BindGroupId = Id<{ ResourceType::BIND_GROUP }>;
pub type RenderPipelineId = Id<{ ResourceType::RENDER_PIPELINE }>;
pub type RenderPassId = Id<{ ResourceType::RENDER_PASS }>;

impl ContextId {
    /// Creates a new context ID.
    const fn new(id: f64) -> Self {
        Self(id)
    }

    /// Gets the context ID.
    pub fn get() -> Self {
        let id = unsafe { CONTEXT_ID };
        #[cfg(feature = "wasm-bindgen")]
        {
            super::mugl::set_context_memory(id.0, wasm_bindgen::memory());
        }
        id
    }

    /// Sets the context ID.
    pub fn set(id: u32) {
        unsafe {
            CONTEXT_ID = ContextId::new(id as f64);
        }
    }
}

#[allow(dead_code)]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FutureStatus {
    Pending = 0,
    Done = 1,
    Error = 2,
}

impl Color<f32> {
    /// Creates a none Color.
    pub fn none() -> Self {
        Self(f32::NAN, f32::NAN, f32::NAN, f32::NAN)
    }
}

impl From<Option<Color>> for Color<f32> {
    #[inline]
    fn from(color: Option<Color>) -> Self {
        color.map(Into::into).unwrap_or(Self::none())
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct JsSamplerDescriptor {
    pub address_mode_u: AddressMode,
    pub address_mode_v: AddressMode,
    pub address_mode_w: AddressMode,
    pub mag_filter: FilterMode,
    pub min_filter: FilterMode,
    pub mipmap_filter: FilterMode,
    pub lod_min_clamp: f32,
    pub lod_max_clamp: f32,
    /// Compare function or 0
    pub compare: u32,
    pub max_anisotropy: u8,
}

impl From<SamplerDescriptor> for JsSamplerDescriptor {
    fn from(desc: SamplerDescriptor) -> Self {
        Self {
            address_mode_u: desc.address_mode_u,
            address_mode_v: desc.address_mode_v,
            address_mode_w: desc.address_mode_w,
            mag_filter: desc.mag_filter,
            min_filter: desc.min_filter,
            mipmap_filter: desc.mipmap_filter,
            lod_min_clamp: desc.lod_min_clamp,
            lod_max_clamp: desc.lod_max_clamp,
            compare: desc.compare.map(|c| c as u32).unwrap_or(0),
            max_anisotropy: desc.max_anisotropy,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct JsBindGroupLayoutEntry {
    pub label: Slice,
    pub binding: u32,
    pub visibility: ShaderStage,
    pub ty: BindingType,
}

impl<'a> From<&BindGroupLayoutEntry<'a>> for JsBindGroupLayoutEntry {
    fn from(entry: &BindGroupLayoutEntry<'a>) -> Self {
        Self {
            label: entry.label.into(),
            binding: entry.binding,
            visibility: entry.visibility,
            ty: entry.ty,
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum BindingTypeId {
    Buffer = 0,
    Sampler = 1,
    Texture = 2,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JsBindGroupEntry {
    pub binding: u32,
    pub ty: BindingTypeId,
    pub resource: JsBindingResource,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union JsBindingResource {
    buffer: JsBufferBinding,
    sampler: SamplerId,
    texture: TextureId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct JsBufferBinding {
    buffer: BufferId,
    offset: BufferSize,
    size: BufferSize,
}

impl<'a> From<&BindGroupEntry<'a, WebGL>> for JsBindGroupEntry {
    #[inline]
    fn from(entry: &BindGroupEntry<'a, WebGL>) -> Self {
        match entry.resource {
            BindingResource::Buffer {
                buffer,
                offset,
                size,
            } => Self {
                binding: entry.binding,
                ty: BindingTypeId::Buffer,
                resource: JsBindingResource {
                    buffer: JsBufferBinding {
                        buffer: buffer.id,
                        offset,
                        size,
                    },
                },
            },
            BindingResource::Sampler(sampler) => Self {
                binding: entry.binding,
                ty: BindingTypeId::Sampler,
                resource: JsBindingResource {
                    sampler: sampler.id,
                },
            },
            BindingResource::Texture(texture) => Self {
                binding: entry.binding,
                ty: BindingTypeId::Texture,
                resource: JsBindingResource {
                    texture: texture.id,
                },
            },
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct JsRenderPassDescriptor {
    pub clear_depth: f32,
    pub clear_stencil: f32,
    pub clear_color: Color<f32>,
    /// bool as f64 for alignment
    pub is_offscreen: f64,
    pub texture: TextureId,
    pub mip_level: u32,
    pub slice: u32,
    pub colors: Slice<JsColorAttachment>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct JsColorAttachment {
    pub texture: TextureId,
    pub mip_level: u32,
    pub slice: u32,
    pub clear_color: Color<f32>,
}

impl<'a> From<&ColorAttachment<'a, WebGL>> for JsColorAttachment {
    fn from(color: &ColorAttachment<'a, WebGL>) -> Self {
        Self {
            texture: color.view.texture.id,
            mip_level: color.view.mip_level,
            slice: color.view.slice,
            clear_color: color.clear.into(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct JsRenderPipelineDescriptor {
    pub vertex: ShaderId,
    pub fragment: ShaderId,
    pub attributes: Slice<VertexAttribute>,
    pub buffers: Slice<JsVertexBufferLayout>,
    pub bind_groups: Slice<BindGroupLayoutId>,
    pub topology: PrimitiveTopology,
    /// IndexFormat or 0
    pub index_format: u32,
    pub front_face: FrontFace,
    pub cull_mode: CullMode,
    pub sample_count: u32,
    /// bool as u32 for alignment
    pub alpha_to_coverage: u32,
    /// bool as u32 for alignment
    pub has_depth_stencil: u32,
    pub depth_stencil_format: TextureFormat,
    /// bool as u32 for alignment
    pub depth_write: u32,
    pub depth_compare: CompareFunction,
    pub stencil_front: StencilFaceState,
    pub stencil_back: StencilFaceState,
    pub stencil_read_mask: u32,
    pub stencil_write_mask: u32,
    pub depth_bias: f32,
    pub depth_bias_slope_scale: f32,
    pub depth_bias_clamp: f32,
    pub targets: Slice<JsColorTargetState>,
    pub write_mask: ColorWrite,
    pub blend: BlendState,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct JsVertexBufferLayout {
    pub attributes_offset: u32,
    pub attributes_len: u32,
    pub stride: BufferSize,
    pub step_mode: VertexStepMode,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct JsColorTargetState {
    pub format: TextureFormat,
    pub write_mask: ColorWrite,
    pub blend: BlendState,
}

impl From<&ColorTargetState> for JsColorTargetState {
    fn from(target: &ColorTargetState) -> Self {
        Self {
            format: target.format,
            write_mask: target.write_mask,
            blend: target.blend.unwrap_or(BlendState::default()),
        }
    }
}
