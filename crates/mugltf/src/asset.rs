use crate::{
    model::Gltf, Error, GltfResourceLoader, LoadGltfResourceError, LoadGltfResourceErrorKind,
    ParseGltfError, ParseGltfErrorKind,
};
use alloc::{borrow::Cow, boxed::Box, vec::Vec};
use mugl::Extent2D;

/// glTF in ASCII
#[allow(dead_code)]
const GLB_HEADER_MAGIC: &[u8] = &[0x67, 0x6C, 0x54, 0x46];
#[allow(dead_code)]
const GLB_HEADER_LENGTH: usize = 12;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
enum GLBChunk {
    Json = 0x4E4F534A,
    Bin = 0x004E4942,
}

/// A parsed glTF 2.0 asset and resources.
/// See: <https://www.khronos.org/registry/glTF/specs/2.0/glTF-2.0.html>
#[derive(Debug)]
#[repr(C)]
pub struct GltfAsset<'a, ImageData = (Vec<u8>, Extent2D)> {
    /// glTF model
    pub gltf: Gltf,
    /// Binary chunk for GLB
    pub bin: Cow<'a, [u8]>,
    /// Buffer data
    pub buffers: Vec<Vec<u8>>,
    /// Image data
    pub images: Vec<ImageData>,
}

impl<'a, ImageData> Default for GltfAsset<'a, ImageData> {
    fn default() -> Self {
        Self {
            gltf: Default::default(),
            bin: Default::default(),
            buffers: Default::default(),
            images: Default::default(),
        }
    }
}

impl<'a, ImageData> GltfAsset<'a, ImageData> {
    /// Parses a binary glTF.
    #[cfg(feature = "serde")]
    pub fn parse_glb(data: &'a [u8]) -> Result<Self, ParseGltfError> {
        if data.len() < GLB_HEADER_LENGTH || GLB_HEADER_MAGIC != &data[0..4] {
            return Err(ParseGltfErrorKind::InvalidHeader.into());
        }
        if read_u32(data, 4, ParseGltfErrorKind::UnsupportedVersion)? != 2 {
            return Err(ParseGltfErrorKind::UnsupportedVersion.into());
        }

        let mut i = GLB_HEADER_LENGTH;

        let gltf: Gltf = {
            let chunk_len = read_u32(data, i, ParseGltfErrorKind::InvalidChunkHeader)? as usize;
            let chunk_type = read_u32(data, i + 4, ParseGltfErrorKind::InvalidChunkHeader)?;
            i += 8;

            if chunk_type != GLBChunk::Json as u32 || data.len() < i + chunk_len {
                return Err(ParseGltfErrorKind::InvalidChunkHeader.into());
            }

            let gltf_str = core::str::from_utf8(&data[i..(i + chunk_len)]).map_err(|err| {
                ParseGltfError::new::<Box<Error>>(ParseGltfErrorKind::InvalidChunk, Box::new(err))
            })?;

            i += chunk_len;

            Self::parse_gltf(gltf_str)?.gltf
        };

        let bin = if i < data.len() {
            let chunk_len = read_u32(data, i, ParseGltfErrorKind::InvalidChunkHeader)? as usize;
            let chunk_type = read_u32(data, i + 4, ParseGltfErrorKind::InvalidChunkHeader)?;
            i += 8;

            #[allow(unused_assignments)]
            if chunk_type != GLBChunk::Bin as u32 {
                Default::default()
            } else if data.len() < i + chunk_len {
                return Err(ParseGltfErrorKind::InvalidChunkHeader.into());
            } else {
                let bin = Cow::Borrowed(&data[i..(i + chunk_len)]);
                i += chunk_len;
                bin
            }
        } else {
            Default::default()
        };

        Ok(Self {
            gltf,
            bin,
            ..Default::default()
        })
    }

    /// Parses a glTF JSON string.
    #[cfg(feature = "serde")]
    pub fn parse_gltf(data: &str) -> Result<Self, ParseGltfError> {
        let gltf: Gltf = serde_json::from_str(data).map_err(|err| {
            ParseGltfError::new::<Box<Error>>(ParseGltfErrorKind::InvalidJson, Box::new(err))
        })?;

        if gltf.asset.version != "2.0" {
            return Err(ParseGltfErrorKind::UnsupportedVersion.into());
        }

        Ok(gltf.into())
    }

    /// Loads all resources of this glTF asset.
    pub async fn load_resources<L: GltfResourceLoader<ImageData = ImageData>>(
        &mut self,
        loader: &L,
    ) -> Result<(), LoadGltfResourceError> {
        let mut buffers = Vec::with_capacity(self.gltf.buffers.len());
        let mut images = Vec::with_capacity(self.gltf.images.len());

        for buffer in &self.gltf.buffers {
            if !buffer.uri.is_empty() {
                let data = loader.get_buffer(&buffer.uri).await.map_err(|err| {
                    LoadGltfResourceError::new(LoadGltfResourceErrorKind::LoadError, err)
                })?;
                buffers.push(data);
            } else {
                // Undefined uri; Refernce to bin chunk
                buffers.push(self.bin.clone().into_owned())
            }
        }

        for (image_id, image) in self.gltf.images.iter().enumerate() {
            if !image.uri.is_empty() {
                let data = loader.get_image(&image.uri).await.map_err(|err| {
                    LoadGltfResourceError::new(LoadGltfResourceErrorKind::LoadError, err)
                })?;
                images.push(data);
            } else {
                let buffer_id_and_range = image.buffer_view.and_then(|buffer_view_id| {
                    let buffer_view = self.gltf.buffer_views.get(buffer_view_id)?;
                    let range = buffer_view.byte_offset
                        ..(buffer_view.byte_offset + buffer_view.byte_length);
                    let data = buffers.get(buffer_view.buffer)?;

                    if range.end <= data.len() {
                        Some((buffer_view.buffer, range))
                    } else {
                        None
                    }
                });

                if let Some((buffer_id, range)) = buffer_id_and_range {
                    let data_slice = &buffers[buffer_id][range];
                    let image_data = loader
                        .decode_image(data_slice, &image.mime_type)
                        .await
                        .map_err(|err| {
                            LoadGltfResourceError::new(LoadGltfResourceErrorKind::LoadError, err)
                        })?;
                    images.push(image_data);
                } else {
                    return Err(LoadGltfResourceErrorKind::InvalidImage(image_id).into());
                }
            }
        }

        self.buffers = buffers;
        self.images = images;

        Ok(())
    }
}

impl<'a, ImageData> From<Gltf> for GltfAsset<'a, ImageData> {
    fn from(gltf: Gltf) -> Self {
        Self {
            gltf,
            ..Default::default()
        }
    }
}

#[allow(dead_code)]
fn read_u32(
    data: &[u8],
    offset: usize,
    error_kind: ParseGltfErrorKind,
) -> Result<u32, ParseGltfError> {
    Ok(u32::from_le_bytes(
        TryInto::<[u8; 4]>::try_into(&data[offset..(offset + 4)])
            .map_err(|err| ParseGltfError::new::<Box<Error>>(error_kind, Box::new(err)))?,
    ))
}
