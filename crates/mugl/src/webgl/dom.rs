use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::future::Future;
use core::task::{Poll, Waker};

use super::interop::{CanvasId, ContextId, FutureId, FutureStatus, ImageSourceId};
use super::mugl;
use crate::primitive::Extent2D;

static mut TASKS: Vec<Rc<RefCell<Option<Waker>>>> = Vec::new();

/// JS/DOM task queue. Apps must call Tasks::poll()
pub struct Tasks;

impl Tasks {
    /// Polls any pending task.
    pub fn poll() {
        for waker in unsafe { TASKS.drain(..) } {
            if let Some(waker) = waker.borrow_mut().take() {
                waker.wake();
            }
        }
    }
}

/// A JS future object.
#[derive(Debug)]
pub(crate) struct JsFuture {
    pub(crate) id: FutureId,
    pub(crate) waker: Rc<RefCell<Option<Waker>>>,
}

impl JsFuture {
    pub fn new(id: FutureId) -> Self {
        Self {
            id,
            waker: Default::default(),
        }
    }
}

impl Future for JsFuture {
    type Output = Result<(), ()>;

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        match unsafe { mugl::get_future_status(self.id) } {
            FutureStatus::Pending => {
                let waker = &self.get_mut().waker;
                *waker.borrow_mut() = Some(cx.waker().clone());
                unsafe { TASKS.push(waker.clone()) };
                Poll::Pending
            }
            FutureStatus::Done => Poll::Ready(Ok(())),
            FutureStatus::Error => Poll::Ready(Err(())),
        }
    }
}

/// A canvas handle
#[derive(Debug)]
pub struct Canvas {
    pub(crate) id: CanvasId,
}

impl Canvas {
    /// Gets a canvas by ID.
    pub fn from_id(context: ContextId, id: &str) -> Self {
        Self {
            id: unsafe { mugl::get_canvas_by_id(context, id.into()) },
        }
    }

    /// Gets the size of the canvas.
    pub fn size(&self) -> Extent2D {
        unsafe {
            Extent2D(
                mugl::get_canvas_width(self.id),
                mugl::get_canvas_height(self.id),
            )
        }
    }
}

/// An external image handle
#[derive(Debug)]
pub struct ImageSource {
    pub(crate) id: ImageSourceId,
}

impl ImageSource {
    /// Loads an image from URI.
    pub fn from_uri(context: ContextId, uri: &str) -> Self {
        Self {
            id: unsafe { mugl::create_image(context, uri.into()) },
        }
    }

    /// Gets an image by ID.
    pub fn from_id(context: ContextId, id: &str) -> Self {
        Self {
            id: unsafe { mugl::get_image_by_id(context, id.into()) },
        }
    }

    /// Gets the size of the image.
    pub fn size(&self) -> Extent2D {
        unsafe {
            Extent2D(
                mugl::get_image_width(self.id),
                mugl::get_image_height(self.id),
            )
        }
    }
}

impl Drop for ImageSource {
    fn drop(&mut self) {
        unsafe { mugl::delete_image(self.id) }
    }
}
