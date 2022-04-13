use super::interop::{
    BindGroupId, BindGroupLayoutId, BufferId, CanvasId, ContextId, DeviceId, FutureId,
    FutureStatus, ImageSourceId, JsBindGroupEntry, JsBindGroupLayoutEntry, JsRenderPassDescriptor,
    JsRenderPipelineDescriptor, JsSamplerDescriptor, RenderPassId, RenderPipelineId, SamplerId,
    ShaderId, Slice, TextureId,
};
use super::primitive::{WebGL2Features, WebGLContextAttribute};
use crate::descriptor::{BufferDescriptor, ImageDataLayout, TextureDescriptor};
use crate::primitive::{
    BufferSize, Color, Extent2D, Extent3D, MipmapHint, Origin2D, Origin3D, ShaderStage,
};

#[link(wasm_import_module = "mugl/wasm")]
extern "C" {
    /// Gets the status of the future
    pub fn get_future_status(future: FutureId) -> FutureStatus;

    /// Loads an image from URI.
    pub fn create_image(context: ContextId, uri: Slice) -> ImageSourceId;

    /// Gets an image handle by string ID.
    pub fn get_image_by_id(context: ContextId, id: Slice) -> ImageSourceId;

    /// Deletes an image.
    pub fn delete_image(image: ImageSourceId);

    /// Gets the width of the given image.
    pub fn get_image_width(image: ImageSourceId) -> u32;

    /// Gets the height of the given image.
    pub fn get_image_height(image: ImageSourceId) -> u32;

    /// Gets a canvas handle by string ID.
    pub fn get_canvas_by_id(context: ContextId, id: Slice) -> CanvasId;

    /// Gets the width of the given canvas.
    pub fn get_canvas_width(canvas: CanvasId) -> u32;

    /// Gets the height of the given image.
    pub fn get_canvas_height(canvas: CanvasId) -> u32;

    /// Requests a WebGL2 GPU device.
    pub fn webgl_request_device(
        canvas: CanvasId,
        desc: WebGLContextAttribute,
        features: WebGL2Features,
    ) -> DeviceId;

    /// Generates mipmap for a WebGL texture.
    pub fn webgl_generate_mipmap(device: DeviceId, texture: TextureId, hint: MipmapHint);

    /// Resets the state of a GPU device.
    pub fn reset_device(device: DeviceId);

    ///Deletes a device.
    pub fn delete_device(device: DeviceId);

    ///Checks if the device is lost.
    pub fn is_device_lost(device: DeviceId) -> bool;

    /// Gets supported and enabled features of a device.
    pub fn get_device_features(device: DeviceId) -> u32;

    /// Creates a GPU buffer.
    pub fn create_buffer(device: DeviceId, descriptor: BufferDescriptor) -> BufferId;

    /// Deletes a GPU buffer.
    pub fn delete_buffer(buffer: BufferId);

    /// Creates a GPU texture.
    pub fn create_texture(device: DeviceId, descriptor: TextureDescriptor) -> TextureId;

    /// Deletes a GPU texture.
    pub fn delete_texture(texture: TextureId);

    /// Creates a GPU sampler.
    pub fn create_sampler(device: DeviceId, descriptor: JsSamplerDescriptor) -> SamplerId;

    /// Deletes a GPU sampler.
    pub fn delete_sampler(sampler: SamplerId);

    /// Creates a GPU shader module.
    pub fn create_shader(device: DeviceId, code: Slice, usage: ShaderStage) -> ShaderId;

    /// Deletes a GPU shader.
    pub fn delete_shader(shader: ShaderId);

    /// Creates a GPU bind group.
    pub fn create_bind_group(
        device: DeviceId,
        layout: BindGroupLayoutId,
        entries: Slice<JsBindGroupEntry>,
    ) -> BindGroupId;

    /// Deletes a GPU bind group.
    pub fn delete_bind_group(bind_group: BindGroupId);

    /// Creates a GPU bind group layout.
    pub fn create_bind_group_layout(
        device: DeviceId,
        entries: Slice<JsBindGroupLayoutEntry>,
    ) -> BindGroupLayoutId;

    /// Deletes a GPU bind group layout.
    pub fn delete_bind_group_layout(bind_group_layout: BindGroupLayoutId);

    /// Creates a GPU render pipeline.
    pub fn create_render_pipeline(
        device: DeviceId,
        descriptor: JsRenderPipelineDescriptor,
    ) -> RenderPipelineId;

    /// Deletes a GPU render pipeline.
    pub fn delete_render_pipeline(pipeline: RenderPipelineId);

    /// Creates a GPU render pass.
    pub fn create_render_pass(device: DeviceId, descriptor: JsRenderPassDescriptor)
        -> RenderPassId;

    /// Deletes a GPU render pass.
    pub fn delete_render_pass(pass: RenderPassId);

    /// Reads data from a GPU buffer.
    pub fn read_buffer(
        device: DeviceId,
        src: BufferId,
        src_offset: BufferSize,
        out: Slice,
    ) -> FutureId;

    ///Writes data to a GPU buffer.
    pub fn write_buffer(device: DeviceId, buffer: BufferId, data: Slice, offset: BufferSize);

    /// Copies data from a GPU buffer to another buffer.
    pub fn copy_buffer(
        device: DeviceId,
        src: BufferId,
        dst: BufferId,
        size: BufferSize,
        src_offset: BufferSize,
        dst_offset: BufferSize,
    );

    /// Writes subregion of data array to a GPU texture.
    pub fn write_texture(
        device: DeviceId,
        texture: TextureId,
        mip_level: u32,
        origin: Origin3D,
        data: Slice,
        layout: ImageDataLayout,
        size: Extent3D,
    );

    /// Copies subregion of a GPU texture to another texture.
    pub fn copy_texture(
        device: DeviceId,
        src: TextureId,
        src_mip_level: u32,
        src_origin: Origin3D,
        dst: TextureId,
        dst_mip_evel: u32,
        dst_origin: Origin3D,
        size: Extent3D,
    );

    /// Copies subregion of a GPU texture to a GPU buffer.
    pub fn copy_texture_to_buffer(
        device: DeviceId,
        src: TextureId,
        src_mip_level: u32,
        src_origin: Origin3D,
        dst: BufferId,
        layout: ImageDataLayout,
        size: Extent3D,
    );

    /// Uploads an image subregion to a GPU texture.
    pub fn copy_external_image_to_texture(
        device: DeviceId,
        src: ImageSourceId,
        src_origin: Origin2D,
        dst: TextureId,
        dst_mip_level: u32,
        dst_origin: Origin3D,
        size: Extent2D,
    );

    /// Begins a render pass.
    pub fn begin_render_pass(device: DeviceId, pass: RenderPassId);

    /// Submits the current render pass.
    pub fn submit_render_pass(device: DeviceId);

    /// Binds a render pipeline to the current render pass.
    pub fn set_render_pipeline(device: DeviceId, pipeline: RenderPipelineId);

    /// Binds an index buffer to the current render pass.
    pub fn set_index(device: DeviceId, buffer: BufferId);

    /// Binds a vertex buffer to a slot in the current render pass.
    pub fn set_vertex(device: DeviceId, slot: u32, buffer: BufferId, offset: BufferSize);

    /// Binds a bind group to the current render pass.
    pub fn set_bind_group(
        device: DeviceId,
        slot: u32,
        bind_group: BindGroupId,
        offsets: Slice<u32>,
    );

    /// Submits a draw call in the current render pass.
    pub fn draw(
        device: DeviceId,
        vertex_count: u32,
        instance_count: u32,
        firstvertex: u32,
        first_instance: u32,
    );

    /// Submits an indexed draw call in the current render pass.
    pub fn draw_indexed(
        device: DeviceId,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        first_instance: u32,
    );

    /// Sets the 3D viewport area for the current render pass.
    pub fn set_viewport(
        device: DeviceId,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        min_depth: f32,
        max_depth: f32,
    );

    /// Sets the scissor rectangle for the current render pass.
    pub fn set_scissor_rect(device: DeviceId, x: u32, y: u32, width: u32, height: u32);

    /// Sets the blend_constant color for the current render pass.
    pub fn set_blend_const(device: DeviceId, color: Color<f32>);

    /// Sets the stencil reference value for the current render pass.
    pub fn set_stencil_ref(device: DeviceId, reference: u32);
}
