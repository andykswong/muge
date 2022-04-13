use super::WGPU;
use crate::descriptor::{
    BindingResource, BindingType, BlendComponent, BlendState, DepthStencilState, ImageCopyTexture,
    ImageDataLayout, MultisampleState, PrimitiveState, StencilFaceState, TextureView,
};
use crate::primitive::{
    AddressMode, BlendFactor, BlendOperation, BufferUsage, Color, ColorWrite, CompareFunction,
    CullMode, Extent2D, Extent3D, FilterMode, FrontFace, IndexFormat, Origin3D, PowerPreference,
    PrimitiveTopology, SamplerBindingType, ShaderStage, StencilOperation, TextureDimension,
    TextureFormat, TextureSampleType, TextureUsage, VertexFormat, VertexStepMode,
};

pub fn wgpu_operations<T>(ops: Option<T>) -> wgpu::Operations<T> {
    wgpu::Operations {
        load: match ops {
            None => wgpu::LoadOp::Load,
            Some(t) => wgpu::LoadOp::Clear(t),
        },
        store: true,
    }
}

impl From<PowerPreference> for wgpu::PowerPreference {
    fn from(preference: PowerPreference) -> Self {
        match preference {
            PowerPreference::LowPower => wgpu::PowerPreference::LowPower,
            PowerPreference::HighPerformance => wgpu::PowerPreference::HighPerformance,
        }
    }
}

impl From<BufferUsage> for wgpu::BufferUsages {
    fn from(usage: BufferUsage) -> Self {
        let mut result = wgpu::BufferUsages::MAP_READ
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST;
        if usage.contains(BufferUsage::VERTEX) {
            result.insert(wgpu::BufferUsages::VERTEX);
        }
        if usage.contains(BufferUsage::INDEX) {
            result.insert(wgpu::BufferUsages::INDEX);
        }
        if usage.contains(BufferUsage::UNIFORM) {
            result.insert(wgpu::BufferUsages::UNIFORM);
        }
        result
    }
}

impl From<Extent2D> for wgpu::Extent3d {
    fn from(extend: Extent2D) -> Self {
        Self {
            width: extend.0,
            height: extend.1,
            depth_or_array_layers: 1,
        }
    }
}

impl From<Extent3D> for wgpu::Extent3d {
    fn from(extend: Extent3D) -> Self {
        Self {
            width: extend.0,
            height: extend.1,
            depth_or_array_layers: extend.2,
        }
    }
}

impl From<Origin3D> for wgpu::Origin3d {
    fn from(origin: Origin3D) -> Self {
        Self {
            x: origin.0,
            y: origin.1,
            z: origin.2,
        }
    }
}

impl From<Color> for wgpu::Color {
    fn from(color: Color) -> Self {
        Self {
            r: color.0,
            g: color.1,
            b: color.2,
            a: color.3,
        }
    }
}

impl<'a> From<ImageCopyTexture<'a, WGPU>> for wgpu::ImageCopyTexture<'a> {
    fn from(texture: ImageCopyTexture<'a, WGPU>) -> Self {
        Self {
            texture: &texture.texture.texture,
            mip_level: texture.mip_level,
            origin: texture.origin.into(),
            aspect: wgpu::TextureAspect::All,
        }
    }
}

impl From<ImageDataLayout> for wgpu::ImageDataLayout {
    fn from(layout: ImageDataLayout) -> Self {
        wgpu::ImageDataLayout {
            offset: layout.offset as u64,
            bytes_per_row: core::num::NonZeroU32::new(layout.bytes_per_row),
            rows_per_image: core::num::NonZeroU32::new(layout.rows_per_image),
        }
    }
}

impl From<FilterMode> for wgpu::FilterMode {
    fn from(mode: FilterMode) -> Self {
        match mode {
            FilterMode::Nearest => wgpu::FilterMode::Nearest,
            FilterMode::Linear => wgpu::FilterMode::Linear,
        }
    }
}

impl From<AddressMode> for wgpu::AddressMode {
    fn from(mode: AddressMode) -> Self {
        match mode {
            AddressMode::ClampToEdge => wgpu::AddressMode::ClampToEdge,
            AddressMode::Repeat => wgpu::AddressMode::Repeat,
            AddressMode::MirrorRepeat => wgpu::AddressMode::MirrorRepeat,
        }
    }
}

