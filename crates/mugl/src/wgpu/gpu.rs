use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Range;

use std::sync::{RwLock, RwLockWriteGuard};

use async_trait::async_trait;
use raw_window_handle::HasRawWindowHandle;

use super::conv::wgpu_operations;
use super::resource::{
    WGPUBindGroup, WGPUBindGroupLayout, WGPUBuffer, WGPUBufferView, WGPUDeviceDescriptor,
    WGPUFeatures, WGPURenderPass, WGPURenderPipeline, WGPUSampler, WGPUShader,
    WGPUSurfaceDescriptor, WGPUTexture,
};
use crate::descriptor::{
    BindGroupDescriptor, BindGroupLayoutDescriptor, BufferDescriptor, ColorTargetStates,
    ImageCopyTexture, ImageDataLayout, RenderPassDescriptor, RenderPipelineDescriptor,
    SamplerDescriptor, ShaderDescriptor, TextureDescriptor,
};
use crate::gpu::{GPUDevice, GPURefTypes, GPURenderPassEncoder, GPU};
use crate::primitive::{BufferSize, Color, Extent2D, Extent3D, TextureUsage};

/// WebGPU interface
#[derive(Debug)]
pub struct WGPU;

#[derive(Debug)]
pub struct WGPUDevice {
    #[allow(dead_code)]
    instance: wgpu::Instance,
    #[allow(dead_code)]
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,

    surface_config: RwLock<wgpu::SurfaceConfiguration>,
    surface_texture: RwLock<WGPUSurfaceTexture>,
    surface_depth_format: Option<wgpu::TextureFormat>,
    surface_msaa_sample_count: u32,

    commands: RwLock<Vec<wgpu::CommandBuffer>>,
    encoder: RwLock<Option<wgpu::CommandEncoder>>,
}

#[derive(Debug, Default)]
struct WGPUSurfaceTexture {
    texture: Option<wgpu::SurfaceTexture>,
    texture_view: Option<wgpu::TextureView>,
    msaa_texture: Option<wgpu::Texture>,
    msaa_texture_view: Option<wgpu::TextureView>,
    depth_texture: Option<wgpu::Texture>,
    depth_texture_view: Option<wgpu::TextureView>,
}

#[derive(Debug)]
pub struct WGPURenderPassEncoder<'a> {
    device: &'a WGPUDevice,
    // The pass must be declare before encoder so that it will be dropped first.
    pass: RwLock<Option<wgpu::RenderPass<'a>>>,
    // Must Box the encoder to provide a stable address for RenderPass to reference to.
    encoder: RwLock<Box<wgpu::CommandEncoder>>,
    index_format: RwLock<wgpu::IndexFormat>,
}

impl WGPU {
    /// Requests a new WGPU device asynchronously
    pub async fn request_device<W: HasRawWindowHandle>(
        window: &W,
        descriptor: WGPUDeviceDescriptor,
        surface_descriptor: WGPUSurfaceDescriptor,
    ) -> Option<WGPUDevice> {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: descriptor.power_preference.into(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: descriptor.force_fallback_adapter,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None, // Trace path
            )
            .await
            .ok()?;

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter)?,
            width: surface_descriptor.size.0,
            height: surface_descriptor.size.1,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);

        let surface_depth_format = surface_descriptor.depth_stencil_format.map(Into::into);
        let surface_msaa_sample_count = surface_descriptor.sample_count;
        let mut surface_texture = WGPUSurfaceTexture::default();
        update_surface_depth_msaa(
            &mut surface_texture,
            &device,
            &surface_config,
            surface_depth_format,
            surface_msaa_sample_count,
        );

        Some(WGPUDevice {
            instance,
            adapter,
            device,
            queue,
            surface,
            surface_config: RwLock::new(surface_config),
            surface_texture: RwLock::new(surface_texture),
            surface_depth_format,
            surface_msaa_sample_count,
            commands: RwLock::default(),
            encoder: RwLock::default(),
        })
    }
}

