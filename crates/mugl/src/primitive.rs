//! Primitive types.

use crate::gl_const;
use bitflags::bitflags;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm64")] {
        /// Unsigned size type for buffer offset/length.
        /// TODO: Support 64-bit (or 53 using f64) buffer size. Using u32 for WASM64 target for now.
        pub type BufferSize = u32;
    } else {
        /// Unsigned size type for buffer offset/length.
        pub type BufferSize = usize;
    }
}

/// A 2D origin.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Origin2D(pub u32, pub u32);

/// A 3D origin.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Origin3D(pub u32, pub u32, pub u32);

impl From<Origin2D> for Origin3D {
    fn from(origin: Origin2D) -> Self {
        Self(origin.0, origin.1, 1)
    }
}

/// A 2D extent.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Extent2D(pub u32, pub u32);

impl Default for Extent2D {
    #[inline]
    fn default() -> Self {
        Self(1, 1)
    }
}

/// A 3D extent.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Extent3D(pub u32, pub u32, pub u32);

impl Default for Extent3D {
    #[inline]
    fn default() -> Self {
        Self(1, 1, 1)
    }
}

impl From<Extent2D> for Extent3D {
    fn from(extent: Extent2D) -> Self {
        Self(extent.0, extent.1, 1)
    }
}

/// A RGBA color.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Color<F = f64>(pub F, pub F, pub F, pub F);

impl From<Color> for Color<f32> {
    #[inline]
    fn from(color: Color) -> Self {
        Self(
            color.0 as f32,
            color.1 as f32,
            color.2 as f32,
            color.3 as f32,
        )
    }
}

impl From<Color<f32>> for Color {
    #[inline]
    fn from(color: Color<f32>) -> Self {
        Self(
            color.0 as f64,
            color.1 as f64,
            color.2 as f64,
            color.3 as f64,
        )
    }
}

bitflags! {
    /// Buffer usage.
    /// See: <https://www.w3.org/TR/webgpu/#buffer-usage>
    #[cfg_attr(
        feature = "serde",
        derive(serde::Serialize, serde::Deserialize),
        serde(transparent)
    )]
    #[repr(transparent)]
    pub struct BufferUsage: u32 {
        // Buffer types
        const INDEX = 0x0010;
        const VERTEX = 0x0020;
        const UNIFORM = 0x0040;

        // OpenGL buffer usage hints
        const DYNAMIC = 0x1000;
        const STREAM = 0x2000;
    }

    /// Texture usage.
    /// See: <https://www.w3.org/TR/webgpu/#typedefdef-gputextureusageflags>
    #[cfg_attr(
        feature = "serde",
        derive(serde::Serialize, serde::Deserialize),
        serde(transparent)
    )]
    #[repr(transparent)]
    pub struct TextureUsage: u32 {
        const TEXTURE_BINDING = 0x04;
        const RENDER_ATTACHMENT = 0x10;
    }

    /// Color write flags.
    /// See: <https://www.w3.org/TR/webgpu/#typedefdef-gpucolorwriteflags>
    #[cfg_attr(
        feature = "serde",
        derive(serde::Serialize, serde::Deserialize),
        serde(transparent)
    )]
    #[repr(transparent)]
    pub struct ColorWrite: u32 {
        const RED = 0x1;
        const GREEN = 0x2;
        const BLUE = 0x4;
        const ALPHA = 0x8;
        const ALL = 0xF;
    }

    /// Shader stage.
    /// See: <https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createShader>
    #[cfg_attr(
        feature = "serde",
        derive(serde::Serialize, serde::Deserialize),
        serde(transparent)
    )]
    #[repr(transparent)]
    pub struct ShaderStage: u32 {
        const VERTEX = 0x01;
        const FRAGMENT = 0x02;
    }
}

impl Default for BufferUsage {
    #[inline]
    fn default() -> Self {
        Self::VERTEX
    }
}

impl Default for TextureUsage {
    #[inline]
    fn default() -> Self {
        Self::TEXTURE_BINDING
    }
}

impl Default for ColorWrite {
    #[inline]
    fn default() -> Self {
        Self::ALL
    }
}

impl Default for ShaderStage {
    #[inline]
    fn default() -> Self {
        Self::VERTEX | Self::FRAGMENT
    }
}