impl From<CompareFunction> for wgpu::CompareFunction {
    fn from(compare: CompareFunction) -> Self {
        match compare {
            CompareFunction::Never => wgpu::CompareFunction::Never,
            CompareFunction::Less => wgpu::CompareFunction::Less,
            CompareFunction::Equal => wgpu::CompareFunction::Equal,
            CompareFunction::LessEqual => wgpu::CompareFunction::LessEqual,
            CompareFunction::Greater => wgpu::CompareFunction::Greater,
            CompareFunction::NotEqual => wgpu::CompareFunction::NotEqual,
            CompareFunction::GreaterEqual => wgpu::CompareFunction::GreaterEqual,
            CompareFunction::Always => wgpu::CompareFunction::Always,
        }
    }
}

impl From<TextureDimension> for wgpu::TextureDimension {
    fn from(dim: TextureDimension) -> Self {
        match dim {
            TextureDimension::D2 => wgpu::TextureDimension::D2,
            TextureDimension::CubeMap => wgpu::TextureDimension::D2,
            TextureDimension::D2Array => wgpu::TextureDimension::D2,
            TextureDimension::D3 => wgpu::TextureDimension::D3,
        }
    }
}

impl From<TextureDimension> for wgpu::TextureViewDimension {
    fn from(dim: TextureDimension) -> Self {
        match dim {
            TextureDimension::D2 => wgpu::TextureViewDimension::D2,
            TextureDimension::CubeMap => wgpu::TextureViewDimension::Cube,
            TextureDimension::D2Array => wgpu::TextureViewDimension::D2Array,
            TextureDimension::D3 => wgpu::TextureViewDimension::D3,
        }
    }
}

impl From<TextureFormat> for wgpu::TextureFormat {
    fn from(format: TextureFormat) -> Self {
        match format {
            TextureFormat::R8 => wgpu::TextureFormat::R8Unorm,
            TextureFormat::R8SNORM => wgpu::TextureFormat::R8Snorm,
            TextureFormat::R8UI => wgpu::TextureFormat::R8Uint,
            TextureFormat::R8I => wgpu::TextureFormat::R8Sint,
            TextureFormat::R16UI => wgpu::TextureFormat::R16Uint,
            TextureFormat::R16I => wgpu::TextureFormat::R16Sint,
            TextureFormat::RG8 => wgpu::TextureFormat::Rg8Unorm,
            TextureFormat::RG8SNORM => wgpu::TextureFormat::Rg8Snorm,
            TextureFormat::RG8UI => wgpu::TextureFormat::Rg8Uint,
            TextureFormat::RG8I => wgpu::TextureFormat::Rg8Sint,
            TextureFormat::R32UI => wgpu::TextureFormat::R32Uint,
            TextureFormat::R32I => wgpu::TextureFormat::R32Sint,
            TextureFormat::RG16UI => wgpu::TextureFormat::Rg16Uint,
            TextureFormat::RG16I => wgpu::TextureFormat::Rg16Sint,
            TextureFormat::RGBA8 => wgpu::TextureFormat::Rgba8Unorm,
            TextureFormat::SRGBA8 => wgpu::TextureFormat::Rgba8UnormSrgb,
            TextureFormat::RGBA8SNORM => wgpu::TextureFormat::Rgba8Snorm,
            TextureFormat::RGBA8UI => wgpu::TextureFormat::Rgba8Uint,
            TextureFormat::RGBA8I => wgpu::TextureFormat::Rgba8Sint,
            TextureFormat::RGB10A2 => wgpu::TextureFormat::Rgb10a2Unorm,
            TextureFormat::RG32UI => wgpu::TextureFormat::Rg32Uint,
            TextureFormat::RG32I => wgpu::TextureFormat::Rg32Sint,
            TextureFormat::RGBA16UI => wgpu::TextureFormat::Rgba16Uint,
            TextureFormat::RGBA16I => wgpu::TextureFormat::Rgba16Sint,
            TextureFormat::RGBA32UI => wgpu::TextureFormat::Rgba32Uint,
            TextureFormat::RGBA32I => wgpu::TextureFormat::Rgba32Sint,
            TextureFormat::R16F => wgpu::TextureFormat::R16Float,
            TextureFormat::RG16F => wgpu::TextureFormat::Rg16Float,
            TextureFormat::RG11B10F => wgpu::TextureFormat::Rg11b10Float,
            TextureFormat::RGBA16F => wgpu::TextureFormat::Rgba16Float,
            TextureFormat::R32F => wgpu::TextureFormat::R32Float,
            TextureFormat::RG32F => wgpu::TextureFormat::Rg32Float,
            TextureFormat::RGBA32F => wgpu::TextureFormat::Rgba32Float,
            TextureFormat::DEPTH24 => wgpu::TextureFormat::Depth24Plus,
            TextureFormat::DEPTH24STENCIL8 => wgpu::TextureFormat::Depth24PlusStencil8,
            TextureFormat::DEPTH32F => wgpu::TextureFormat::Depth32Float,
            TextureFormat::DEPTH16 => wgpu::TextureFormat::Depth24Plus, // TODO: not supported by wgpu yet
            TextureFormat::DEPTH32FSTENCIL8 => wgpu::TextureFormat::Depth24PlusStencil8, // TODO: not supported by wgpu yet
        }
    }
}

