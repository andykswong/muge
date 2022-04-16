use alloc::boxed::Box;
use alloc::vec::Vec;
use async_trait::async_trait;
use core::ops::Range;

use super::dom::{Canvas, ImageSource, JsFuture};
use super::interop::{
    DeviceId, JsBindGroupEntry, JsBindGroupLayoutEntry, JsColorAttachment, JsColorTargetState,
    JsRenderPassDescriptor, JsRenderPipelineDescriptor, JsVertexBufferLayout, Slice, TextureId,
};
use super::mugl;
use super::primitive::{WebGL2Features, WebGLContextAttribute};
use super::resource::{
    WebGLBindGroup, WebGLBindGroupLayout, WebGLBuffer, WebGLBufferView, WebGLRenderPass,
    WebGLRenderPipeline, WebGLSampler, WebGLShader, WebGLTexture,
};
use crate::descriptor::{
    BindGroupDescriptor, BindGroupLayoutDescriptor, BlendState, BufferDescriptor,
    ColorTargetStates, DepthStencilState, ImageCopyExternalImage, ImageCopyTexture,
    ImageDataLayout, RenderPassDescriptor, RenderPipelineDescriptor, SamplerDescriptor,
    ShaderDescriptor, TextureDescriptor, VertexAttribute,
};
use crate::gpu::{GPUDevice, GPUDeviceWebExt, GPURefTypes, GPURenderPassEncoder, GPUWebExt, GPU};
use crate::primitive::{BufferSize, Color, ColorWrite, Extent2D, Extent3D, MipmapHint};

/// WebGL GPU interface.
#[derive(Debug)]
pub struct WebGL;

impl WebGL {
    /// Requests a new WebGL GPU device.
    pub fn request_device(
        canvas: &Canvas,
        desc: WebGLContextAttribute,
        features: WebGL2Features,
    ) -> Option<WebGLDevice> {
        let id = unsafe { mugl::webgl_request_device(canvas.id, desc, features) };
        if id.is_null() {
            None
        } else {
            Some(WebGLDevice { id })
        }
    }
}

/// WebGL GPU device.
#[derive(Debug)]
pub struct WebGLDevice {
    id: DeviceId,
}

/// WebGL GPU render pass encoder.
#[derive(Debug)]
pub struct WebGLRenderPassEncoder<'a> {
    device: &'a WebGLDevice,
}

impl GPU for WebGL {
    type Features = WebGL2Features;
    type Device = WebGLDevice;
    type Buffer = WebGLBuffer;
    type Texture = WebGLTexture;
    type Sampler = WebGLSampler;
    type Shader = WebGLShader;
    type RenderPipeline = WebGLRenderPipeline;
    type RenderPass = WebGLRenderPass;
    type BindGroup = WebGLBindGroup;
    type BindGroupLayout = WebGLBindGroupLayout;
}

impl<'a> GPURefTypes<'a, WebGL> for WebGL {
    type RenderPassEncoder = WebGLRenderPassEncoder<'a>;
    type BufferView = WebGLBufferView;
}

impl GPUWebExt for WebGL {
    type ImageSource = ImageSource;

    type Device = WebGLDevice;
}

impl WebGLDevice {
    /// Resets the state of the GPU device.
    pub fn reset(&self) {
        unsafe { mugl::reset_device(self.id) }
    }
}

impl Drop for WebGLDevice {
    fn drop(&mut self) {
        unsafe { mugl::delete_device(self.id) }
    }
}

#[async_trait(?Send)]
impl GPUDevice<WebGL> for WebGLDevice {
    fn features(&self) -> WebGL2Features {
        unsafe { WebGL2Features::from_bits_unchecked(mugl::get_device_features(self.id)) }
    }

    fn create_buffer(&self, descriptor: BufferDescriptor) -> WebGLBuffer {
        WebGLBuffer {
            id: unsafe { mugl::create_buffer(self.id, descriptor) },
        }
    }

    fn create_texture(&self, descriptor: TextureDescriptor) -> WebGLTexture {
        WebGLTexture {
            id: unsafe { mugl::create_texture(self.id, descriptor) },
        }
    }

