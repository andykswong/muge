use bitflags::bitflags;

bitflags! {
    /// WebGL2 features.
    #[repr(transparent)]
    #[derive(Default)]
    pub struct WebGL2Features: u32 {
        const TEXTURE_ANISOTROPIC = 0x0001;
        const TEXTURE_HALF_FLOAT_LINEAR = 0x0002;
        const TEXTURE_FLOAT_LINEAR = 0x0004;
        const COLOR_BUFFER_FLOAT = 0x0008;
    }

    /// WebGL context attribute flags.
    #[repr(transparent)]
    pub struct WebGLContextAttribute: u32 {
        const ALPHA = 0x0001;
        const ANTIALIAS = 0x0002;
        const DEPTH = 0x0004;
        const DESYNCHRONIZED = 0x0008;
        const FAIL_IF_MAJOR_PERFORMANCE_CAVEAT = 0x0010;
        const HIGH_PEFORMANCE = 0x0020;
        const PREMULTIPLIED_ALPHA = 0x0040;
        const PRESERVE_DRAWING_BUFFER = 0x0080;
        const STENCIL = 0x0100;
    }
}

impl Default for WebGLContextAttribute {
    #[inline]
    fn default() -> Self {
        Self::ALPHA
            | Self::ANTIALIAS
            | Self::DEPTH
            | Self::HIGH_PEFORMANCE
            | Self::PREMULTIPLIED_ALPHA
    }
}