impl From<TextureUsage> for wgpu::TextureUsages {
    fn from(usage: TextureUsage) -> Self {
        let mut result = wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST;
        if usage.contains(TextureUsage::TEXTURE_BINDING) {
            result.insert(wgpu::TextureUsages::TEXTURE_BINDING);
        }
        if usage.contains(TextureUsage::RENDER_ATTACHMENT) {
            result.insert(wgpu::TextureUsages::RENDER_ATTACHMENT);
        }
        result
    }
}

impl<'a> From<TextureView<'a, WGPU>> for wgpu::TextureView {
    fn from(view: TextureView<'a, WGPU>) -> Self {
        view.texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                label: None,
                format: Some(view.texture.format.into()),
                dimension: Some(view.texture.dimension.into()),
                aspect: wgpu::TextureAspect::All,
                base_mip_level: view.mip_level,
                mip_level_count: core::num::NonZeroU32::new(1),
                base_array_layer: view.slice,
                array_layer_count: core::num::NonZeroU32::new(1),
            })
    }
}

impl From<IndexFormat> for wgpu::IndexFormat {
    fn from(format: IndexFormat) -> Self {
        match format {
            IndexFormat::UI16 => wgpu::IndexFormat::Uint16,
            IndexFormat::UI32 => wgpu::IndexFormat::Uint32,
        }
    }
}

impl From<PrimitiveTopology> for wgpu::PrimitiveTopology {
    fn from(topology: PrimitiveTopology) -> Self {
        match topology {
            PrimitiveTopology::Points => wgpu::PrimitiveTopology::PointList,
            PrimitiveTopology::Lines => wgpu::PrimitiveTopology::LineList,
            PrimitiveTopology::LineStrip => wgpu::PrimitiveTopology::LineStrip,
            PrimitiveTopology::Triangles => wgpu::PrimitiveTopology::TriangleList,
            PrimitiveTopology::TriangleStrip => wgpu::PrimitiveTopology::TriangleStrip,
        }
    }
}

impl From<MultisampleState> for wgpu::MultisampleState {
    fn from(state: MultisampleState) -> Self {
        wgpu::MultisampleState {
            count: state.count,
            mask: !0,
            alpha_to_coverage_enabled: state.alpha_to_coverage,
        }
    }
}

impl From<FrontFace> for wgpu::FrontFace {
    fn from(face: FrontFace) -> Self {
        match face {
            FrontFace::CCW => wgpu::FrontFace::Ccw,
            FrontFace::CW => wgpu::FrontFace::Cw,
        }
    }
}

impl From<CullMode> for Option<wgpu::Face> {
    fn from(mode: CullMode) -> Self {
        match mode {
            CullMode::None => None,
            CullMode::Front => Some(wgpu::Face::Front),
            CullMode::Back => Some(wgpu::Face::Back),
        }
    }
}

impl From<PrimitiveState> for wgpu::PrimitiveState {
    fn from(state: PrimitiveState) -> Self {
        wgpu::PrimitiveState {
            topology: state.topology.into(),
            strip_index_format: match state.topology {
                PrimitiveTopology::LineStrip | PrimitiveTopology::TriangleStrip => state.index_format.map(Into::into),
                _ => None,
            },
            front_face: state.front_face.into(),
            cull_mode: state.cull_mode.into(),
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        }
    }
}

impl From<DepthStencilState> for wgpu::DepthStencilState {
    fn from(state: DepthStencilState) -> Self {
        wgpu::DepthStencilState {
            format: state.format.into(),
            depth_write_enabled: state.depth_write,
            depth_compare: state.depth_compare.into(),
            stencil: wgpu::StencilState {
                front: state.stencil_front.into(),
                back: state.stencil_back.into(),
                read_mask: state.stencil_read_mask,
                write_mask: state.stencil_write_mask,
            },
            bias: wgpu::DepthBiasState {
                constant: state.depth_bias as i32,
                slope_scale: state.depth_bias_slope_scale,
                clamp: state.depth_bias_clamp,
            },
        }
    }
}

