use super::interop::{
    BindGroupId, BindGroupLayoutId, BufferId, RenderPassId, RenderPipelineId, SamplerId, ShaderId,
    TextureId,
};
use super::mugl;
use alloc::vec::Vec;
use core::ops::Deref;

/// GPU buffer.
#[derive(Debug)]
pub struct WebGLBuffer {
    pub(crate) id: BufferId,
}

impl Drop for WebGLBuffer {
    #[inline]
    fn drop(&mut self) {
        unsafe { mugl::delete_buffer(self.id) }
    }
}

/// GPU texture.
#[derive(Debug)]
pub struct WebGLTexture {
    pub(crate) id: TextureId,
}

impl Drop for WebGLTexture {
    #[inline]
    fn drop(&mut self) {
        unsafe { mugl::delete_texture(self.id) }
    }
}

/// GPU sampler.
#[derive(Debug)]
pub struct WebGLSampler {
    pub(crate) id: SamplerId,
}

impl Drop for WebGLSampler {
    #[inline]
    fn drop(&mut self) {
        unsafe { mugl::delete_sampler(self.id) }
    }
}

/// GPU shader.
#[derive(Debug)]
pub struct WebGLShader {
    pub(crate) id: ShaderId,
}

impl Drop for WebGLShader {
    #[inline]
    fn drop(&mut self) {
        unsafe { mugl::delete_shader(self.id) }
    }
}

/// GPU bind group.
#[derive(Debug)]
pub struct WebGLBindGroup {
    pub(crate) id: BindGroupId,
}

impl Drop for WebGLBindGroup {
    #[inline]
    fn drop(&mut self) {
        unsafe { mugl::delete_bind_group(self.id) }
    }
}

/// GPU bind group layout.
#[derive(Debug)]
pub struct WebGLBindGroupLayout {
    pub(crate) id: BindGroupLayoutId,
}

impl Drop for WebGLBindGroupLayout {
    #[inline]
    fn drop(&mut self) {
        unsafe { mugl::delete_bind_group_layout(self.id) }
    }
}

/// GPU render pipeline.
#[derive(Debug)]
pub struct WebGLRenderPipeline {
    pub(crate) id: RenderPipelineId,
}

impl Drop for WebGLRenderPipeline {
    #[inline]
    fn drop(&mut self) {
        unsafe { mugl::delete_render_pipeline(self.id) }
    }
}

/// GPU render pass.
#[derive(Debug)]
pub struct WebGLRenderPass {
    pub(crate) id: RenderPassId,
}

impl Drop for WebGLRenderPass {
    #[inline]
    fn drop(&mut self) {
        unsafe { mugl::delete_render_pass(self.id) }
    }
}

/// Readonly GPU buffer view.
#[derive(Debug)]
pub struct WebGLBufferView {
    pub(crate) data: Vec<u8>,
}

impl Deref for WebGLBufferView {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