impl GPU for WGPU {
    type Features = WGPUFeatures;
    type Device = WGPUDevice;
    type Buffer = WGPUBuffer;
    type Texture = WGPUTexture;
    type Sampler = WGPUSampler;
    type Shader = WGPUShader;
    type RenderPass = WGPURenderPass;
    type RenderPipeline = WGPURenderPipeline;
    type BindGroup = WGPUBindGroup;
    type BindGroupLayout = WGPUBindGroupLayout;
}

impl<'a> GPURefTypes<'a, WGPU> for WGPU {
    type RenderPassEncoder = WGPURenderPassEncoder<'a>;
    type BufferView = WGPUBufferView<'a>;
}

impl WGPUDevice {
    /// Submits a command buffer.
    fn submit(&self, buffer: wgpu::CommandBuffer) {
        if let Ok(mut commands) = self.commands.write() {
            // Push any pending commands in device encoder first

            if let Some(encoder) = self.encoder.write().unwrap().take() {
                commands.push(encoder.finish());
            }

            commands.push(buffer);
        }
    }

    fn get_encoder(&self) -> RwLockWriteGuard<Option<wgpu::CommandEncoder>> {
        let mut encoder = self.encoder.write().unwrap();
        if encoder.is_none() {
            *encoder = Some(
                self.device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None }),
            );
        }
        encoder
    }
}

#[async_trait(?Send)]
impl GPUDevice<WGPU> for WGPUDevice {
    fn features(&self) -> WGPUFeatures {
        WGPUFeatures::empty()
    }