    fn create_sampler(&self, descriptor: SamplerDescriptor) -> WebGLSampler {
        WebGLSampler {
            id: unsafe { mugl::create_sampler(self.id, descriptor.into()) },
        }
    }

    fn create_shader(&self, descriptor: ShaderDescriptor) -> WebGLShader {
        WebGLShader {
            id: unsafe { mugl::create_shader(self.id, descriptor.code.into(), descriptor.usage) },
        }
    }

    fn create_bind_group_layout(
        &self,
        descriptor: BindGroupLayoutDescriptor,
    ) -> WebGLBindGroupLayout {
        let entries: Vec<JsBindGroupLayoutEntry> = descriptor
            .entries
            .iter()
            .map(Into::into)
            .collect::<Vec<_>>();

        WebGLBindGroupLayout {
            id: unsafe { mugl::create_bind_group_layout(self.id, (&entries).into()) },
        }
    }

    fn create_bind_group(&self, descriptor: BindGroupDescriptor<WebGL>) -> WebGLBindGroup {
        let entries: Vec<JsBindGroupEntry> = descriptor
            .entries
            .iter()
            .map(Into::into)
            .collect::<Vec<_>>();

        WebGLBindGroup {
            id: unsafe {
                mugl::create_bind_group(self.id, descriptor.layout.id, (&entries).into())
            },
        }
    }

    fn create_render_pipeline(
        &self,
        descriptor: RenderPipelineDescriptor<WebGL>,
    ) -> WebGLRenderPipeline {
        let mut attributes = Vec::<VertexAttribute>::new();
        let buffers = descriptor
            .buffers
            .iter()
            .map(|buffer| {
                let attributes_offset = attributes.len() as u32;
                for attribute in buffer.attributes {
                    attributes.push(*attribute);
                }
                JsVertexBufferLayout {
                    attributes_offset,
                    attributes_len: buffer.attributes.len() as u32,
                    stride: buffer.stride,
                    step_mode: buffer.step_mode,
                }
            })
            .collect::<Vec<_>>();
        let bind_groups = descriptor
            .bind_groups
            .iter()
            .map(|bind_group| bind_group.id)
            .collect::<Vec<_>>();
        let offscreen_targets: Vec<JsColorTargetState>;

        let depth_stencil = descriptor
            .depth_stencil
            .unwrap_or(DepthStencilState::default());

        let color_states = match descriptor.targets {
            ColorTargetStates::Default { write_mask, blend } => (
                Slice::empty(),
                write_mask,
                blend.unwrap_or(BlendState::default()),
            ),
            ColorTargetStates::Offscreen { targets } => {
                offscreen_targets = targets.iter().map(Into::into).collect();
                (
                    (&offscreen_targets).into(),
                    ColorWrite::default(),
                    BlendState::default(),
                )
            }
        };

        let js_desc = JsRenderPipelineDescriptor {
            vertex: descriptor.vertex.id,
            fragment: descriptor.fragment.id,
            attributes: (&attributes).into(),
            buffers: (&buffers).into(),
            bind_groups: (&bind_groups).into(),
            topology: descriptor.primitive.topology,
            index_format: descriptor
                .primitive
                .index_format
                .map(|format| format as u32)
                .unwrap_or(0),
            front_face: descriptor.primitive.front_face,
            cull_mode: descriptor.primitive.cull_mode,
            sample_count: descriptor.multisample.count,
            alpha_to_coverage: descriptor.multisample.alpha_to_coverage as u32,
            has_depth_stencil: descriptor.depth_stencil.is_some() as u32,
            depth_stencil_format: depth_stencil.format,
            depth_write: depth_stencil.depth_write as u32,
            depth_compare: depth_stencil.depth_compare,
            stencil_front: depth_stencil.stencil_front,
            stencil_back: depth_stencil.stencil_back,
            stencil_read_mask: depth_stencil.stencil_read_mask,
            stencil_write_mask: depth_stencil.stencil_write_mask,
            depth_bias: depth_stencil.depth_bias,
            depth_bias_slope_scale: depth_stencil.depth_bias_slope_scale,
            depth_bias_clamp: depth_stencil.depth_bias_clamp,
            targets: color_states.0,
            write_mask: color_states.1,
            blend: color_states.2,
        };

        WebGLRenderPipeline {
            id: unsafe { mugl::create_render_pipeline(self.id, js_desc) },
        }
    }