/// Texture dimension.
/// See: <https://www.w3.org/TR/webgpu/#enumdef-gputextureviewdimension>
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum TextureDimension {
    D2 = gl_const::TEXTURE_2D,
    D2Array = gl_const::TEXTURE_2D_ARRAY,
    CubeMap = gl_const::TEXTURE_CUBE_MAP,
    D3 = gl_const::TEXTURE_3D,
}

impl Default for TextureDimension {
    #[inline]
    fn default() -> Self {
        Self::D2
    }
}

/// Texture format.
/// See: <https://www.w3.org/TR/webgpu/#texture-formats>
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum TextureFormat {
    // 8-bit formats
    R8 = gl_const::R8,
    R8SNORM = gl_const::R8_SNORM,
    R8UI = gl_const::R8UI,
    R8I = gl_const::R8I,

    // 16-bit formats
    R16UI = gl_const::R16UI,
    R16I = gl_const::R16I,
    RG8 = gl_const::RG8,
    RG8SNORM = gl_const::RG8_SNORM,
    RG8UI = gl_const::RG8UI,
    RG8I = gl_const::RG8I,

    // 32-bit formats
    R32UI = gl_const::R32UI,
    R32I = gl_const::R32I,
    RG16UI = gl_const::RG16UI,
    RG16I = gl_const::RG16I,
    RGBA8 = gl_const::RGBA8,
    SRGBA8 = gl_const::SRGB8_ALPHA8,
    RGBA8SNORM = gl_const::RGBA8_SNORM,
    RGBA8UI = gl_const::RGBA8UI,
    RGBA8I = gl_const::RGBA8I,
    // Packed 32-bit formats
    RGB10A2 = gl_const::RGB10_A2,

    // 64-bit formats
    RG32UI = gl_const::RG32UI,
    RG32I = gl_const::RG32I,
    RGBA16UI = gl_const::RGBA16UI,
    RGBA16I = gl_const::RGBA16I,

    // 128-bit formats
    RGBA32UI = gl_const::RGBA32UI,
    RGBA32I = gl_const::RGBA32I,

    // Float formats.
    R16F = gl_const::R16F,
    RG16F = gl_const::RG16F,
    RG11B10F = gl_const::R11F_G11F_B10F,
    RGBA16F = gl_const::RGBA16F,
    R32F = gl_const::R32F,
    RG32F = gl_const::RG32F,
    RGBA32F = gl_const::RGBA32F,

    // TODO: support BC / ETC2 / ASTC compressed formats

    // Depth/stencil formats
    DEPTH16 = gl_const::DEPTH_COMPONENT16,
    DEPTH24 = gl_const::DEPTH_COMPONENT24,
    DEPTH24STENCIL8 = gl_const::DEPTH24_STENCIL8,
    DEPTH32F = gl_const::DEPTH_COMPONENT32F,
    DEPTH32FSTENCIL8 = gl_const::DEPTH32F_STENCIL8,
}

impl Default for TextureFormat {
    #[inline]
    fn default() -> Self {
        Self::RGBA8
    }
}

impl TextureFormat {
    /// Returns if the texture format is a depth/stencil format
    pub const fn is_depth_stencil(&self) -> bool {
        match self {
            TextureFormat::DEPTH16
            | TextureFormat::DEPTH24
            | TextureFormat::DEPTH24STENCIL8
            | TextureFormat::DEPTH32F
            | TextureFormat::DEPTH32FSTENCIL8 => true,
            _ => false,
        }
    }

    /// Returns the byte size of the texture format
    pub const fn size(&self) -> u32 {
        match self {
            TextureFormat::R8
            | TextureFormat::R8SNORM
            | TextureFormat::R8UI
            | TextureFormat::R8I => 1,
            TextureFormat::R16UI
            | TextureFormat::R16I
            | TextureFormat::RG8
            | TextureFormat::RG8SNORM
            | TextureFormat::RG8UI
            | TextureFormat::RG8I
            | TextureFormat::R16F => 2,
            TextureFormat::R32UI
            | TextureFormat::R32I
            | TextureFormat::RG16UI
            | TextureFormat::RG16I
            | TextureFormat::RGBA8
            | TextureFormat::SRGBA8
            | TextureFormat::RGBA8SNORM
            | TextureFormat::RGBA8UI
            | TextureFormat::RGBA8I
            | TextureFormat::RGB10A2
            | TextureFormat::R32F
            | TextureFormat::RG16F
            | TextureFormat::RG11B10F => 4,
            TextureFormat::RG32UI
            | TextureFormat::RG32I
            | TextureFormat::RGBA16UI
            | TextureFormat::RGBA16I
            | TextureFormat::RGBA16F
            | TextureFormat::RG32F => 8,
            TextureFormat::RGBA32UI | TextureFormat::RGBA32I | TextureFormat::RGBA32F => 16,
            // TODO: are these sizes correct?
            TextureFormat::DEPTH16
            | TextureFormat::DEPTH24
            | TextureFormat::DEPTH32F
            | TextureFormat::DEPTH24STENCIL8
            | TextureFormat::DEPTH32FSTENCIL8 => 4,
        }
    }
}

