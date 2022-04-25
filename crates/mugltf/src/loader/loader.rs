use crate::Error;
use alloc::{boxed::Box, vec::Vec};
use async_trait::async_trait;

/// Loader of glTF resources.
#[async_trait(?Send)]
pub trait GltfResourceLoader {
    /// Loading error type.
    type Error: Into<Box<Error>>;

    /// Image data type.
    type ImageData;

    /// Sets the root path of the asset.
    fn set_path(&mut self, _path: &str) {}

    /// Loads a glTF JSON or GLB file from path into bytes.
    async fn get_gltf(&self, uri: &str) -> Result<Vec<u8>, Self::Error>;

    /// Loads a binary buffer from path or data url.
    async fn get_buffer(&self, uri: &str) -> Result<Vec<u8>, Self::Error>;

    /// Loads an image from path or data url.
    async fn get_image(&self, uri: &str) -> Result<Self::ImageData, Self::Error>;

    /// Decodes an image file binary of given mime type as image data.
    async fn decode_image(
        &self,
        image: &[u8],
        mime_type: &str,
    ) -> Result<Self::ImageData, Self::Error>;
}