    fn create_render_pass(&self, descriptor: RenderPassDescriptor<WebGL>) -> WebGLRenderPass {
        let color_atts: Vec<JsColorAttachment>;
        let js_desc = match descriptor {
            RenderPassDescriptor::Default {
                clear_depth,
                clear_stencil,
                clear_color,
            } => JsRenderPassDescriptor {
                clear_depth: clear_depth.unwrap_or(f32::NAN),
                clear_stencil: clear_stencil.map(|s| s as f32).unwrap_or(f32::NAN),
                clear_color: clear_color.into(),
                is_offscreen: 0.,
                texture: TextureId::null(),
                mip_level: 0,
                slice: 0,
                colors: Slice::empty(),
            },
            RenderPassDescriptor::Offscreen {
                clear_depth,
                clear_stencil,
                depth_stencil,
                colors,
            } => {
                color_atts = colors
                    .iter()
                    .map(Into::into)
                    .collect::<Vec<JsColorAttachment>>();
                JsRenderPassDescriptor {
                    clear_depth: clear_depth.unwrap_or(f32::NAN),
                    clear_stencil: clear_stencil
                        .map(|stencil| stencil as f32)
                        .unwrap_or(f32::NAN),
                    clear_color: Color::none(),
                    is_offscreen: 1.,
                    texture: depth_stencil
                        .map(|ds| ds.texture.id)
                        .unwrap_or(TextureId::null()),
                    mip_level: depth_stencil.map(|ds| ds.mip_level).unwrap_or(0),
                    slice: depth_stencil.map(|ds| ds.slice).unwrap_or(0),
                    colors: (&color_atts).into(),
                }
            }
        };
        WebGLRenderPass {
            id: unsafe { mugl::create_render_pass(self.id, js_desc) },
        }
    }

    fn render<'a>(&'a self, pass: &'a WebGLRenderPass) -> WebGLRenderPassEncoder {
        unsafe {
            mugl::begin_render_pass(self.id, pass.id);
        }
        WebGLRenderPassEncoder { device: self }
    }

    async fn read_buffer<'a>(
        &self,
        buffer: &'a WebGLBuffer,
        range: Range<BufferSize>,
    ) -> Result<WebGLBufferView, ()> {
        let len = range.len();
        let mut out = Vec::<u8>::with_capacity(len);
        unsafe { out.set_len(len) };

        JsFuture::new(unsafe {
            mugl::read_buffer(
                self.id,
                buffer.id,
                range.start,
                Slice::from_raw_parts(out.as_ptr(), len),
            )
        })
        .await?;

        Ok(WebGLBufferView { data: out })
    }

    fn write_buffer(&self, buffer: &WebGLBuffer, buffer_offset: BufferSize, data: &[u8]) {
        unsafe { mugl::write_buffer(self.id, buffer.id, data.into(), buffer_offset) }
    }

    fn copy_buffer(
        &self,
        src: &WebGLBuffer,
        src_offset: BufferSize,
        dst: &WebGLBuffer,
        dst_offset: BufferSize,
        size: BufferSize,
    ) {
        unsafe { mugl::copy_buffer(self.id, src.id, dst.id, size, src_offset, dst_offset) }
    }

    fn write_texture(
        &self,
        texture: ImageCopyTexture<WebGL>,
        data: &[u8],
        layout: ImageDataLayout,
        size: Extent3D,
    ) {
        unsafe {
            mugl::write_texture(
                self.id,
                texture.texture.id,
                texture.mip_level,
                texture.origin,
                data.into(),
                layout,
                size,
            )
        }
    }

    fn copy_texture(
        &self,
        src: ImageCopyTexture<WebGL>,
        dst: ImageCopyTexture<WebGL>,
        size: Extent3D,
    ) {
        unsafe {
            mugl::copy_texture(
                self.id,
                src.texture.id,
                src.mip_level,
                src.origin,
                dst.texture.id,
                dst.mip_level,
                dst.origin,
                size,
            )
        }
    }

    fn copy_texture_to_buffer(
        &self,
        src: ImageCopyTexture<WebGL>,
        dst: &WebGLBuffer,
        layout: ImageDataLayout,
        size: Extent3D,
    ) {
        unsafe {
            mugl::copy_texture_to_buffer(
                self.id,
                src.texture.id,
                src.mip_level,
                src.origin,
                dst.id,
                layout,
                size,
            )
        }
    }

    fn is_lost(&self) -> bool {
        unsafe { mugl::is_device_lost(self.id) }
    }

    fn flush(&self) {
        // noop
    }

    fn present(&self) {
        // noop
    }

    fn resize_surface(&self, _size: Extent2D) {
        // noop
    }
}