/// Texture address mode.
/// See: <https://www.w3.org/TR/webgpu/#enumdef-gpuaddressmode>
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum AddressMode {
    ClampToEdge = gl_const::CLAMP_TO_EDGE,
    Repeat = gl_const::REPEAT,
    MirrorRepeat = gl_const::MIRRORED_REPEAT,
}

impl Default for AddressMode {
    #[inline]
    fn default() -> Self {
        Self::ClampToEdge
    }
}

/// Texture filter mode.
/// See: <https://www.w3.org/TR/webgpu/#enumdef-gpufiltermode>
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum FilterMode {
    Nearest = gl_const::NEAREST,
    Linear = gl_const::LINEAR,
}

impl Default for FilterMode {
    #[inline]
    fn default() -> Self {
        Self::Nearest
    }
}

/// Compare function.
/// See: <https://www.w3.org/TR/webgpu/#enumdef-gpucomparefunction>
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum CompareFunction {
    Never = gl_const::NEVER,
    Less = gl_const::LESS,
    Equal = gl_const::EQUAL,
    LessEqual = gl_const::LEQUAL,
    Greater = gl_const::GREATER,
    NotEqual = gl_const::NOTEQUAL,
    GreaterEqual = gl_const::GEQUAL,
    Always = gl_const::ALWAYS,
}

impl Default for CompareFunction {
    #[inline]
    fn default() -> Self {
        Self::Always
    }
}

/// Primitive topology.
/// See: <https://www.w3.org/TR/webgpu/#enumdef-gpuprimitivetopology>
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum PrimitiveTopology {
    Points = gl_const::POINTS,
    Lines = gl_const::LINES,
    LineStrip = gl_const::LINE_STRIP,
    Triangles = gl_const::TRIANGLES,
    TriangleStrip = gl_const::TRIANGLE_STRIP,
}

impl Default for PrimitiveTopology {
    #[inline]
    fn default() -> Self {
        Self::Triangles
    }
}

/// Vertex index format.
/// See: <https://www.w3.org/TR/webgpu/#enumdef-gpuindexformat>
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum IndexFormat {
    UI16 = gl_const::UNSIGNED_SHORT,
    UI32 = gl_const::UNSIGNED_INT,
}

impl Default for IndexFormat {
    #[inline]
    fn default() -> Self {
        Self::UI16
    }
}

/// Identify which side is the front face by setting a winding orientation.
/// See: <https://www.w3.org/TR/webgpu/#enumdef-gpufrontface>
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum FrontFace {
    CCW = gl_const::CCW,
    CW = gl_const::CW,
}

impl Default for FrontFace {
    #[inline]
    fn default() -> Self {
        Self::CCW
    }
}

/// Specify the face to cull.
/// See: <https://www.w3.org/TR/webgpu/#enumdef-gpucullmode>
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum CullMode {
    None = gl_const::NONE,
    Front = gl_const::FRONT,
    Back = gl_const::BACK,
}

impl Default for CullMode {
    #[inline]
    fn default() -> Self {
        Self::None
    }
}

/// Stencil-buffer operation.
/// See: <https://www.w3.org/TR/webgpu/#enumdef-gpustenciloperation>
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum StencilOperation {
    Keep = gl_const::KEEP,
    Zero = gl_const::ZERO,
    Replace = gl_const::REPLACE,
    Invert = gl_const::INVERT,
    IncrementClamp = gl_const::INCR,
    DecrementClamp = gl_const::DECR,
    IncrementWrap = gl_const::INCR_WRAP,
    DecrementWrap = gl_const::DECR_WRAP,
}

impl Default for StencilOperation {
    #[inline]
    fn default() -> Self {
        Self::Keep
    }
}