    fn create_buffer(&self, descriptor: BufferDescriptor) -> WGPUBuffer {
        WGPUBuffer {
            buffer: self.device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: descriptor.size as u64,
                usage: descriptor.usage.into(),
                mapped_at_creation: false,
            }),
        }
    }

    fn create_texture(&self, descriptor: TextureDescriptor) -> WGPUTexture {
        let msaa_resolve = !descriptor.format.is_depth_stencil()  // depth-stencil cannot be MSAA resolved
            && descriptor.sample_count > 1
            && descriptor.usage.contains(TextureUsage::RENDER_ATTACHMENT);

        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: descriptor.size.into(),
            mip_level_count: descriptor.mip_level_count,
            sample_count: if msaa_resolve {
                1 // sample count of resolve target must be 1
            } else {
                descriptor.sample_count
            },
            dimension: descriptor.dimension.into(),
            format: descriptor.format.into(),
            usage: descriptor.usage.into(),
        });

        WGPUTexture {
            view: texture.create_view(&wgpu::TextureViewDescriptor::default()),
            texture,
            msaa_texture: if msaa_resolve {
                Some(self.device.create_texture(&wgpu::TextureDescriptor {
                    label: None,
                    size: wgpu::Extent3d {
                        width: descriptor.size.0,
                        height: descriptor.size.1,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: descriptor.sample_count,
                    dimension: wgpu::TextureDimension::D2,
                    format: descriptor.format.into(),
                    usage: descriptor.usage.into(),
                }))
            } else {
                None
            },
            format: descriptor.format,
            dimension: descriptor.dimension,
        }
    }

    fn create_sampler(&self, descriptor: SamplerDescriptor) -> WGPUSampler {
        WGPUSampler {
            sampler: self.device.create_sampler(&wgpu::SamplerDescriptor {
                label: None,
                address_mode_u: descriptor.address_mode_u.into(),
                address_mode_v: descriptor.address_mode_v.into(),
                address_mode_w: descriptor.address_mode_w.into(),
                mag_filter: descriptor.mag_filter.into(),
                min_filter: descriptor.min_filter.into(),
                mipmap_filter: descriptor.mipmap_filter.into(),
                lod_min_clamp: descriptor.lod_min_clamp,
                lod_max_clamp: descriptor.lod_max_clamp,
                compare: descriptor.compare.map(Into::into),
                anisotropy_clamp: core::num::NonZeroU8::new(descriptor.max_anisotropy),
                border_color: None,
            }),
        }
    }

    fn create_shader(&self, descriptor: ShaderDescriptor) -> WGPUShader {
        WGPUShader {
            shader: self
                .device
                .create_shader_module(&wgpu::ShaderModuleDescriptor {
                    label: None,
                    source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(descriptor.code)),
                }),
        }
    }

    fn create_render_pipeline(
        &self,
        descriptor: RenderPipelineDescriptor<WGPU>,
    ) -> WGPURenderPipeline {
        let attributes = {
            let mut attributes = Vec::<wgpu::VertexAttribute>::new();
            for layout in descriptor.buffers {
                for attr in layout.attributes {
                    attributes.push(wgpu::VertexAttribute {
                        format: attr.format.into(),
                        offset: attr.offset as u64,
                        shader_location: attr.shader_location,
                    });
                }
            }
            attributes
        };
        let buffers = {
            let mut buffers = Vec::<wgpu::VertexBufferLayout>::new();
            let mut i = 0;
            for layout in descriptor.buffers {
                buffers.push(wgpu::VertexBufferLayout {
                    array_stride: layout.stride as u64,
                    step_mode: layout.step_mode.into(),
                    attributes: &attributes[i..(i + layout.attributes.len())],
                });
                i += layout.attributes.len();
            }
            buffers
        };

        WGPURenderPipeline {
            pipeline: self
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: None,
                    layout: Some(
                        &self
                            .device
                            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                                label: None,
                                bind_group_layouts: &descriptor
                                    .bind_groups
                                    .iter()
                                    .map(|layout| &layout.layout)
                                    .collect::<Vec<_>>(),
                                push_constant_ranges: &[],
                            }),
                    ),
                    vertex: wgpu::VertexState {
                        module: &descriptor.vertex.shader,
                        entry_point: "vs_main", // TODO: should this be customizable?
                        buffers: &buffers,
                    },
                    primitive: descriptor.primitive.into(),
                    depth_stencil: descriptor.depth_stencil.map(Into::into),
                    multisample: descriptor.multisample.into(),
                    fragment: Some(wgpu::FragmentState {
                        module: &descriptor.fragment.shader,
                        entry_point: "fs_main", // TODO: should this be customizable?
                        targets: &(match descriptor.targets {
                            ColorTargetStates::Default { blend, write_mask } => {
                                vec![wgpu::ColorTargetState {
                                    format: self
                                        .surface
                                        .get_preferred_format(&self.adapter)
                                        .unwrap(),
                                    blend: blend.map(Into::into),
                                    write_mask: write_mask.into(),
                                }]
                            }
                            ColorTargetStates::Offscreen { targets } => targets
                                .iter()
                                .map(|target| wgpu::ColorTargetState {
                                    format: target.format.into(),
                                    blend: target.blend.map(Into::into),
                                    write_mask: target.write_mask.into(),
                                })
                                .collect::<Vec<_>>(),
                        }),
                    }),
                    multiview: None, // TODO: multiview
                }),
            index_format: descriptor
                .primitive
                .index_format
                .map(Into::into)
                .unwrap_or(wgpu::IndexFormat::Uint16),
        }
    }

    fn create_render_pass(&self, descriptor: RenderPassDescriptor<WGPU>) -> WGPURenderPass {
        match descriptor {
            RenderPassDescriptor::Default {
                clear_color,
                clear_depth,
                clear_stencil,
            } => {
                WGPURenderPass {
                    // Leave texture view as empty for default pass. They must be recreated every frame
                    color_views: Vec::default(),
                    resolve_targets: Vec::default(),
                    depth_view: None,
                    color_ops: vec![wgpu_operations(clear_color.map(Into::into))],
                    depth_ops: Some(wgpu_operations(clear_depth)),
                    stencil_ops: Some(wgpu_operations(clear_stencil)),
                }
            }
            RenderPassDescriptor::Offscreen {
                colors,
                depth_stencil,
                clear_depth,
                clear_stencil,
            } => WGPURenderPass {
                color_views: colors
                    .iter()
                    .map(|color| {
                        color
                            .view
                            .texture
                            .msaa_texture
                            .as_ref()
                            .map(|texture| {
                                texture.create_view(&wgpu::TextureViewDescriptor::default())
                            })
                            .unwrap_or_else(|| color.view.into())
                    })
                    .collect(),
                resolve_targets: colors
                    .iter()
                    .map(|color| {
                        if color.view.texture.msaa_texture.is_some() {
                            Some(color.view.into())
                        } else {
                            None
                        }
                    })
                    .collect(),
                depth_view: depth_stencil.map(Into::into),
                color_ops: colors
                    .iter()
                    .map(|color| wgpu_operations(color.clear.map(Into::into)))
                    .collect(),
                depth_ops: depth_stencil.map(|_| wgpu_operations(clear_depth)),
                stencil_ops: depth_stencil.map(|_| wgpu_operations(clear_stencil)),
            },
        }
    }

    fn create_bind_group_layout(
        &self,
        descriptor: BindGroupLayoutDescriptor,
    ) -> WGPUBindGroupLayout {
        WGPUBindGroupLayout {
            layout: self
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &descriptor
                        .entries
                        .iter()
                        .map(|entry| wgpu::BindGroupLayoutEntry {
                            binding: entry.binding,
                            visibility: entry.visibility.into(),
                            ty: entry.ty.into(),
                            count: None,
                        })
                        .collect::<Vec<_>>(),
                }),
        }
    }

    fn create_bind_group(&self, descriptor: BindGroupDescriptor<WGPU>) -> WGPUBindGroup {
        WGPUBindGroup {
            bind_group: self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &descriptor.layout.layout,
                entries: &descriptor
                    .entries
                    .iter()
                    .map(|entry| wgpu::BindGroupEntry {
                        binding: entry.binding,
                        resource: entry.resource.into(),
                    })
                    .collect::<Vec<_>>(),
            }),
        }
    }

    fn render<'a>(&'a self, pass: &'a WGPURenderPass) -> WGPURenderPassEncoder<'a> {
        let is_default_pass = pass.color_views.is_empty();

        if is_default_pass {
            update_surface_texture(self);
        }

        let surface_texture = self.surface_texture.read().unwrap();
        let encoder = RwLock::new(Box::new(
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None }),
        ));

        WGPURenderPassEncoder {
            device: &self,
            pass: RwLock::new(if is_default_pass && surface_texture.texture.is_none() {
                None // TODO: Not able to get a valid surface texture
            } else {
                let mut encoder = encoder.write().unwrap();
                let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &(if is_default_pass {
                        vec![wgpu::RenderPassColorAttachment {
                            view: surface_texture
                                .msaa_texture_view
                                .as_ref()
                                .unwrap_or(surface_texture.texture_view.as_ref().unwrap()),
                            resolve_target: if surface_texture.msaa_texture_view.is_some() {
                                surface_texture.texture_view.as_ref()
                            } else {
                                None
                            },
                            ops: pass.color_ops[0],
                        }]
                    } else {
                        pass.color_views
                            .iter()
                            .enumerate()
                            .map(|(i, ref view)| wgpu::RenderPassColorAttachment {
                                view,
                                resolve_target: pass.resolve_targets[i].as_ref(),
                                ops: pass.color_ops[i],
                            })
                            .collect::<Vec<_>>()
                    }),
                    depth_stencil_attachment: (if is_default_pass {
                        surface_texture.depth_texture_view.as_ref()
                    } else {
                        pass.depth_view.as_ref()
                    })
                    .map(|view| wgpu::RenderPassDepthStencilAttachment {
                        view,
                        depth_ops: pass.depth_ops,
                        stencil_ops: pass.stencil_ops,
                    }),
                });

                Some(unsafe {
                    // Bypassing brrow checker.
                    // We have to make sure that RenderPass is dropped before the encoder
                    core::mem::transmute::<wgpu::RenderPass<'_>, wgpu::RenderPass<'a>>(render_pass)
                })
            }),
            encoder,
            index_format: RwLock::new(wgpu::IndexFormat::Uint16),
        }
    }

    fn write_buffer(&self, buffer: &WGPUBuffer, buffer_offset: BufferSize, data: &[u8]) {
        self.queue
            .write_buffer(&buffer.buffer, buffer_offset as u64, data);
    }

    fn write_texture(
        &self,
        texture: ImageCopyTexture<WGPU>,
        data: &[u8],
        layout: ImageDataLayout,
        size: Extent3D,
    ) {
        self.queue
            .write_texture(texture.into(), data, layout.into(), size.into())
    }

    async fn read_buffer<'a>(
        &self,
        buffer: &'a WGPUBuffer,
        range: Range<BufferSize>,
    ) -> Result<WGPUBufferView<'a>, ()> {
        let slice = buffer
            .buffer
            .slice((range.start as u64)..(range.end as u64));
        Ok(WGPUBufferView {
            buffer: &buffer.buffer,
            view: if let Ok(_) = slice.map_async(wgpu::MapMode::Read).await {
                Some(slice.get_mapped_range())
            } else {
                None
            },
        })
    }

    fn copy_buffer(
        &self,
        src: &WGPUBuffer,
        src_offset: BufferSize,
        dst: &WGPUBuffer,
        dst_offset: BufferSize,
        size: BufferSize,
    ) {
        self.get_encoder().as_mut().map(|encoder| {
            encoder.copy_buffer_to_buffer(
                &src.buffer,
                src_offset as u64,
                &dst.buffer,
                dst_offset as u64,
                size as u64,
            );
        });
    }

    fn copy_texture(
        &self,
        src: ImageCopyTexture<WGPU>,
        dst: ImageCopyTexture<WGPU>,
        size: Extent3D,
    ) {
        self.get_encoder()
            .as_mut()
            .map(|encoder| encoder.copy_texture_to_texture(src.into(), dst.into(), size.into()));
    }

    fn copy_texture_to_buffer(
        &self,
        src: ImageCopyTexture<WGPU>,
        dst: &WGPUBuffer,
        layout: ImageDataLayout,
        size: Extent3D,
    ) {
        self.get_encoder().as_mut().map(|encoder| {
            encoder.copy_texture_to_buffer(
                src.into(),
                wgpu::ImageCopyBuffer {
                    buffer: &dst.buffer,
                    layout: layout.into(),
                },
                size.into(),
            )
        });
    }

    fn is_lost(&self) -> bool {
        // TODO
        std::dbg!("is_lost is currently unsupported by WGPU backend");
        false
    }

    fn flush(&self) {
        if let Ok(mut commands) = self.commands.write() {
            if commands.len() > 0 {
                self.queue.submit(core::mem::take(&mut *commands));
            }
        }
    }

    fn present(&self) {
        self.flush();

        if let Some(texture) = self.surface_texture.write().unwrap().texture.take() {
            texture.present();
        }
    }

    fn resize_surface(&self, size: Extent2D) {
        let surface_config = {
            let mut surface_config = self.surface_config.write().unwrap();
            if size.0 > 0 && size.1 > 0 {
                surface_config.width = size.0;
                surface_config.height = size.1;
            }
            surface_config.clone()
        };
        self.surface.configure(&self.device, &surface_config);

        update_surface_depth_msaa(
            &mut *self.surface_texture.write().unwrap(),
            &self.device,
            &surface_config,
            self.surface_depth_format,
            self.surface_msaa_sample_count,
        );
    }
}