impl From<StencilOperation> for wgpu::StencilOperation {
    fn from(op: StencilOperation) -> Self {
        match op {
            StencilOperation::Keep => wgpu::StencilOperation::Keep,
            StencilOperation::Zero => wgpu::StencilOperation::Zero,
            StencilOperation::Replace => wgpu::StencilOperation::Replace,
            StencilOperation::Invert => wgpu::StencilOperation::Invert,
            StencilOperation::IncrementClamp => wgpu::StencilOperation::IncrementClamp,
            StencilOperation::DecrementClamp => wgpu::StencilOperation::DecrementClamp,
            StencilOperation::IncrementWrap => wgpu::StencilOperation::IncrementWrap,
            StencilOperation::DecrementWrap => wgpu::StencilOperation::DecrementWrap,
        }
    }
}

impl From<StencilFaceState> for wgpu::StencilFaceState {
    fn from(state: StencilFaceState) -> Self {
        wgpu::StencilFaceState {
            compare: state.compare.into(),
            fail_op: state.fail_op.into(),
            depth_fail_op: state.depth_fail_op.into(),
            pass_op: state.pass_op.into(),
        }
    }
}

impl From<ColorWrite> for wgpu::ColorWrites {
    fn from(color: ColorWrite) -> Self {
        let mut result = wgpu::ColorWrites::empty();
        if color.contains(ColorWrite::RED) {
            result.insert(wgpu::ColorWrites::RED);
        }
        if color.contains(ColorWrite::BLUE) {
            result.insert(wgpu::ColorWrites::BLUE);
        }
        if color.contains(ColorWrite::GREEN) {
            result.insert(wgpu::ColorWrites::GREEN);
        }
        if color.contains(ColorWrite::ALPHA) {
            result.insert(wgpu::ColorWrites::ALPHA);
        }
        result
    }
}

impl From<BlendFactor> for wgpu::BlendFactor {
    fn from(factor: BlendFactor) -> Self {
        match factor {
            BlendFactor::Zero => wgpu::BlendFactor::Zero,
            BlendFactor::One => wgpu::BlendFactor::One,
            BlendFactor::Src => wgpu::BlendFactor::Src,
            BlendFactor::OneMinusSrc => wgpu::BlendFactor::OneMinusSrc,
            BlendFactor::SrcAlpha => wgpu::BlendFactor::SrcAlpha,
            BlendFactor::OneMinusSrcAlpha => wgpu::BlendFactor::OneMinusSrcAlpha,
            BlendFactor::Dst => wgpu::BlendFactor::Dst,
            BlendFactor::OneMinusDst => wgpu::BlendFactor::OneMinusDst,
            BlendFactor::DstAlpha => wgpu::BlendFactor::DstAlpha,
            BlendFactor::OneMinusDstAlpha => wgpu::BlendFactor::OneMinusDstAlpha,
            BlendFactor::SrcAlphaSaturated => wgpu::BlendFactor::SrcAlphaSaturated,
            BlendFactor::Constant => wgpu::BlendFactor::Constant,
            BlendFactor::OneMinusConstant => wgpu::BlendFactor::OneMinusConstant,
        }
    }
}

impl From<BlendOperation> for wgpu::BlendOperation {
    fn from(op: BlendOperation) -> Self {
        match op {
            BlendOperation::Add => wgpu::BlendOperation::Add,
            BlendOperation::Subtract => wgpu::BlendOperation::Subtract,
            BlendOperation::ReverseSubtract => wgpu::BlendOperation::ReverseSubtract,
            BlendOperation::Min => wgpu::BlendOperation::Min,
            BlendOperation::Max => wgpu::BlendOperation::Max,
        }
    }
}

impl From<BlendComponent> for wgpu::BlendComponent {
    fn from(state: BlendComponent) -> Self {
        wgpu::BlendComponent {
            src_factor: state.src_factor.into(),
            dst_factor: state.dst_factor.into(),
            operation: state.operation.into(),
        }
    }
}

impl From<BlendState> for wgpu::BlendState {
    fn from(state: BlendState) -> Self {
        wgpu::BlendState {
            color: state.color.into(),
            alpha: state.alpha.into(),
        }
    }
}

impl From<VertexStepMode> for wgpu::VertexStepMode {
    fn from(mode: VertexStepMode) -> Self {
        match mode {
            VertexStepMode::Vertex => wgpu::VertexStepMode::Vertex,
            VertexStepMode::Instance => wgpu::VertexStepMode::Instance,
        }
    }
}