/// Specify the alpha-blending factor.
/// See: <https://www.w3.org/TR/webgpu/#enumdef-gpublendfactor>
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum BlendFactor {
    Zero = gl_const::ZERO,
    One = gl_const::ONE,
    Src = gl_const::SRC_COLOR,
    OneMinusSrc = gl_const::ONE_MINUS_SRC_COLOR,
    SrcAlpha = gl_const::SRC_ALPHA,
    OneMinusSrcAlpha = gl_const::ONE_MINUS_SRC_ALPHA,
    Dst = gl_const::DST_COLOR,
    OneMinusDst = gl_const::ONE_MINUS_DST_COLOR,
    DstAlpha = gl_const::DST_ALPHA,
    OneMinusDstAlpha = gl_const::ONE_MINUS_DST_ALPHA,
    SrcAlphaSaturated = gl_const::SRC_ALPHA_SATURATE,
    Constant = gl_const::CONSTANT_COLOR,
    OneMinusConstant = gl_const::ONE_MINUS_CONSTANT_COLOR,
}

/// Specify the blend operation.
/// See: <https://www.w3.org/TR/webgpu/#enumdef-gpublendoperation>
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum BlendOperation {
    Add = gl_const::FUNC_ADD,
    Subtract = gl_const::FUNC_SUBTRACT,
    ReverseSubtract = gl_const::FUNC_REVERSE_SUBTRACT,
    Min = gl_const::MIN,
    Max = gl_const::MAX,
}

impl Default for BlendOperation {
    #[inline]
    fn default() -> Self {
        Self::Add
    }
}

/// Vertex step mode.
/// See: <https://www.w3.org/TR/webgpu/#enumdef-gpuvertexstepmode>
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum VertexStepMode {
    Vertex = 0,
    Instance = 1,
}

impl Default for VertexStepMode {
    #[inline]
    fn default() -> Self {
        Self::Vertex
    }
}

/// Vertex format.
/// See: <https://www.w3.org/TR/webgpu/#enumdef-gpuvertexformat>
/// Enum values encode the properties of the formats:
/// - bits 0-3 encodes the number of components (1, 2, 3 or 4)
/// - bits 4-7 encodes the number of bytes per component (1, 2 or 4)
/// - bits 8-11 encodes the data type (1 = int, 2 = float)
/// - bits 12-15 encodes the signedness and normalization for int (0 = unsigned, 1 = signed, 2 = unsigned normalized, 3 = signed normalized)
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum VertexFormat {
    UI8x2 = 0x01_1_2,
    UI8x4 = 0x01_1_4,
    I8x2 = 0x11_1_2,
    I8x4 = 0x11_1_4,
    UNORM8x2 = 0x21_1_2,
    UNORM8x4 = 0x21_1_4,
    SNORM8x2 = 0x31_1_2,
    SNORM8x4 = 0x31_1_4,

    UI16x2 = 0x01_2_2,
    UI16x4 = 0x01_2_4,
    I16x2 = 0x11_2_2,
    I16x4 = 0x11_2_4,
    UNORM16x2 = 0x21_2_2,
    UNORM16x4 = 0x21_2_4,
    SNORM16x2 = 0x31_2_2,
    SNORM16x4 = 0x31_2_4,

    F16x2 = 0x02_2_2,
    F16x4 = 0x02_2_4,
    F32 = 0x02_4_1,
    F32x2 = 0x02_4_2,
    F32x3 = 0x02_4_3,
    F32x4 = 0x02_4_4,
}

impl Default for VertexFormat {
    #[inline]
    fn default() -> Self {
        Self::F32x3
    }
}

/// Hint indicating what class of device should be selected.
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum PowerPreference {
    LowPower = 0,
    HighPerformance = 1,
}

impl Default for PowerPreference {
    #[inline]
    fn default() -> Self {
        Self::HighPerformance
    }
}

/// The required type for a sampler binding
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum SamplerBindingType {
    Filtering = 0,
    NonFiltering = 1,
    Comparison = 2,
}

impl Default for SamplerBindingType {
    #[inline]
    fn default() -> Self {
        Self::Filtering
    }
}

/// The type of a sample for a texture binding
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum TextureSampleType {
    Float = 0,
    Depth = 1,
    Int = 2,
    Uint = 3,
}

impl Default for TextureSampleType {
    #[inline]
    fn default() -> Self {
        Self::Float
    }
}

/// Hint for mipmap generation.
/// See: <https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/hint>
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum MipmapHint {
    Fast = gl_const::FASTEST,
    Nice = gl_const::NICEST,
}