impl<'a> GPURenderPassEncoder<'a, WGPU> for WGPURenderPassEncoder<'a> {
    fn pipeline(&self, pipeline: &'a WGPURenderPipeline) {
        if let Ok(mut lock) = self.pass.write() {
            if let Some(pass) = lock.as_mut() {
                pass.set_pipeline(&pipeline.pipeline);
                *self.index_format.write().unwrap() = pipeline.index_format;
            }
        }
    }

    fn index(&self, buffer: &'a WGPUBuffer) {
        if let Ok(mut lock) = self.pass.write() {
            if let Some(pass) = lock.as_mut() {
                pass.set_index_buffer(buffer.buffer.slice(..), *self.index_format.read().unwrap());
            }
        }
    }

    fn vertex(&self, slot: u32, buffer: &'a WGPUBuffer, offset: BufferSize) {
        if let Ok(mut lock) = self.pass.write() {
            if let Some(pass) = lock.as_mut() {
                pass.set_vertex_buffer(slot, buffer.buffer.slice((offset as u64)..));
            }
        }
    }

    fn bind_group(&self, slot: u32, bind_group: &'a WGPUBindGroup, offsets: &[u32]) {
        if let Ok(mut lock) = self.pass.write() {
            if let Some(pass) = lock.as_mut() {
                pass.set_bind_group(
                    slot,
                    &bind_group.bind_group,
                    &offsets.iter().map(|offset| *offset).collect::<Vec<_>>(),
                );
            }
        }
    }

    fn draw(&self, vertices: Range<u32>, instances: Range<u32>) {
        if let Ok(mut lock) = self.pass.write() {
            if let Some(pass) = lock.as_mut() {
                pass.draw(vertices, instances);
            }
        }
    }

    fn draw_indexed(&self, indices: Range<u32>, instances: Range<u32>) {
        if let Ok(mut lock) = self.pass.write() {
            if let Some(pass) = lock.as_mut() {
                pass.draw_indexed(indices, 0, instances);
            }
        }
    }

    fn viewport(&self, x: f32, y: f32, width: f32, height: f32, min_depth: f32, max_depth: f32) {
        if let Ok(mut lock) = self.pass.write() {
            if let Some(pass) = lock.as_mut() {
                pass.set_viewport(x, y, width, height, min_depth, max_depth);
            }
        }
    }

    fn scissor_rect(&self, x: u32, y: u32, width: u32, height: u32) {
        if let Ok(mut lock) = self.pass.write() {
            if let Some(pass) = lock.as_mut() {
                pass.set_scissor_rect(x, y, width, height);
            }
        }
    }

    fn blend_const(&self, color: Color) {
        if let Ok(mut lock) = self.pass.write() {
            if let Some(pass) = lock.as_mut() {
                pass.set_blend_constant(color.into());
            }
        }
    }

    fn stencil_ref(&self, reference: u32) {
        if let Ok(mut lock) = self.pass.write() {
            if let Some(pass) = lock.as_mut() {
                pass.set_stencil_reference(reference);
            }
        }
    }

    fn submit(self) {
        {
            // Drops the render pass before consuming encoder
            let _pass = self.pass.into_inner();
        }
        self.device
            .submit(self.encoder.into_inner().unwrap().finish());
    }
}

