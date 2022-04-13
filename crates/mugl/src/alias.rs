use crate::gpu::{GPURefTypes, GPUWebExt, GPU};

cfg_if::cfg_if! {
    if #[cfg(all(target_family = "wasm", feature = "backend-webgl"))] {
        /// The default GPU device type. Set to WebGL.
        pub type DefaultGPU = crate::webgl::WebGL;
    } else if #[cfg(feature = "backend-wgpu")] {
        /// The default GPU device type. Set to WGPU.
        pub type DefaultGPU = crate::wgpu::WGPU;
    } else {
        /// The default GPU device type. Set to an empty backend as there is no compatible backend found.
        pub type DefaultGPU = crate::empty::EmptyGPU;
    }
}

/// A GPU device.
pub type Device<G = DefaultGPU> = <G as GPU>::Device;

/// GPU device features.
pub type Features<G = DefaultGPU> = <G as GPU>::Features;

/// A GPU buffer.
pub type Buffer<G = DefaultGPU> = <G as GPU>::Buffer;

/// A GPU texture.
pub type Texture<G = DefaultGPU> = <G as GPU>::Texture;

/// A GPU texture sampler.
pub type Sampler<G = DefaultGPU> = <G as GPU>::Sampler;

/// A GPU shader.
pub type Shader<G = DefaultGPU> = <G as GPU>::Shader;

/// A GPU render pipeline.
pub type RenderPipeline<G = DefaultGPU> = <G as GPU>::RenderPipeline;

/// A GPU render pass.
pub type RenderPass<G = DefaultGPU> = <G as GPU>::RenderPass;

/// A GPU bind group.
pub type BindGroup<G = DefaultGPU> = <G as GPU>::BindGroup;

/// A GPU bind group layout.
pub type BindGroupLayout<G = DefaultGPU> = <G as GPU>::BindGroupLayout;

/// The GPU render pass encoder type.
pub type RenderPassEncoder<'a, G = DefaultGPU> = <G as GPURefTypes<'a, G>>::RenderPassEncoder;

// A mapped view into a GPU buffer.
pub type BufferView<'a, G = DefaultGPU> = <G as GPURefTypes<'a, G>>::BufferView;

/// An external image source.
pub type ImageSource<G = DefaultGPU> = <G as GPUWebExt>::ImageSource;
