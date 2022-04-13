use async_trait::async_trait;
use mugl::prelude::*;

use crate::common::{App, ColorVertex, TRIANGLE};

#[derive(Debug)]
pub struct BasicExample {
    device: Device,
    pipeline: RenderPipeline,
    pass: RenderPass,
    buffer: Buffer,
}

#[async_trait(?Send)]
impl App for BasicExample {
    fn new(device: Device, _size: mugl::Extent2D) -> Self {
        let vertices = bytemuck::cast_slice(TRIANGLE);
        let buffer = device.create_buffer(BufferDescriptor {
            usage: BufferUsage::VERTEX,
            size: vertices.len() as BufferSize,
        });
        device.write_buffer(&buffer, 0, vertices);

        cfg_if::cfg_if! {
            if #[cfg(feature = "backend-wgpu")] {
                let vertex = &device.create_shader(ShaderDescriptor {
                    usage: ShaderStage::VERTEX | ShaderStage::FRAGMENT,
                    code: include_str!("../shader/basic.wgsl").into(),
                });
                let fragment = vertex;
            } else {
                let vertex = &device.create_shader(ShaderDescriptor {
                    usage: ShaderStage::VERTEX,
                    code: include_str!("../shader/basic.vs.glsl").into(),
                });
                let fragment = &device.create_shader(ShaderDescriptor {
                    usage: ShaderStage::FRAGMENT,
                    code: include_str!("../shader/basic.fs.glsl").into(),
                });
            }
        }

        let pipeline = device.create_render_pipeline(RenderPipelineDescriptor {
            vertex,
            fragment,
            buffers: &[VertexBufferLayout {
                stride: core::mem::size_of::<ColorVertex>() as BufferSize,
                step_mode: VertexStepMode::Vertex,
                attributes: &[
                    VertexAttribute {
                        offset: 0,
                        shader_location: 0,
                        format: VertexFormat::F32x3,
                    },
                    VertexAttribute {
                        offset: core::mem::size_of::<[f32; 3]>() as BufferSize,
                        shader_location: 1,
                        format: VertexFormat::F32x4,
                    },
                ],
            }],
            bind_groups: &[],
            targets: Default::default(),
            primitive: Default::default(),
            depth_stencil: Default::default(),
            multisample: Default::default(),
        });

        let pass = device.create_render_pass(RenderPassDescriptor::Default {
            clear_color: Some(Color(0.1, 0.2, 0.3, 1.0)),
            clear_depth: None,
            clear_stencil: None,
        });

        Self {
            device,
            pipeline,
            pass,
            buffer,
        }
    }

    fn device(&self) -> &Device {
        &self.device
    }

    fn render(&mut self, _t: f64) -> bool {
        {
            let encoder = self.device.render(&self.pass);
            encoder.pipeline(&self.pipeline);
            encoder.vertex(0, &self.buffer, 0);
            encoder.draw(0..3, 0..1);
            encoder.submit();
        }
        self.device.present();
        false
    }
}
