//! GPU object descriptors.

use crate::gpu::{GPUWebExt, GPU};
use crate::primitive::{
    AddressMode, BlendFactor, BlendOperation, BufferSize, BufferUsage, Color, ColorWrite,
    CompareFunction, CullMode, Extent3D, FilterMode, FrontFace, IndexFormat, Origin2D, Origin3D,
    PrimitiveTopology, SamplerBindingType, ShaderStage, StencilOperation, TextureDimension,
    TextureFormat, TextureSampleType, TextureUsage, VertexFormat, VertexStepMode,
};

/// This specifies the options to use in creating a Buffer.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct BufferDescriptor {
    pub size: BufferSize,
    pub usage: BufferUsage,
}

/// This specifies the options to use in creating a Texture.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct TextureDescriptor {
    pub size: Extent3D,
    pub mip_level_count: u32,
    pub sample_count: u32,
    pub dimension: TextureDimension,
    pub format: TextureFormat,
    pub usage: TextureUsage,
}

impl Default for TextureDescriptor {
    fn default() -> Self {
        Self {
            size: Extent3D(0, 0, 0),
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::default(),
            format: TextureFormat::default(),
            usage: TextureUsage::default(),
        }
    }
}

/// This specifies a texture view.
#[derive(Debug)]
pub struct TextureView<'a, G: GPU> {
    pub texture: &'a G::Texture,
    pub mip_level: u32,
    pub slice: u32,
}

impl<'a, G: GPU> From<&'a G::Texture> for TextureView<'a, G> {
    fn from(texture: &'a G::Texture) -> Self {
        Self {
            texture,
            mip_level: 0,
            slice: 0,
        }
    }
}

impl<'a, G: GPU> Clone for TextureView<'a, G> {
    fn clone(&self) -> Self {
        Self {
            texture: self.texture,
            mip_level: self.mip_level,
            slice: self.slice,
        }
    }
}

impl<'a, G: GPU> Copy for TextureView<'a, G> {}

/// This specifies the texture with origin offset for a texture write operation.
/// See: https://www.w3.org/TR/webgpu/#dictdef-gpuimagecopytexture
#[derive(Debug)]
pub struct ImageCopyTexture<'a, G: GPU> {
    pub texture: &'a G::Texture,
    pub mip_level: u32,
    pub origin: Origin3D,
}

impl<'a, G: GPU> From<&'a G::Texture> for ImageCopyTexture<'a, G> {
    fn from(texture: &'a G::Texture) -> Self {
        Self {
            texture,
            mip_level: 0,
            origin: Origin3D(0, 0, 0),
        }
    }
}

impl<'a, G: GPU> Clone for ImageCopyTexture<'a, G> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            texture: self.texture,
            mip_level: self.mip_level,
            origin: self.origin,
        }
    }
}

impl<'a, G: GPU> Copy for ImageCopyTexture<'a, G> {}

/// This specifies the external image source with origin offset for a texture write operation.
#[derive(Debug)]
pub struct ImageCopyExternalImage<'a, G: GPUWebExt> {
    pub src: &'a G::ImageSource,
    pub origin: Origin2D,
}

impl<'a, G: GPUWebExt> From<&'a G::ImageSource> for ImageCopyExternalImage<'a, G> {
    fn from(src: &'a G::ImageSource) -> Self {
        Self {
            src,
            origin: Origin2D(0, 0),
        }
    }
}

impl<'a, G: GPUWebExt> Clone for ImageCopyExternalImage<'a, G> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            src: self.src,
            origin: self.origin,
        }
    }
}

impl<'a, G: GPUWebExt> Copy for ImageCopyExternalImage<'a, G> {}

/// This specifies the layout of a texture image buffer data for a texture write.
/// See: https://www.w3.org/TR/webgpu/#dictdef-gpuimagedatalayout
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct ImageDataLayout {
    pub offset: BufferSize,
    pub bytes_per_row: u32,
    pub rows_per_image: u32,
}

/// This specifies the options to use in creating a Sampler.
#[derive(Clone, Copy, Debug)]
pub struct SamplerDescriptor {
    pub address_mode_u: AddressMode,
    pub address_mode_v: AddressMode,
    pub address_mode_w: AddressMode,
    pub mag_filter: FilterMode,
    pub min_filter: FilterMode,
    pub mipmap_filter: FilterMode,
    pub lod_min_clamp: f32,
    pub lod_max_clamp: f32,
    pub compare: Option<CompareFunction>,
    /// Max anisotropy level. Requires EXT_texture_filter_anisotropic extension.
    pub max_anisotropy: u8,
}

