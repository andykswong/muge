<h1 align="center">█▓▒­░⡷⠂μ ＧＬ⠐⢾░▒▓█</h1>
<h2 align="center">Micro WebGL2 / WebGPU Graphics Library for Rust</h2>
<br />
<p align="center">
  <a href="./LICENSE"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT" /></a> 
  <a href="https://crates.io/crates/mugl"><img src="https://img.shields.io/crates/v/mugl.svg" alt="Crates.io" /></a> 
  <a href="https://docs.rs/mugl"><img src="https://docs.rs/mugl/badge.svg" alt="Docs.rs" /></a> 
</p>

## Overview
`mugl` is a minimal, modern WebGL 2.0 / WebGPU 3D graphics abstraction layer. It provides a simplified WebGPU-style API that runs on the web using WebGL 2.0, and other platforms using native WebGPU.

## Install
```toml
[dependencies]
mugl = "0.1"
```
Features:
- `backend-webgl` - enables WebGL 2.0 backend for WASM. Requires [`mugl/wasm`](https://github.com/andykswong/mugl) npm package for glue code. (see [usage](#hello-world))
- `backend-wgpu` - enables WebGPU backend based on `wgpu`
- `std` - enables `std` support.
- `serde` - enables `serde` serialize/deserialize implementations

## [Documentation](https://docs.rs/mugl)
See Docs.rs: https://docs.rs/mugl

## Usage

### [Examples](./examples)
Several examples can be found in this repository:
- [basic](./examples/app/basic.rs): ```cargo run --features backend-wgpu --example basic```
- [instancing](./examples/app/instancing.rs): ```cargo run --features backend-wgpu --example instancing```
- [stencil](./examples/app/stencil.rs): ```cargo run --features backend-wgpu --example stencil```

Use npm to run the above examples on web: ```npm install && npm start```

### Hello World
Below is the minimal WASM app to draw a triangle using the WebGL backend (See full example code [here](./examples/app/basic.rs)):
```rust
use mugl::{prelude::*, webgl::*};

// 0. Define a unique app ID to use in JS glue code
#[no_mangle]
pub extern "C" fn app_id() -> ContextId { ContextId::new(123.) }

#[no_mangle]
pub extern "C" fn render() {
    // 1. Create device from canvas of id "canvas"
    let canvas = Canvas::from_id(app_id(), "canvas");
    let device = WebGL::request_device(&canvas, WebGLContextAttribute::default(), WebGL2Features::empty())
        .expect("WebGL 2.0 is unsupported");

    // 2. Create buffer
    let vertices: &[f32] = &[
        // position      color 
        0.0, 0.5, 0.0,   1.0, 0.0, 0.0, 1.0,
        0.5, -0.5, 0.0,  0.0, 1.0, 0.0, 1.0,
        -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 1.0
    ];
    let vertices: &[u8] = bytemuck::cast_slice(vertices);
    let buffer = device.create_buffer(BufferDescriptor { usage: BufferUsage::VERTEX, size: 3 });
    device.write_buffer(&buffer, 0, vertices);

    // 3. Create shaders
    let vertex = &device.create_shader(ShaderDescriptor {
        usage: ShaderStage::VERTEX,
        code: "#version 300 es
        layout (location=0) in vec3 position;
        layout (location=1) in vec4 color;
        out vec4 vColor;
        void main () {
          gl_Position = vec4(position, 1);
          vColor = color;
        }
        ".into(),
    });
    let fragment = &device.create_shader(ShaderDescriptor {
        usage: ShaderStage::FRAGMENT,
        code: "#version 300 es
        precision mediump float;
        in vec4 vColor;
        out vec4 outColor;
        void main () {
          outColor = vColor;
        }
        ".into(),
    });

    // 4. Create pipeline
    let pipeline = device.create_render_pipeline(RenderPipelineDescriptor {
        vertex,
        fragment,
        buffers: &[VertexBufferLayout {
            stride: core::mem::size_of::<[f32; 7]>() as BufferSize,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute { shader_location: 0, format: VertexFormat::F32x3, offset: 0 },
                VertexAttribute { shader_location: 1, format: VertexFormat::F32x4, offset: core::mem::size_of::<[f32; 3]>() as BufferSize },
            ],
        }],
        bind_groups: &[],
        targets: Default::default(),
        primitive: Default::default(),
        depth_stencil: Default::default(),
        multisample: Default::default(),
    });

    // 5. Create default pass
    let pass = device.create_render_pass(RenderPassDescriptor::Default {
        clear_color: Some(Color(0.1, 0.2, 0.3, 1.0)),
        clear_depth: None,
        clear_stencil: None,
    });

    // 6. Render
    {
        let encoder = device.render(&pass);
        encoder.pipeline(&pipeline);
        encoder.vertex(0, &buffer, 0);
        encoder.draw(0..3, 0..1);
        encoder.submit();
    }
    device.present();
}

```

Below is the JS glue code to initialize the WASM app:

```javascript
import { set_context_memory } from "mugl/wasm";
import { memory, app_id, render } from "hello_world.wasm";

// 0. Init
set_context_memory(app_id(), memory);

// 1. Create canvas with id "canvas"
const canvas = document.createElement("canvas");
canvas.id = "canvas";
canvas.width = canvas.height = 512;
document.body.appendChild(canvas);

// 2. Call render
render();
```
