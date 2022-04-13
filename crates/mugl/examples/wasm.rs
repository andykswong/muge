#![cfg(all(target_family = "wasm", feature = "backend-webgl"))]
#![no_main]

extern crate alloc;
mod app;
mod common;

use alloc::boxed::Box;
use common::{App, APP_ID};
use mugl::{prelude::*, webgl::*};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static mut APP: Option<Box<dyn App>> = None;
static mut CANVAS: Option<Canvas> = None;

#[no_mangle]
pub extern "C" fn app_id() -> ContextId {
    APP_ID
}

#[no_mangle]
pub extern "C" fn init(id: u32) {
    unsafe {
        if CANVAS.is_none() {
            CANVAS = Some(Canvas::from_id(APP_ID, "canvas"));
        }
        CANVAS.as_ref().map(|canvas| {
            APP = select_app(id, canvas);
        });
    }
}

fn select_app(id: u32, canvas: &Canvas) -> Option<Box<dyn App>> {
    match id {
        0 => create_app::<app::BasicExample>(canvas),
        1 => create_app::<app::InstancingExample>(canvas),
        2 => create_app::<app::StencilExample>(canvas),
        _ => None,
    }
}

fn create_app<A: App + 'static>(canvas: &Canvas) -> Option<Box<dyn App>> {
    let device = A::request_webgl_device(canvas)?;
    Some(Box::new(A::new(device, canvas.size())))
}

#[no_mangle]
pub extern "C" fn render(t: f64) -> bool {
    if let Some(app) = unsafe { APP.as_mut() } {
        app.render(t)
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn resize(width: u32, height: u32) {
    if let Some(app) = unsafe { APP.as_mut() } {
        app.resize(Extent2D(width, height));
    }
}

#[no_mangle]
pub extern "C" fn destroy() {
    unsafe {
        APP.take();
    }
}
