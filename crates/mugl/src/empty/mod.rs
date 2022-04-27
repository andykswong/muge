//! Empty backend

use alloc::boxed::Box;
use async_trait::async_trait;
use core::ops::Range;

use crate::{
    BufferDescriptor, BufferSize, Color, Extent2D, Extent3D, GPUDevice, GPUDeviceWebExt,
    GPURefTypes, GPURenderPassEncoder, GPUWebExt, ImageCopyExternalImage, ImageCopyTexture,
    ImageDataLayout, MipmapHint, RenderPassDescriptor, RenderPipelineDescriptor, SamplerDescriptor,
    ShaderDescriptor, TextureDescriptor, GPU,
};

/// Empty GPU backend
#[derive(Debug)]
pub struct EmptyGPU;

/// Empty GPU device
#[derive(Debug)]
pub struct EmptyGPUDevice;

/// Empty / No-op GPU render pass encoder
#[derive(Debug)]
pub struct EmptyGPURenderPassEncoder;

impl GPU for EmptyGPU {
    type Features = ();
    type Device = EmptyGPUDevice;
    type Buffer = ();
    type Texture = ();
    type Sampler = ();
    type Shader = ();
    type RenderPipeline = ();
    type RenderPass = ();
    type BindGroup = ();
    type BindGroupLayout = ();
}

impl<'a> GPURefTypes<'a, EmptyGPU> for EmptyGPU {
    type RenderPassEncoder = EmptyGPURenderPassEncoder;
    type BufferView = &'a [u8];
}

impl GPUWebExt for EmptyGPU {
    type ImageSource = ();
    type Device = EmptyGPUDevice;
}

#[async_trait(?Send)]
impl GPUDevice<EmptyGPU> for EmptyGPUDevice {
    fn features(&self) -> () {}

    fn create_buffer(&self, _descriptor: BufferDescriptor) -> () {}

    fn create_texture(&self, _descriptor: TextureDescriptor) -> () {}

    fn create_sampler(&self, _descriptor: SamplerDescriptor) -> () {}

    fn create_shader(&self, _descriptor: ShaderDescriptor) -> () {}

    fn create_render_pipeline(&self, _descriptor: RenderPipelineDescriptor<EmptyGPU>) -> () {}

    fn create_render_pass(&self, _descriptor: RenderPassDescriptor<EmptyGPU>) -> () {}

    fn create_bind_group_layout(
        &self,
        _descriptor: crate::BindGroupLayoutDescriptor,
    ) -> <EmptyGPU as GPU>::BindGroupLayout {
        todo!()
    }

    fn create_bind_group(
        &self,
        _descriptor: crate::BindGroupDescriptor<EmptyGPU>,
    ) -> <EmptyGPU as GPU>::BindGroup {
        todo!()
    }

    fn render<'a>(&'a self, _pass: &'a ()) -> EmptyGPURenderPassEncoder {
        EmptyGPURenderPassEncoder
    }

    async fn read_buffer<'a>(
        &self,
        _buffer: &'a (),
        _range: Range<BufferSize>,
    ) -> Result<&'a [u8], ()> {
        Ok(&[])
    }

    fn write_buffer(&self, _buffer: &(), _buffer_offset: BufferSize, _data: &[u8]) {}

    fn copy_buffer(
        &self,
        _src: &(),
        _src_offset: BufferSize,
        _dst: &(),
        _dst_offset: BufferSize,
        _size: BufferSize,
    ) {
    }

    fn write_texture(
        &self,
        _texture: ImageCopyTexture<EmptyGPU>,
        _data: &[u8],
        _layout: ImageDataLayout,
        _size: Extent3D,
    ) {
    }

    fn copy_texture(
        &self,
        _src: ImageCopyTexture<EmptyGPU>,
        _dst: ImageCopyTexture<EmptyGPU>,
        _size: Extent3D,
    ) {
    }

    fn copy_texture_to_buffer(
        &self,
        _src: ImageCopyTexture<EmptyGPU>,
        _dst: &(),
        _layout: ImageDataLayout,
        _size: Extent3D,
    ) {
    }

    fn is_srgb_surface(&self) -> bool {
        false
    }

    fn is_lost(&self) -> bool {
        false
    }

    fn flush(&self) {}

    fn present(&self) {}

    fn resize_surface(&self, _size: Extent2D) {}
}

impl GPUDeviceWebExt<EmptyGPU> for EmptyGPUDevice {
    fn generate_mipmap(&self, _texture: &(), _hint: MipmapHint) {}

    fn copy_external_image_to_texture(
        &self,
        _src: ImageCopyExternalImage<EmptyGPU>,
        _dst: ImageCopyTexture<EmptyGPU>,
        _size: Extent2D,
    ) {
    }
}

impl<'a> GPURenderPassEncoder<'a, EmptyGPU> for EmptyGPURenderPassEncoder {
    fn pipeline(&self, _pipeline: &'a ()) {}

    fn index(&self, _buffer: &'a ()) {}

    fn vertex(&self, _slot: u32, _buffer: &'a (), _offset: BufferSize) {}

    fn bind_group(&self, _slot: u32, _bind_group: &'a (), _offsets: &[u32]) {}

    fn draw(&self, _vertices: Range<u32>, _instances: Range<u32>) {}

    fn draw_indexed(&self, _indices: Range<u32>, _instances: Range<u32>) {}

    fn viewport(
        &self,
        _x: f32,
        _y: f32,
        _width: f32,
        _height: f32,
        _min_depth: f32,
        _max_depth: f32,
    ) {
    }

    fn scissor_rect(&self, _x: u32, _y: u32, _width: u32, _height: u32) {}

    fn blend_const(&self, _color: Color) {}

    fn stencil_ref(&self, _reference: u32) {}

    fn submit(self) {}
}