impl From<VertexFormat> for wgpu::VertexFormat {
    fn from(format: VertexFormat) -> Self {
        match format {
            VertexFormat::UI8x2 => wgpu::VertexFormat::Uint8x2,
            VertexFormat::UI8x4 => wgpu::VertexFormat::Uint8x4,
            VertexFormat::I8x2 => wgpu::VertexFormat::Sint8x2,
            VertexFormat::I8x4 => wgpu::VertexFormat::Sint8x4,
            VertexFormat::UNORM8x2 => wgpu::VertexFormat::Unorm8x2,
            VertexFormat::UNORM8x4 => wgpu::VertexFormat::Unorm8x4,
            VertexFormat::SNORM8x2 => wgpu::VertexFormat::Snorm8x2,
            VertexFormat::SNORM8x4 => wgpu::VertexFormat::Snorm8x4,
            VertexFormat::UI16x2 => wgpu::VertexFormat::Uint16x2,
            VertexFormat::UI16x4 => wgpu::VertexFormat::Uint16x4,
            VertexFormat::I16x2 => wgpu::VertexFormat::Sint16x2,
            VertexFormat::I16x4 => wgpu::VertexFormat::Sint16x4,
            VertexFormat::UNORM16x2 => wgpu::VertexFormat::Unorm16x2,
            VertexFormat::UNORM16x4 => wgpu::VertexFormat::Unorm16x4,
            VertexFormat::SNORM16x2 => wgpu::VertexFormat::Snorm16x2,
            VertexFormat::SNORM16x4 => wgpu::VertexFormat::Snorm16x4,
            VertexFormat::F16x2 => wgpu::VertexFormat::Float16x2,
            VertexFormat::F16x4 => wgpu::VertexFormat::Float16x4,
            VertexFormat::F32 => wgpu::VertexFormat::Float32,
            VertexFormat::F32x2 => wgpu::VertexFormat::Float32x2,
            VertexFormat::F32x3 => wgpu::VertexFormat::Float32x3,
            VertexFormat::F32x4 => wgpu::VertexFormat::Float32x4,
        }
    }
}

impl From<ShaderStage> for wgpu::ShaderStages {
    fn from(usage: ShaderStage) -> Self {
        let mut result = wgpu::ShaderStages::empty();
        if usage.contains(ShaderStage::VERTEX) {
            result.insert(wgpu::ShaderStages::VERTEX);
        }
        if usage.contains(ShaderStage::FRAGMENT) {
            result.insert(wgpu::ShaderStages::FRAGMENT);
        }
        result
    }
}

impl From<SamplerBindingType> for wgpu::SamplerBindingType {
    fn from(ty: SamplerBindingType) -> Self {
        match ty {
            SamplerBindingType::Filtering => wgpu::SamplerBindingType::Filtering,
            SamplerBindingType::NonFiltering => wgpu::SamplerBindingType::NonFiltering,
            SamplerBindingType::Comparison => wgpu::SamplerBindingType::Comparison,
        }
    }
}

impl From<TextureSampleType> for wgpu::TextureSampleType {
    fn from(ty: TextureSampleType) -> Self {
        match ty {
            TextureSampleType::Float => wgpu::TextureSampleType::Float { filterable: true },
            TextureSampleType::Depth => wgpu::TextureSampleType::Depth,
            TextureSampleType::Int => wgpu::TextureSampleType::Sint,
            TextureSampleType::Uint => wgpu::TextureSampleType::Uint,
        }
    }
}

impl From<BindingType> for wgpu::BindingType {
    fn from(ty: BindingType) -> Self {
        match ty {
            BindingType::Buffer { dynamic_offset } => wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: dynamic_offset,
                min_binding_size: None,
            },
            BindingType::Sampler { ty } => wgpu::BindingType::Sampler(ty.into()),
            BindingType::Texture {
                sample_type,
                dimension,
                multisampled,
            } => wgpu::BindingType::Texture {
                sample_type: sample_type.into(),
                view_dimension: dimension.into(),
                multisampled,
            },
        }
    }
}

impl<'a> From<BindingResource<'a, WGPU>> for wgpu::BindingResource<'a> {
    fn from(resource: BindingResource<'a, WGPU>) -> Self {
        match resource {
            BindingResource::Buffer { buffer, offset, size } => wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &buffer.buffer,
                offset: offset as u64,
                size: core::num::NonZeroU64::new(size as u64),
            }),
            BindingResource::Sampler(sampler) => wgpu::BindingResource::Sampler(&sampler.sampler),
            BindingResource::Texture(texture) => wgpu::BindingResource::TextureView(&texture.view),
        }
    }
}
