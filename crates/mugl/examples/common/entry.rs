#[cfg(all(not(target_family = "wasm"), feature = "backend-wgpu"))]
pub fn init<A: super::app::App + 'static>() {
    use winit::{
        event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };

    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let instant = std::time::Instant::now();
    let device = pollster::block_on(A::request_wgpu_device(&window)).unwrap();
    let size = window.inner_size();
    let mut app = A::new(device, mugl::Extent2D(size.width, size.height));

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    app.resize(mugl::Extent2D(physical_size.width, physical_size.height));
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    app.resize(mugl::Extent2D(new_inner_size.width, new_inner_size.height));
                }
                _ => {}
            }
        }
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            app.render(instant.elapsed().as_secs_f64());
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}

#[macro_export]
macro_rules! main {
    ($app:ty) => {
        #[cfg(not(target_family = "wasm"))]
        fn main() {
            #[cfg(feature = "backend-wgpu")]
            crate::common::init::<$app>();
        }
    };
}