impl Default for SamplerDescriptor {
    fn default() -> Self {
        Self {
            address_mode_u: AddressMode::default(),
            address_mode_v: AddressMode::default(),
            address_mode_w: AddressMode::default(),
            mag_filter: FilterMode::default(),
            min_filter: FilterMode::default(),
            mipmap_filter: FilterMode::default(),
            lod_min_clamp: 0.,
            lod_max_clamp: 32.,
            compare: None,
            max_anisotropy: 1,
        }
    }
}

/// This specifies the options to use in creating a Shader.
#[derive(Clone, Copy, Debug)]
pub struct ShaderDescriptor<'a> {
    pub code: &'a str,
    pub usage: ShaderStage,
}

/// This describes the state of a render pipeline.
#[derive(Clone, Copy, Debug)]
pub struct RenderPipelineDescriptor<'a, G: GPU> {
    pub vertex: &'a G::Shader,
    pub fragment: &'a G::Shader,
    pub buffers: &'a [VertexBufferLayout<'a>],
    pub bind_groups: &'a [&'a G::BindGroupLayout],
    pub primitive: PrimitiveState,
    pub multisample: MultisampleState,
    pub depth_stencil: Option<DepthStencilState>,
    pub targets: ColorTargetStates<'a>,
}

/// This describes the primitive state of a render pipeline.
/// See: https://www.w3.org/TR/webgpu/#dictdef-gpuprimitivestate
#[derive(Clone, Copy, Debug, Default)]
pub struct PrimitiveState {
    pub topology: PrimitiveTopology,
    pub index_format: Option<IndexFormat>,
    pub front_face: FrontFace,
    pub cull_mode: CullMode,
}

/// This describes the multisample state of a render pipeline.
/// See: https://www.w3.org/TR/webgpu/#dictdef-gpumultisamplestate
#[derive(Clone, Copy, Debug)]
pub struct MultisampleState {
    pub count: u32,
    pub alpha_to_coverage: bool,
}

impl Default for MultisampleState {
    fn default() -> Self {
        Self {
            count: 1,
            alpha_to_coverage: false,
        }
    }
}

/// This describes the depth-stencil state of a render pipeline.
/// See: https://www.w3.org/TR/webgpu/#dictdef-gpudepthstencilstate
#[derive(Clone, Copy, Debug)]
pub struct DepthStencilState {
    pub format: TextureFormat,
    pub depth_write: bool,
    pub depth_compare: CompareFunction,
    pub stencil_front: StencilFaceState,
    pub stencil_back: StencilFaceState,
    pub stencil_read_mask: u32,
    pub stencil_write_mask: u32,
    pub depth_bias: f32,
    pub depth_bias_slope_scale: f32,
    pub depth_bias_clamp: f32,
}

impl Default for DepthStencilState {
    fn default() -> Self {
        Self {
            format: TextureFormat::DEPTH16,
            depth_write: false,
            depth_compare: CompareFunction::default(),
            stencil_front: StencilFaceState::default(),
            stencil_back: StencilFaceState::default(),
            stencil_read_mask: 0xFFFFFFFF,
            stencil_write_mask: 0xFFFFFFFF,
            depth_bias: 0.,
            depth_bias_slope_scale: 0.,
            depth_bias_clamp: 0.,
        }
    }
}

/// This describes a stencil face state of a DepthStencilState.
/// See: https://www.w3.org/TR/webgpu/#dictdef-gpudepthstencilstate
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct StencilFaceState {
    pub compare: CompareFunction,
    pub fail_op: StencilOperation,
    pub depth_fail_op: StencilOperation,
    pub pass_op: StencilOperation,
}

/// This describes the color target states of a render pipeline.
/// See: https://www.w3.org/TR/webgpu/#dictdef-gpucolortargetstate
#[derive(Clone, Copy, Debug)]
pub enum ColorTargetStates<'a> {
    /// Default render pass color target states.
    Default {
        write_mask: ColorWrite,
        blend: Option<BlendState>,
    },
    /// Offscreen render pass color target states.
    Offscreen { targets: &'a [ColorTargetState] },
}

impl<'a> Default for ColorTargetStates<'a> {
    #[inline]
    fn default() -> Self {
        Self::Default {
            write_mask: Default::default(),
            blend: Default::default(),
        }
    }
}

/// This describes a color target state of a render pipeline.
/// See: https://www.w3.org/TR/webgpu/#dictdef-gpucolortargetstate
#[derive(Clone, Copy, Debug, Default)]
pub struct ColorTargetState {
    pub format: TextureFormat,
    pub write_mask: ColorWrite,
    pub blend: Option<BlendState>,
}

/// This describes the blend state of a color target.
/// See: https://www.w3.org/TR/webgpu/#dictdef-gpublendstate
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct BlendState {
    pub color: BlendComponent,
    pub alpha: BlendComponent,
}

