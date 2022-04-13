//! GPU interface
use alloc::boxed::Box;
use async_trait::async_trait;
use core::fmt::Debug;
use core::ops::{Deref, Range};

use crate::descriptor::{
    BindGroupDescriptor, BindGroupLayoutDescriptor, BufferDescriptor, ImageCopyExternalImage,
    ImageCopyTexture, ImageDataLayout, RenderPassDescriptor, RenderPipelineDescriptor,
    SamplerDescriptor, ShaderDescriptor, TextureDescriptor,
};
use crate::primitive::{BufferSize, Color, Extent2D, Extent3D, MipmapHint};

/// Defines a GPU backend.
pub trait GPU: Sized + for<'s> GPURefTypes<'s, Self> {
    /// GPU feature flags type.
    type Features: Debug;

    /// A GPU device.
    type Device: GPUDevice<Self>;

    /// A GPU buffer.
    type Buffer: Debug;

    /// A GPU texture.
    type Texture: Debug;

    /// A GPU texture sampler.
    type Sampler: Debug;

    /// A GPU shader.
    type Shader: Debug;

    /// A GPU render pipeline.
    type RenderPipeline: Debug;

    /// A GPU render pass.
    type RenderPass: Debug;

    /// A GPU bind group.
    type BindGroup: Debug;

    /// A GPU bind group layout.
    type BindGroupLayout: Debug;
}

/// Defines the resource reference types for a GPU backend.
pub trait GPURefTypes<'s, G: GPU> {
    /// The GPU render pass encoder type.
    type RenderPassEncoder: GPURenderPassEncoder<'s, G>;

    // A mapped view into a GPU buffer.
    type BufferView: Debug + Deref<Target = [u8]> + 's;
}

/// Defines the Web-only extension methods for a GPU backend.
pub trait GPUWebExt: GPU {
    /// A GPU external image source.
    type ImageSource: Debug;

    /// A GPU device.
    type Device: GPUDevice<Self> + GPUDeviceWebExt<Self>;
}

/// Defines the Web-only extension methods for a GPU device.
pub trait GPUDeviceWebExt<G: GPUWebExt>: GPUDevice<G> {
    /// Generates mipmap for a texture.
    fn generate_mipmap(&self, texture: &G::Texture, hint: MipmapHint);

    /// Uploads an image subregion to a GPU texture.
    fn copy_external_image_to_texture(
        &self,
        src: ImageCopyExternalImage<G>,
        dst: ImageCopyTexture<G>,
        size: Extent2D,
    );
}

/// A GPU device.
#[async_trait(?Send)]
pub trait GPUDevice<G: GPU> {
    /// Gets the enabled features for the device.
    fn features(&self) -> G::Features;

    /// Creates a Buffer.
    fn create_buffer(&self, descriptor: BufferDescriptor) -> G::Buffer;

    /// Creates a Texture.
    fn create_texture(&self, descriptor: TextureDescriptor) -> G::Texture;

    /// Creates a Sampler.
    fn create_sampler(&self, descriptor: SamplerDescriptor) -> G::Sampler;

    /// Creates a Shader.
    fn create_shader(&self, descriptor: ShaderDescriptor) -> G::Shader;

    /// Creates a RenderPipeline.
    fn create_render_pipeline(&self, descriptor: RenderPipelineDescriptor<G>) -> G::RenderPipeline;

    /// Creates a RenderPass.
    fn create_render_pass(&self, descriptor: RenderPassDescriptor<G>) -> G::RenderPass;

    /// Creates a BindGroupLayout.
    fn create_bind_group_layout(&self, descriptor: BindGroupLayoutDescriptor)
        -> G::BindGroupLayout;

    /// Creates a BindGroup.
    fn create_bind_group(&self, descriptor: BindGroupDescriptor<G>) -> G::BindGroup;

    /// Begins a render pass.
    fn render<'a>(
        &'a self,
        pass: &'a G::RenderPass,
    ) -> <G as GPURefTypes<'a, G>>::RenderPassEncoder;

    /// Asynchronously reads a Buffer.
    async fn read_buffer<'a>(
        &self,
        buffer: &'a G::Buffer,
        range: Range<BufferSize>,
    ) -> Result<<G as GPURefTypes<'a, G>>::BufferView, ()>;

    /// Submits a write operation of the provided data into a Buffer.
    fn write_buffer(&self, buffer: &G::Buffer, buffer_offset: BufferSize, data: &[u8]);

    /// Submits a command that copies data from a sub-region of a Buffer to a sub-region of another Buffer.
    fn copy_buffer(
        &self,
        src: &G::Buffer,
        src_offset: BufferSize,
        dst: &G::Buffer,
        dst_offset: BufferSize,
        size: BufferSize,
    );

    /// Submits a write operation of the provided data into a Texture.
    fn write_texture(
        &self,
        texture: ImageCopyTexture<G>,
        data: &[u8],
        layout: ImageDataLayout,
        size: Extent3D,
    );

    /// Submits a command that copies data from a sub-region of one or multiple contiguous texture subresources
    /// to another sub-region of one or multiple continuous texture subresources.
    fn copy_texture(&self, src: ImageCopyTexture<G>, dst: ImageCopyTexture<G>, size: Extent3D);

    /// Submits a command that copies data from a sub-region of a Buffer to a sub-region of another Buffer.
    fn copy_texture_to_buffer(
        &self,
        src: ImageCopyTexture<G>,
        dst: &G::Buffer,
        layout: ImageDataLayout,
        size: Extent3D,
    );

    /// Returns if the device is lost.
    fn is_lost(&self) -> bool;

    /// Submits any buffered commands
    fn flush(&self);

    /// Presents the backbuffer to screen. This implicitly calls `flush`.
    fn present(&self);

    /// Resize the surface for presentation.
    fn resize_surface(&self, size: Extent2D);
}

/// A GPU render pass encoder.
pub trait GPURenderPassEncoder<'a, G: GPU> {
    /// Sets the render pipeline
    fn pipeline(&self, pipeline: &'a G::RenderPipeline);

    /// Sets the index buffer
    fn index(&self, buffer: &'a G::Buffer);

    /// Sets the vertex buffer
    fn vertex(&self, slot: u32, buffer: &'a G::Buffer, offset: BufferSize);

    /// Sets the bind group
    fn bind_group(&self, slot: u32, bind_group: &'a G::BindGroup, offsets: &[u32]);

    /// Draws primitives
    fn draw(&self, vertices: Range<u32>, instances: Range<u32>);

    /// Draws indexed primitives
    fn draw_indexed(&self, indices: Range<u32>, instances: Range<u32>);

    /// Sets the viewport used during the rasterization stage.
    fn viewport(&self, x: f32, y: f32, width: f32, height: f32, min_depth: f32, max_depth: f32);

    /// Sets the scissor rectangle used during the rasterization stage.
    fn scissor_rect(&self, x: u32, y: u32, width: u32, height: u32);

    /// Sets the constant blend color and alpha values used with "constant" and "one-minus-constant" BlendFactors.
    fn blend_const(&self, color: Color);

    /// Sets the stencil reference value used during stencil tests with the "replace" StencilOperation.
    fn stencil_ref(&self, reference: u32);

    /// Ends and submits the render pass
    fn submit(self);
}
