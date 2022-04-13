use async_trait::async_trait;
use mugl::prelude::*;

use crate::common::App;

const PI_2: f32 = std::f32::consts::PI * 2.0;
const N: usize = 10; // N * N triangles
const INSTANCE_COUNT: u32 = (N * N) as u32;

const INDICES: &[u16] = &[0, 1, 2, 0]; // last index is a padding
const POSITIONS: &[f32] = &[0.0, -0.05, -0.05, 0.0, 0.05, 0.05];

static mut OFFSET_COLORS: &mut [f32] = &mut [0.; N * N * 5];
static mut ANGLES: &mut [f32] = &mut [0.; N * N * 5];

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Data {
    ambient: Color<f32>,
}

unsafe impl bytemuck::Pod for Data {}
unsafe impl bytemuck::Zeroable for Data {}

pub struct InstancingExample {
    device: Device,
    pipeline: RenderPipeline,
    pass: RenderPass,
    indices: Buffer,
    vertices: Buffer,
    offset_colors: Buffer,
    angles: Buffer,
    bind_group: BindGroup,
    ambient: Buffer,
}

#[async_trait(?Send)]
impl App for InstancingExample {
    fn new(device: Device, _size: mugl::Extent2D) -> Self {
        let indices_data = bytemuck::cast_slice(INDICES);
        let indices = device.create_buffer(BufferDescriptor {
            usage: BufferUsage::INDEX,
            size: indices_data.len() as BufferSize,
        });
        device.write_buffer(&indices, 0, indices_data);

        let vertices_data = bytemuck::cast_slice(POSITIONS);
        let vertices = device.create_buffer(BufferDescriptor {
            usage: BufferUsage::VERTEX,
            size: vertices_data.len() as BufferSize,
        });
        device.write_buffer(&vertices, 0, vertices_data);

        unsafe { generate_offset_colors() };
        let offset_colors_data = bytemuck::cast_slice(unsafe { OFFSET_COLORS });
        let offset_colors = device.create_buffer(BufferDescriptor {
            usage: BufferUsage::VERTEX,
            size: offset_colors_data.len() as BufferSize,
        });
        device.write_buffer(&offset_colors, 0, offset_colors_data);

        let angles = device.create_buffer(BufferDescriptor {
            usage: BufferUsage::VERTEX | BufferUsage::STREAM,
            size: bytemuck::cast_slice::<f32, u8>(unsafe { ANGLES }).len() as BufferSize,
        });

        let ambient_size = std::mem::size_of::<Data>() as BufferSize;
        let ambient = device.create_buffer(BufferDescriptor {
            usage: BufferUsage::UNIFORM | BufferUsage::STREAM,
            size: ambient_size,
        });

        let layout = device.create_bind_group_layout(BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                label: "Data",
                binding: 0,
                visibility: ShaderStage::FRAGMENT,
                ty: BindingType::Buffer {
                    dynamic_offset: false,
                },
            }],
        });

        let bind_group = device.create_bind_group(BindGroupDescriptor {
            layout: &layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer {
                    buffer: &ambient,
                    offset: 0,
                    size: ambient_size,
                },
            }],
        });

        cfg_if::cfg_if! {
            if #[cfg(feature = "backend-wgpu")] {
                let vertex = &device.create_shader(ShaderDescriptor {
                    usage: ShaderStage::VERTEX | ShaderStage::FRAGMENT,
                    code: include_str!("../shader/instancing.wgsl").into(),
                });
                let fragment = vertex;
            } else {
                let vertex = &device.create_shader(ShaderDescriptor {
                    usage: ShaderStage::VERTEX,
                    code: include_str!("../shader/instancing.vs.glsl").into(),
                });

                let fragment = &device.create_shader(ShaderDescriptor {
                    usage: ShaderStage::FRAGMENT,
                    code: include_str!("../shader/instancing.fs.glsl").into(),
                });
            }
        }

        let pipeline = device.create_render_pipeline(RenderPipelineDescriptor {
            vertex,
            fragment,
            buffers: &[
                VertexBufferLayout {
                    stride: std::mem::size_of::<[f32; 2]>() as BufferSize,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &[VertexAttribute {
                        offset: 0,
                        shader_location: 0,
                        format: VertexFormat::F32x2,
                    }],
                },
                VertexBufferLayout {
                    stride: std::mem::size_of::<[f32; 5]>() as BufferSize,
                    step_mode: VertexStepMode::Instance,
                    attributes: &[
                        VertexAttribute {
                            offset: 0,
                            shader_location: 1,
                            format: VertexFormat::F32x2,
                        },
                        VertexAttribute {
                            offset: std::mem::size_of::<[f32; 2]>() as BufferSize,
                            shader_location: 2,
                            format: VertexFormat::F32x3,
                        },
                    ],
                },
                VertexBufferLayout {
                    stride: std::mem::size_of::<f32>() as BufferSize,
                    step_mode: VertexStepMode::Instance,
                    attributes: &[VertexAttribute {
                        offset: 0,
                        shader_location: 3,
                        format: VertexFormat::F32,
                    }],
                },
            ],
            bind_groups: &[&layout],
            targets: Default::default(),
            primitive: PrimitiveState {
                index_format: Some(IndexFormat::UI16),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: Default::default(),
        });

        let pass = device.create_render_pass(RenderPassDescriptor::Default {
            clear_color: Some(Color(0., 0., 0., 1.0)),
            clear_depth: None,
            clear_stencil: None,
        });

        Self {
            device,
            pipeline,
            pass,
            indices,
            vertices,
            offset_colors,
            angles,
            ambient,
            bind_group,
        }
    }

    fn device(&self) -> &Device {
        &self.device
    }

    fn render(&mut self, t: f64) -> bool {
        let t = t as f32;

        for i in 0..(N * N) {
            unsafe {
                ANGLES[i] = PI_2 * i as f32 / (N * N) as f32 + t;
                ANGLES[i] -= (ANGLES[i] / PI_2).trunc() * PI_2;
            }
        }
        self
            .device
            .write_buffer(&self.angles, 0, bytemuck::cast_slice(unsafe { ANGLES }));

        let a = f32::sin(t) / 2.0;
        let data = &[Data {
            ambient: Color::<f32>(a, a, a, 1.0),
        }];
        self
            .device
            .write_buffer(&self.ambient, 0, bytemuck::cast_slice(data));

        {
            let encoder = self.device.render(&self.pass);
            encoder.pipeline(&self.pipeline);
            encoder.index(&self.indices);
            encoder.vertex(0, &self.vertices, 0);
            encoder.vertex(1, &self.offset_colors, 0);
            encoder.vertex(2, &self.angles, 0);
            encoder.bind_group(0, &self.bind_group, &[]);
            encoder.draw_indexed(0..3, 0..INSTANCE_COUNT);
            encoder.submit();
        }
        self.device.present();
        true
    }
}

unsafe fn generate_offset_colors() {
    let n = N as f32;
    for i in 0..(N * N) {
        // Offsets
        OFFSET_COLORS[5 * i] = -1.0 + 2.0 * (i / N) as f32 / n + 0.1;
        OFFSET_COLORS[5 * i + 1] = -1.0 + 2.0 * ((i % N) as f32) / n + 0.1;

        // Colors
        let r = (i / N) as f32 / n;
        let g = ((i % N) as f32) / n;
        let b = r * g + 0.2;
        OFFSET_COLORS[5 * i + 2] = r;
        OFFSET_COLORS[5 * i + 3] = g;
        OFFSET_COLORS[5 * i + 4] = b;
    }
}
