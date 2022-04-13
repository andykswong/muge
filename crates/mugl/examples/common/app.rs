use async_trait::async_trait;

#[async_trait(?Send)]
pub trait App {
    #[cfg(feature = "backend-wgpu")]
    async fn request_wgpu_device(window: &winit::window::Window) -> Option<mugl::wgpu::WGPUDevice>
    where
        Self: Sized,
    {
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
                depth_stencil_format: None,
                sample_count: 1,
                size,
            },
        )
        .await
    }

    #[cfg(feature = "backend-webgl")]
    fn request_webgl_device(canvas: &mugl::webgl::Canvas) -> Option<mugl::webgl::WebGLDevice>
    where
        Self: Sized,
    {
        use mugl::webgl::{WebGL, WebGL2Features, WebGLContextAttribute};
        WebGL::request_device(
            &canvas,
            WebGLContextAttribute::default() | WebGLContextAttribute::STENCIL,
            WebGL2Features::all(),
        )
    }

    fn new(device: mugl::Device, size: mugl::Extent2D) -> Self
    where
        Self: Sized;

    fn device(&self) -> &mugl::Device;

    fn render(&mut self, _t: f64) -> bool {
        false
    }

    fn resize(&mut self, new_size: mugl::Extent2D) {
        use mugl::GPUDevice;
        self.device().resize_surface(new_size);
    }
}
