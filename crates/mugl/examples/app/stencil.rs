use async_trait::async_trait;
use mugl::prelude::*;
use munum::{transform, vec3, Mat4, Vec3};

use crate::common::{App, UvVertex, CUBE_INDICES, CUBE_VERTICES};

const PI: f32 = std::f32::consts::PI;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Data {
    mvp: Mat4,
    outline: Color<f32>,
}

unsafe impl bytemuck::Pod for Data {}
unsafe impl bytemuck::Zeroable for Data {}

pub struct StencilExample {
    size: Extent2D,
    device: Device,
    pass: RenderPass,
    cube: RenderBundle,
    outline: RenderBundle,
    sky: RenderBundle,
    #[allow(dead_code)]
    image_loaded: bool,
    #[cfg(all(target_family = "wasm", feature = "backend-webgl"))]
    image: mugl::webgl::ImageSource,
}

#[async_trait(?Send)]
impl App for StencilExample {
    #[cfg(feature = "backend-wgpu")]
    async fn request_wgpu_device(window: &winit::window::Window) -> Option<mugl::wgpu::WGPUDevice> {
        use mugl::{
            wgpu::{WGPUSurfaceDescriptor, WGPU},
            Extent2D,
        };

        let size = window.inner_size();
        let size = Extent2D(size.width, size.height);

        WGPU::request_device(
            window,
            Default::default(),
            WGPUSurfaceDescriptor {
                depth_stencil_format: Some(TextureFormat::DEPTH24STENCIL8),
                sample_count: 1,
                size,
            },
        )
        .await
    }

    fn new(device: Device, size: Extent2D) -> Self {
        let pass = device.create_render_pass(RenderPassDescriptor::Default {
            clear_color: Some(Color(0., 0., 0., 1.0)),
            clear_depth: Some(1.),
            clear_stencil: Some(0),
        });

        cfg_if::cfg_if! {
            if #[cfg(all(target_family = "wasm", feature = "backend-webgl"))] {
                let vertex = include_str!("../shader/stencil.vs.glsl").into();
                let fragment = include_str!("../shader/stencil.cube.fs.glsl").into();
            } else {
                let vertex = include_str!("../shader/stencil.vs.wgsl").into();
                let fragment = include_str!("../shader/stencil.cube.fs.wgsl").into();
            }
        }

        let cube = RenderBundle::new(
            &device,
            vertex,
            fragment,
            true,
            CompareFunction::LessEqual,
            CullMode::Back,
            StencilFaceState {
                pass_op: StencilOperation::Replace,
                ..Default::default()
            },
            0xFF,
            1,
        );

        let sky = RenderBundle::new(
            &device,
            vertex,
            fragment,
            true,
            CompareFunction::LessEqual,
            CullMode::Front,
            Default::default(),
            0,
            1,
        );

        cfg_if::cfg_if! {
            if #[cfg(all(target_family = "wasm", feature = "backend-webgl"))] {
                let vertex = include_str!("../shader/stencil.vs.glsl").into();
                let fragment = include_str!("../shader/stencil.outline.fs.glsl").into();
            } else {
                let vertex = include_str!("../shader/stencil.vs.wgsl").into();
                let fragment = include_str!("../shader/stencil.outline.fs.wgsl").into();
            }
        }

        let outline = RenderBundle::new(
            &device,
            vertex,
            fragment,
            false,
            CompareFunction::Always,
            CullMode::Back,
            StencilFaceState {
                compare: CompareFunction::NotEqual,
                ..Default::default()
            },
            0,
            1,
        );

        #[cfg(not(target_family = "wasm"))]
        {
            let texture_image = image::load_from_memory(include_bytes!("../assets/airplane.png")).unwrap().into_rgba8();
            let texture_size = Extent3D(
                texture_image.dimensions().0,
                texture_image.dimensions().1,
                1,
            );
            cube.upload_texture_data(&device, &texture_image, texture_size);
            sky.upload_texture_data(&device, &texture_image, texture_size);
        }

        Self {
            size,
            device,
            pass,
            cube,
            outline,
            sky,
            image_loaded: false,
            #[cfg(all(target_family = "wasm", feature = "backend-webgl"))]
            image: mugl::webgl::ImageSource::from_uri("airplane.png"),
        }
    }

    fn device(&self) -> &Device {
        &self.device
    }

    fn render(&mut self, t: f64) -> bool {
        let t = t as f32;

        #[cfg(all(target_family = "wasm", feature = "backend-webgl"))]
        if !self.image_loaded && self.image.size().0 > 0 {
            self.cube.upload_image(&self.device, &self.image);
            self.sky.upload_image(&self.device, &self.image);
        }

        let aspect = self.size.0 as f32 / self.size.1 as f32;
        let proj = transform::perspective(aspect, PI / 4.0, 0.1, 100.0);
        let view = transform::look_at(
            vec3(10.0 * t.cos(), 5.0 * t.sin(), 10.0 * t.sin()),
            Vec3::default(),
            vec3(0., 1., 0.),
        );

        let mut mvp = proj * view;
        self.cube.update(mvp);
        mvp *= transform::scaling(vec3(1.1, 1.1, 1.1));
        self.outline.update(mvp);
        mvp *= transform::scaling(vec3(10.0, 10.0, 10.0));
        self.sky.update(mvp);

        {
            let encoder = self.device.render(&self.pass);
            self.cube.render(&self.device, &encoder);
            self.outline.render(&self.device, &encoder);
            self.sky.render(&self.device, &encoder);
            encoder.submit();
        }
        self.device.present();
        true
    }

    fn resize(&mut self, new_size: Extent2D) {
        self.device.resize_surface(new_size);
        self.size = new_size;
    }
}

