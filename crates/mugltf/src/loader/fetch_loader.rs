#![cfg(feature = "fetch-loader")]

use super::GltfResourceLoader;
use alloc::{boxed::Box, string::String, vec::Vec};
use async_trait::async_trait;
use core::fmt;
use js_sys::{Promise, Uint8Array};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlImageElement, Request, RequestInit, RequestMode, Response};

/// Loader of glTF resources using [fetch](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API) API.
#[derive(Debug)]
pub struct GltfResourceFetchLoader {
    path: String,
}

impl Default for GltfResourceFetchLoader {
    fn default() -> Self {
        Self { path: "./".into() }
    }
}

#[async_trait(?Send)]
impl GltfResourceLoader for GltfResourceFetchLoader {
    type Error = FetchLoaderError;
    type ImageData = HtmlImageElement;

    #[inline]
    fn set_path(&mut self, path: &str) {
        self.path = if path.is_empty() {
            "./".into()
        } else if path.chars().last().filter(|end| '/' == *end).is_some() {
            path.into()
        } else {
            [path, "/"].concat()
        };
    }

    async fn get_gltf(&self, uri: &str) -> Result<Vec<u8>, Self::Error> {
        fetch(&[&self.path, uri].concat())
            .await
            .map_err(FetchLoaderError)
    }

    async fn get_buffer(&self, uri: &str) -> Result<Vec<u8>, Self::Error> {
        fetch(&resolve_url(&self.path, uri))
            .await
            .map_err(FetchLoaderError)
    }

    async fn get_image(&self, uri: &str) -> Result<Self::ImageData, Self::Error> {
        load_image(uri).await.map_err(FetchLoaderError)
    }

    async fn decode_image(
        &self,
        img: &[u8],
        mime_type: &str,
    ) -> Result<Self::ImageData, Self::Error> {
        let data_url = ["data:", mime_type, ";base64,", &base64::encode(img)].concat();
        self.get_image(&data_url).await
    }
}

async fn fetch(url: &str) -> Result<Vec<u8>, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = web_sys::window().ok_or(JsError::new("window is not defined"))?;
    let response: Response = JsFuture::from(window.fetch_with_request(&request))
        .await?
        .dyn_into()?;

    if !response.ok() {
        return Err(response.status().into());
    }

    let content =
        Uint8Array::new_with_byte_offset(&JsFuture::from(response.array_buffer()?).await?, 0)
            .to_vec();

    Ok(content)
}

async fn load_image(uri: &str) -> Result<HtmlImageElement, JsValue> {
    let image = HtmlImageElement::new()?;
    image.set_cross_origin(Some("anonymous"));
    JsFuture::from(Promise::new(&mut |resolve, reject| {
        image.set_onload(Some(&resolve));
        image.set_onerror(Some(&reject));
        image.set_src(uri);
    }))
    .await?;

    Ok(image)
}

fn resolve_url(path: &str, uri: &str) -> String {
    if uri.starts_with("data:") {
        uri.into()
    } else {
        [path, uri].concat()
    }
}

/// Error when calling fetch API
#[derive(Debug)]
pub struct FetchLoaderError(pub JsValue);

impl fmt::Display for FetchLoaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to fetch: {}", self.0.as_string().unwrap_or("unknown error".into()))
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FetchLoaderError {}
