#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColorVertex {
    position: [f32; 3],
    color: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UvVertex {
    position: [f32; 3],
    uv: [f32; 2],
}

pub const TRIANGLE: &[ColorVertex] = &[
    ColorVertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0, 1.0],
    },
    ColorVertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0, 1.0],
    },
    ColorVertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0, 1.0],
    },
];

pub const CUBE_VERTICES: &[UvVertex] = &[
    UvVertex { position: [-1.0,  1.0,  1.0], uv: [0.0, 0.0] }, UvVertex { position: [ 1.0,  1.0,  1.0], uv: [1.0, 0.0] }, UvVertex { position: [ 1.0, -1.0,  1.0], uv: [1.0, 1.0] }, UvVertex { position: [-1.0, -1.0,  1.0], uv: [0.0, 1.0] }, // positive z face
    UvVertex { position: [ 1.0,  1.0, -1.0], uv: [0.0, 0.0] }, UvVertex { position: [-1.0,  1.0, -1.0], uv: [1.0, 0.0] }, UvVertex { position: [-1.0, -1.0, -1.0], uv: [1.0, 1.0] }, UvVertex { position: [ 1.0, -1.0, -1.0], uv: [0.0, 1.0] }, // negative z face
    UvVertex { position: [ 1.0,  1.0,  1.0], uv: [0.0, 0.0] }, UvVertex { position: [ 1.0,  1.0, -1.0], uv: [1.0, 0.0] }, UvVertex { position: [ 1.0, -1.0, -1.0], uv: [1.0, 1.0] }, UvVertex { position: [ 1.0, -1.0,  1.0], uv: [0.0, 1.0] }, // positive x face
    UvVertex { position: [-1.0,  1.0, -1.0], uv: [0.0, 0.0] }, UvVertex { position: [-1.0,  1.0,  1.0], uv: [1.0, 0.0] }, UvVertex { position: [-1.0, -1.0,  1.0], uv: [1.0, 1.0] }, UvVertex { position: [-1.0, -1.0, -1.0], uv: [0.0, 1.0] }, // negative x face
    UvVertex { position: [-1.0,  1.0, -1.0], uv: [0.0, 0.0] }, UvVertex { position: [ 1.0,  1.0, -1.0], uv: [1.0, 0.0] }, UvVertex { position: [ 1.0,  1.0,  1.0], uv: [1.0, 1.0] }, UvVertex { position: [-1.0,  1.0,  1.0], uv: [0.0, 1.0] }, // positive y face
    UvVertex { position: [-1.0, -1.0, -1.0], uv: [0.0, 0.0] }, UvVertex { position: [ 1.0, -1.0, -1.0], uv: [1.0, 0.0] }, UvVertex { position: [ 1.0, -1.0,  1.0], uv: [1.0, 1.0] }, UvVertex { position: [-1.0, -1.0,  1.0], uv: [0.0, 1.0] }  // negative y face
];

pub const CUBE_INDICES: &[u16] = &[
    2, 1, 0, 2, 0, 3, // positive z face
    6, 5, 4, 6, 4, 7, // negative z face
    10, 9, 8, 10, 8, 11, // positive x face
    14, 13, 12, 14, 12, 15, // negative x face
    18, 17, 16, 18, 16, 19, // positive y face
    20, 21, 22, 23, 20, 22, // negative y face
];