struct RenderBundle {
    pipeline: RenderPipeline,
    stencil_ref: u32,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    data_bind_group: BindGroup,
    data_buffer: Buffer,
    data: Data,
    texture_bind_group: Option<BindGroup>,
    texture: Option<Texture>,
    #[allow(dead_code)]
    sampler: Option<Sampler>,
}

impl RenderBundle {
    fn new(
        device: &Device,
        vertex: &str,
        fragment: &str,
        has_texture: bool,
        depth_compare: CompareFunction,
        cull_mode: CullMode,
        stencil: StencilFaceState,
        stencil_write_mask: u32,
        stencil_ref: u32,
    ) -> Self {
        let data_layout = device.create_bind_group_layout(BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                label: "Data",
                binding: 0,
                visibility: ShaderStage::VERTEX | ShaderStage::FRAGMENT,
                ty: BindingType::Buffer {
                    dynamic_offset: false,
                },
            }],
        });

        let texture_layout = device.create_bind_group_layout(BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    label: "tex",
                    binding: 0,
                    visibility: ShaderStage::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        dimension: TextureDimension::D2,
                        sample_type: TextureSampleType::Float,
                    },
                },
                BindGroupLayoutEntry {
                    label: "tex",
                    binding: 1,
                    visibility: ShaderStage::FRAGMENT,
                    ty: BindingType::Sampler {
                        ty: SamplerBindingType::Filtering,
                    },
                },
            ],
        });

        let vertex = device.create_shader(ShaderDescriptor {
            usage: ShaderStage::VERTEX,
            code: vertex,
        });
        let fragment = device.create_shader(ShaderDescriptor {
            usage: ShaderStage::FRAGMENT,
            code: fragment,
        });

        let layout_with_tex = [&data_layout, &texture_layout];
        let layout_without_tex = [&data_layout];
        let pipeline = device.create_render_pipeline(RenderPipelineDescriptor {
            vertex: &vertex,
            fragment: &fragment,
            buffers: &[VertexBufferLayout {
                stride: std::mem::size_of::<UvVertex>() as BufferSize,
                step_mode: VertexStepMode::Vertex,
                attributes: &[
                    VertexAttribute {
                        offset: 0,
                        shader_location: 0,
                        format: VertexFormat::F32x3,
                    },
                    VertexAttribute {
                        offset: std::mem::size_of::<[f32; 3]>() as BufferSize,
                        shader_location: 1,
                        format: VertexFormat::F32x2,
                    },
                ],
            }],
            bind_groups: if has_texture {
                &layout_with_tex
            } else {
                &layout_without_tex
            },
            targets: Default::default(),
            primitive: PrimitiveState {
                index_format: Some(IndexFormat::UI16),
                cull_mode,
                ..Default::default()
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::DEPTH24STENCIL8,
                depth_write: true,
                depth_compare,
                stencil_back: stencil,
                stencil_front: stencil,
                stencil_write_mask,
                stencil_read_mask: 0xFF,
                ..Default::default()
            }),
            multisample: Default::default(),
        });

        let indices_data = bytemuck::cast_slice(CUBE_INDICES);
        let index_buffer = device.create_buffer(BufferDescriptor {
            usage: BufferUsage::INDEX,
            size: indices_data.len() as BufferSize,
        });
        device.write_buffer(&index_buffer, 0, indices_data);

        let vertices_data = bytemuck::cast_slice(CUBE_VERTICES);
        let vertex_buffer = device.create_buffer(BufferDescriptor {
            usage: BufferUsage::VERTEX,
            size: vertices_data.len() as BufferSize,
        });
        device.write_buffer(&vertex_buffer, 0, vertices_data);

        let data = [Data {
            mvp: Mat4::identity(),
            outline: Color(0.1, 0.3, 0.2, 1.0),
        }];
        let data_raw = bytemuck::cast_slice(&data);
        let data_buffer = device.create_buffer(BufferDescriptor {
            usage: BufferUsage::UNIFORM | BufferUsage::STREAM,
            size: data_raw.len() as BufferSize,
        });
        device.write_buffer(&data_buffer, 0, data_raw);

        let data_bind_group = device.create_bind_group(BindGroupDescriptor {
            layout: &data_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer {
                    buffer: &data_buffer,
                    offset: 0,
                    size: data_raw.len() as BufferSize,
                },
            }],
        });

        let texture = if has_texture {
            Some(device.create_texture(TextureDescriptor {
                format: TextureFormat::SRGBA8,
                size: Extent3D(512, 512, 1),
                ..Default::default()
            }))
        } else {
            None
        };

        let sampler = if has_texture {
            Some(device.create_sampler(SamplerDescriptor {
                address_mode_u: AddressMode::Repeat,
                address_mode_v: AddressMode::Repeat,
                mag_filter: FilterMode::Linear,
                min_filter: FilterMode::Linear,
                mipmap_filter: FilterMode::Linear,
                ..Default::default()
            }))
        } else {
            None
        };

        let texture_bind_group = if let Some(ref texture) = texture {
            if let Some(ref sampler) = sampler {
                Some(device.create_bind_group(BindGroupDescriptor {
                    layout: &texture_layout,
                    entries: &[
                        BindGroupEntry {
                            binding: 0,
                            resource: BindingResource::Texture(texture),
                        },
                        BindGroupEntry {
                            binding: 1,
                            resource: BindingResource::Sampler(sampler),
                        },
                    ],
                }))
            } else {
                None
            }
        } else {
            None
        };

        Self {
            pipeline,
            stencil_ref,
            vertex_buffer,
            index_buffer,
            data_bind_group,
            data_buffer,
            data: data[0],
            texture_bind_group,
            texture,
            sampler,
        }
    }

    #[cfg(all(target_family = "wasm", feature = "backend-webgl"))]
    fn upload_image(&self, device: &Device, image: &mugl::webgl::ImageSource) {
        if let Some(ref texture) = self.texture {
            device.copy_external_image_to_texture(
                ImageCopyExternalImage {
                    src: image,
                    origin: Origin2D(0, 0),
                },
                texture.into(),
                image.size(),
            )
        }
    }

    #[allow(dead_code)]
    fn upload_texture_data(&self, device: &Device, data: &[u8], texture_size: Extent3D) {
        if let Some(ref texture) = self.texture {
            device.write_texture(
                texture.into(),
                &data,
                ImageDataLayout {
                    offset: 0,
                    bytes_per_row: texture_size.0 * 4,
                    rows_per_image: texture_size.1,
                },
                texture_size,
            );
        }
    }

    #[inline]
    fn update(&mut self, mvp: Mat4) {
        self.data.mvp = mvp;
    }

    fn render<'a>(&'a self, device: &Device, pass: &RenderPassEncoder<'a>) {
        device.write_buffer(&self.data_buffer, 0, bytemuck::cast_slice(&[self.data]));
        pass.pipeline(&self.pipeline);
        pass.index(&self.index_buffer);
        pass.vertex(0, &self.vertex_buffer, 0);
        pass.bind_group(0, &self.data_bind_group, &[]);
        self.texture_bind_group.as_ref().map(|texture_bind_group| {
            pass.bind_group(1, texture_bind_group, &[]);
        });
        pass.stencil_ref(self.stencil_ref);
        pass.draw_indexed(0..(CUBE_INDICES.len() as u32), 0..1);
    }
}
