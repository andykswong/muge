use super::Error;
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

    /// Loads a buffer.
    async fn get_buffer(&self, uri: &str) -> Result<Vec<u8>, Self::Error>;

    /// Loads an image.
    async fn get_image(&self, uri: &str) -> Result<Self::ImageData, Self::Error>;

    /// Decodes an image file blob of given mime type as image data.
    async fn decode_image(
        &self,
        image: &[u8],
        mime_type: &str,
    ) -> Result<Self::ImageData, Self::Error>;
}

#[cfg(feature = "file-loader")]
pub use file_loader::GltfResourceFileLoader;

#[cfg(feature = "file-loader")]
mod file_loader {
    use super::{Error, GltfResourceLoader};
    use alloc::{boxed::Box, string::String, vec::Vec};
    use async_trait::async_trait;
    use core::fmt::{self, Debug};
    use data_url::DataUrl;
    use image::{
        error::{ImageFormatHint, UnsupportedError, UnsupportedErrorKind},
        ImageError, ImageFormat, ImageResult,
    };
    use mugl::Extent2D;
    use std::{fs::File, io::Read, path::PathBuf};

    /// Loader of glTF resources from file system.
    #[derive(Debug)]
    pub struct GltfResourceFileLoader {
        path: String,
    }

    impl Default for GltfResourceFileLoader {
        fn default() -> Self {
            Self { path: "./".into() }
        }
    }

    #[async_trait(?Send)]
    impl GltfResourceLoader for GltfResourceFileLoader {
        type Error = Box<Error>;
        type ImageData = (Vec<u8>, Extent2D);

        #[inline]
        fn set_path(&mut self, path: &str) {
            self.path = path.into();
        }

        async fn get_buffer(&self, uri: &str) -> Result<Vec<u8>, Self::Error> {
            if let Some(data) = try_read_data_url(uri, false)? {
                Ok(data)
            } else {
                Ok(read_file(&self.path, uri)?)
            }
        }

        async fn get_image(&self, uri: &str) -> Result<Self::ImageData, Self::Error> {
            let data = if let Some(data) = try_read_data_url(uri, true)? {
                data
            } else {
                read_file(&self.path, uri)?
            };
            let dynimage = image::load_from_memory(data.as_slice())?;
            let size = Extent2D(dynimage.width(), dynimage.height());
            Ok((dynimage.into_bytes(), size))
        }

        async fn decode_image(
            &self,
            img: &[u8],
            mime_type: &str,
        ) -> Result<Self::ImageData, Self::Error> {
            let format = get_image_format(mime_type)?;
            let dynimage = image::load_from_memory_with_format(img, format)?;
            let size = Extent2D(dynimage.width(), dynimage.height());
            Ok((dynimage.into_bytes(), size))
        }
    }

    fn read_file(path: &str, file: &str) -> Result<Vec<u8>, std::io::Error> {
        let mut file_path = PathBuf::from(path);
        file_path.push(file);
        let mut file = File::open(file_path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        Ok(content)
    }

    fn try_read_data_url(uri: &str, is_image: bool) -> Result<Option<Vec<u8>>, Box<Error>> {
        if uri.starts_with("data:") {
            let data_url =
                DataUrl::process(uri).map_err(|err| DataUrlError::InvalidDataUrl(err))?;
            let is_supported_mime = if is_image {
                is_supported_image_mime(&data_url.mime_type().type_, &data_url.mime_type().subtype)
            } else {
                is_supported_buffer_mime(&data_url.mime_type().type_, &data_url.mime_type().subtype)
            };
            if !is_supported_mime {
                return Err(DataUrlError::UnsupportedMimeType(
                    data_url.mime_type().type_.clone(),
                    data_url.mime_type().subtype.clone(),
                )
                .into());
            }

            let (data, _) = data_url
                .decode_to_vec()
                .map_err(|err| DataUrlError::InvalidBase64(err))?;
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    #[inline]
    fn is_supported_buffer_mime(type_: &str, subtype: &str) -> bool {
        type_ == "application" && (subtype == "gltf-buffer" || subtype == "octet-stream")
    }

    #[inline]
    fn is_supported_image_mime(type_: &str, subtype: &str) -> bool {
        type_ == "image" && (subtype == "png" || subtype == "jpeg")
    }

    fn get_image_format(mime_type: &str) -> ImageResult<ImageFormat> {
        if mime_type == "image/png" {
            Ok(ImageFormat::Png)
        } else if mime_type == "image/jpeg" {
            Ok(ImageFormat::Jpeg)
        } else {
            let format_hint = ImageFormatHint::Name(mime_type.into());
            Err(ImageError::Unsupported(
                UnsupportedError::from_format_and_kind(
                    format_hint.clone(),
                    UnsupportedErrorKind::Format(format_hint),
                ),
            ))
        }
    }

    #[derive(Debug)]
    enum DataUrlError {
        InvalidDataUrl(data_url::DataUrlError),
        InvalidBase64(data_url::forgiving_base64::InvalidBase64),
        UnsupportedMimeType(String, String),
    }

    impl fmt::Display for DataUrlError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                DataUrlError::InvalidDataUrl(err) => err.fmt(f),
                DataUrlError::InvalidBase64(err) => err.fmt(f),
                DataUrlError::UnsupportedMimeType(type_, sub_type) => {
                    write!(f, "unsupported mime type: {}/{}", type_, sub_type)
                }
            }
        }
    }

    impl std::error::Error for DataUrlError {}
}