fn update_surface_texture(device: &WGPUDevice) {
    match device.surface.get_current_texture() {
        Ok(surface_texture) => {
            let view = surface_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let mut lock = device.surface_texture.write().unwrap();
            lock.texture = Some(surface_texture);
            lock.texture_view = Some(view);
        }
        Err(wgpu::SurfaceError::Lost) => {
            // Reconfigure surface if lost
            device.resize_surface(Extent2D(0, 0));
        }
        // Outdated, Timeout and hopefully OutOfMemory should be resolved by the next frame
        _ => (),
    }
}

fn update_surface_depth_msaa(
    surface_texture: &mut WGPUSurfaceTexture,
    device: &wgpu::Device,
    surface_config: &wgpu::SurfaceConfiguration,
    depth_format: Option<wgpu::TextureFormat>,
    sample_count: u32,
) {
    if sample_count > 1 {
        let msaa_tex = create_surface_texture(
            &device,
            &surface_config,
            surface_config.format,
            sample_count,
        );
        surface_texture.msaa_texture_view =
            Some(msaa_tex.create_view(&wgpu::TextureViewDescriptor::default()));
        surface_texture.msaa_texture = Some(msaa_tex);
    }
    if let Some(format) = depth_format {
        let depth_tex = create_surface_texture(&device, &surface_config, format, sample_count);
        surface_texture.depth_texture_view =
            Some(depth_tex.create_view(&wgpu::TextureViewDescriptor::default()));
        surface_texture.depth_texture = Some(depth_tex);
    }
}

fn create_surface_texture(
    device: &wgpu::Device,
    surface_config: &wgpu::SurfaceConfiguration,
    format: wgpu::TextureFormat,
    sample_count: u32,
) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: wgpu::Extent3d {
            width: surface_config.width,
            height: surface_config.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        dimension: wgpu::TextureDimension::D2,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        sample_count,
        format,
    })
}