/// This describes the blend component state.
/// See: https://www.w3.org/TR/webgpu/#dictdef-gpublendcomponent
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct BlendComponent {
    pub operation: BlendOperation,
    pub src_factor: BlendFactor,
    pub dst_factor: BlendFactor,
}

impl Default for BlendComponent {
    #[inline]
    fn default() -> Self {
        Self {
            operation: BlendOperation::default(),
            src_factor: BlendFactor::One,
            dst_factor: BlendFactor::Zero,
        }
    }
}

/// This describes the layout of a vertex buffer.
/// See: https://www.w3.org/TR/webgpu/#dictdef-gpuvertexbufferlayout
#[derive(Clone, Copy, Debug, Default)]
pub struct VertexBufferLayout<'a> {
    pub stride: BufferSize,
    pub step_mode: VertexStepMode,
    pub attributes: &'a [VertexAttribute],
}

/// This describes the layout of a vertex buffer.
/// See: https://www.w3.org/TR/webgpu/#dictdef-gpuvertexbufferlayout
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct VertexAttribute {
    pub format: VertexFormat,
    pub offset: BufferSize,
    pub shader_location: u32,
}

/// This describes a render pass.
/// See: https://www.w3.org/TR/webgpu/#render-pass-encoder-creation
#[derive(Clone, Copy, Debug)]
pub enum RenderPassDescriptor<'a, 'b, G: GPU> {
    /// Default render pass
    Default {
        clear_color: Option<Color>,
        clear_depth: Option<f32>,
        clear_stencil: Option<u32>,
    },

    /// Offscreen render pass
    Offscreen {
        colors: &'b [ColorAttachment<'a, G>],
        depth_stencil: Option<TextureView<'a, G>>,
        clear_depth: Option<f32>,
        clear_stencil: Option<u32>,
    },
}

impl<'a, 'b, G: GPU> Default for RenderPassDescriptor<'a, 'b, G> {
    #[inline]
    fn default() -> Self {
        Self::Default {
            clear_color: None,
            clear_depth: None,
            clear_stencil: None,
        }
    }
}

/// This describes a color attachment for a render pass.
/// See: https://www.w3.org/TR/webgpu/#dictdef-gpurenderpasscolorattachment
#[derive(Clone, Copy, Debug)]
pub struct ColorAttachment<'a, G: GPU> {
    pub view: TextureView<'a, G>,
    pub clear: Option<Color>,
}

/// This describes the layout of a uniform resource binding group.
#[derive(Clone, Copy, Debug)]
pub struct BindGroupLayoutDescriptor<'a> {
    pub entries: &'a [BindGroupLayoutEntry<'a>],
}

/// This describes the layout of a single shader uniform resource binding.
#[derive(Clone, Copy, Debug)]
pub struct BindGroupLayoutEntry<'a> {
    pub label: &'a str,
    pub binding: u32,
    pub visibility: ShaderStage,
    pub ty: BindingType,
}

/// This describes the type of a uniform resource binding.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum BindingType {
    Buffer {
        dynamic_offset: bool,
    },
    Sampler {
        ty: SamplerBindingType,
    },
    Texture {
        sample_type: TextureSampleType,
        dimension: TextureDimension,
        multisampled: bool,
    },
}

/// This describes a uniform resource binding group.
#[derive(Clone, Copy, Debug)]
pub struct BindGroupDescriptor<'a, G: GPU> {
    pub layout: &'a G::BindGroupLayout,
    pub entries: &'a [BindGroupEntry<'a, G>],
}

/// This describes a single shader uniform resource binding.
#[derive(Clone, Copy, Debug)]
pub struct BindGroupEntry<'a, G: GPU> {
    pub binding: u32,
    pub resource: BindingResource<'a, G>,
}

/// This describes a single shader uniform resource to be bound.
#[derive(Debug)]
pub enum BindingResource<'a, G: GPU> {
    Buffer {
        buffer: &'a G::Buffer,
        offset: BufferSize,
        size: BufferSize,
    },
    Sampler(&'a G::Sampler),
    Texture(&'a G::Texture),
}

impl<'a, G: GPU> Clone for BindingResource<'a, G> {
    fn clone(&self) -> Self {
        match self {
            BindingResource::Buffer {
                buffer,
                offset,
                size,
            } => BindingResource::Buffer {
                buffer,
                offset: *offset,
                size: *size,
            },
            BindingResource::Sampler(ref sampler) => BindingResource::<'a, G>::Sampler(sampler),
            BindingResource::Texture(ref texture) => BindingResource::<'a, G>::Texture(texture),
        }
    }
}

impl<'a, G: GPU> Copy for BindingResource<'a, G> {}