impl GPUDeviceWebExt<WebGL> for WebGLDevice {
    fn generate_mipmap(&self, texture: &WebGLTexture, hint: MipmapHint) {
        unsafe { mugl::webgl_generate_mipmap(self.id, texture.id, hint) }
    }

    fn copy_external_image_to_texture(
        &self,
        src: ImageCopyExternalImage<WebGL>,
        dst: ImageCopyTexture<WebGL>,
        size: Extent2D,
    ) {
        unsafe {
            mugl::copy_external_image_to_texture(
                self.id,
                src.src.id,
                src.origin,
                dst.texture.id,
                dst.mip_level,
                dst.origin,
                size,
            )
        }
    }
}

impl<'a> GPURenderPassEncoder<'a, WebGL> for WebGLRenderPassEncoder<'a> {
    fn pipeline(&self, pipeline: &'a WebGLRenderPipeline) {
        unsafe { mugl::set_render_pipeline(self.device.id, pipeline.id) }
    }

    fn index(&self, buffer: &'a WebGLBuffer) {
        unsafe { mugl::set_index(self.device.id, buffer.id) }
    }

    fn vertex(&self, slot: u32, buffer: &'a WebGLBuffer, offset: BufferSize) {
        unsafe { mugl::set_vertex(self.device.id, slot, buffer.id, offset) }
    }

    fn bind_group(&self, slot: u32, bind_group: &'a WebGLBindGroup, offsets: &[u32]) {
        unsafe { mugl::set_bind_group(self.device.id, slot, bind_group.id, offsets.into()) }
    }

    fn draw(&self, vertices: Range<u32>, instances: Range<u32>) {
        unsafe {
            mugl::draw(
                self.device.id,
                vertices.len() as u32,
                instances.len() as u32,
                vertices.start,
                instances.start,
            )
        }
    }

    fn draw_indexed(&self, indices: Range<u32>, instances: Range<u32>) {
        unsafe {
            mugl::draw_indexed(
                self.device.id,
                indices.len() as u32,
                instances.len() as u32,
                indices.start,
                instances.start,
            )
        }
    }

    fn viewport(&self, x: f32, y: f32, width: f32, height: f32, min_depth: f32, max_depth: f32) {
        unsafe {
            mugl::set_viewport(
                self.device.id,
                x as u32,
                y as u32,
                width as u32,
                height as u32,
                min_depth,
                max_depth,
            )
        }
    }

    fn scissor_rect(&self, x: u32, y: u32, width: u32, height: u32) {
        unsafe { mugl::set_scissor_rect(self.device.id, x, y, width, height) }
    }

    fn blend_const(&self, color: Color) {
        unsafe { mugl::set_blend_const(self.device.id, color.into()) }
    }

    fn stencil_ref(&self, reference: u32) {
        unsafe { mugl::set_stencil_ref(self.device.id, reference) }
    }

    fn submit(self) {
        // noop. Submit pass on drop
    }
}

impl<'a> Drop for WebGLRenderPassEncoder<'a> {
    fn drop(&mut self) {
        // We always submit the render pass.
        // WebGL commands are executed in immediate mode anyway so there is no reason not to submit.
        unsafe { mugl::submit_render_pass(self.device.id) }
    }
}
