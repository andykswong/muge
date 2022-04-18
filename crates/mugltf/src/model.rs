//! glTF 2.0 data model.

use alloc::string::String;
use alloc::vec::Vec;
use alloc::{borrow::ToOwned, collections::BTreeMap};
use mugl::{gl_const, AddressMode, FilterMode, PrimitiveTopology};

/// Id type.
pub type Id = usize;

/// Size type.
pub type Size = usize;

/// Float type.
pub type Float = f32;

pub type Map<K, V> = BTreeMap<K, V>;

/// The root object for a glTF asset.
/// See: <https://www.khronos.org/registry/glTF/specs/2.0/glTF-2.0.html>
#[derive(Clone, Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase"),
    serde(default)
)]
#[repr(C)]
pub struct Gltf {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub accessors: Vec<Accessor>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub animations: Vec<Animation>,
    pub asset: Asset,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub buffers: Vec<Buffer>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub buffer_views: Vec<BufferView>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub cameras: Vec<Camera>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub images: Vec<Image>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub materials: Vec<Material>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub meshes: Vec<Mesh>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub nodes: Vec<Node>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub samplers: Vec<Sampler>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub scene: Option<Id>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub scenes: Vec<Scene>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub skins: Vec<Skin>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub textures: Vec<Texture>,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub extensions_used: Vec<String>,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub extensions_required: Vec<String>,
}

/// Application-specific data.
#[cfg(feature = "serde")]
pub type Extras = serde_json::Value;

/// JSON object with extension-specific objects.
#[cfg(feature = "serde")]
pub type Extensions = Map<String, serde_json::Value>;

/// Metadata about the glTF asset.
#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[repr(C)]
pub struct Asset {
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "String::is_empty")
    )]
    pub copyright: String,
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "String::is_empty")
    )]
    pub generator: String,
    pub version: String,
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "String::is_empty")
    )]
    pub min_version: String,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

impl Default for Asset {
    fn default() -> Self {
        Self {
            copyright: Default::default(),
            generator: Default::default(),
            version: "2.0".to_owned(),
            min_version: Default::default(),
            #[cfg(feature = "gltf-extras")]
            extras: Default::default(),
            #[cfg(feature = "gltf-extensions")]
            extensions: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[repr(C)]
pub struct Accessor {
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub buffer_view: Option<Id>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub byte_offset: Size,
    pub component_type: AccessorComponentType,
    #[cfg_attr(feature = "serde", serde(default), serde(skip_serializing_if = "is_false"))]
    pub normalized: bool,
    pub count: Size,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub ty: AccessorType,
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub max: Vec<f64>,
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub min: Vec<f64>,
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub sparse: Option<AccessorSparse>,
    #[cfg(feature = "gltf-name")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "String::is_empty")
    )]
    pub name: String,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[repr(C)]
pub struct AccessorSparse {
    pub count: Size,
    pub indices: AccessorSparseIndices,
    pub values: AccessorSparseValues,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[repr(C)]
pub struct AccessorSparseIndices {
    pub buffer_view: Id,
    #[cfg_attr(feature = "serde", serde(default))]
    pub byte_offset: Size,
    pub component_type: AccessorIndicesComponentType,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[repr(C)]
pub struct AccessorSparseValues {
    pub buffer_view: Id,
    #[cfg_attr(feature = "serde", serde(default))]
    pub byte_offset: Size,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "UPPERCASE")
)]
#[repr(u32)]
pub enum AccessorType {
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)
)]
#[repr(u32)]
pub enum AccessorComponentType {
    Byte = gl_const::BYTE,
    UnsignedByte = gl_const::UNSIGNED_BYTE,
    Short = gl_const::SHORT,
    UnsignedShort = gl_const::UNSIGNED_SHORT,
    UnsignedInt = gl_const::UNSIGNED_INT,
    Float = gl_const::FLOAT,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)
)]
#[repr(u32)]
pub enum AccessorIndicesComponentType {
    UnsignedByte = gl_const::UNSIGNED_BYTE,
    UnsignedShort = gl_const::UNSIGNED_SHORT,
    UnsignedInt = gl_const::UNSIGNED_INT,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[repr(C)]
pub struct Animation {
    pub channels: Vec<AnimationChannel>,
    pub samplers: Vec<AnimationSampler>,
    #[cfg(feature = "gltf-name")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "String::is_empty")
    )]
    pub name: String,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[repr(C)]
pub struct AnimationChannel {
    pub sampler: Id,
    pub target: AnimationChannelTarget,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[repr(C)]
pub struct AnimationChannelTarget {
    pub node: Id,
    pub path: NodePath,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
#[repr(u32)]
pub enum NodePath {
    Weights,
    Translation,
    Rotation,
    Scale,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[repr(C)]
pub struct AnimationSampler {
    pub input: Id,
    pub output: Id,
    pub interpolation: Interpolation,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "UPPERCASE")
)]
#[repr(u32)]
pub enum Interpolation {
    Linear,
    Step,
    Cubicspline,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[repr(C)]
pub struct Buffer {
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "String::is_empty")
    )]
    pub uri: String,
    pub byte_length: Size,
    #[cfg(feature = "gltf-name")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "String::is_empty")
    )]
    pub name: String,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase"),
    serde(default)
)]
#[repr(C)]
pub struct BufferView {
    pub buffer: Id,
    pub byte_offset: Size,
    pub byte_length: Size,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "is_zero"))]
    pub byte_stride: Size,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub target: Option<BufferViewTarget>,
    #[cfg(feature = "gltf-name")]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "String::is_empty"))]
    pub name: String,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)
)]
#[repr(u32)]
pub enum BufferViewTarget {
    Vertex = gl_const::ARRAY_BUFFER,
    Index = gl_const::ELEMENT_ARRAY_BUFFER,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase"),
    serde(tag = "type")
)]
#[repr(C)]
pub enum Camera {
    Orthographic {
        orthographic: CameraOrthographic,
        #[cfg(feature = "gltf-name")]
        #[cfg_attr(
            feature = "serde",
            serde(default),
            serde(skip_serializing_if = "String::is_empty")
        )]
        name: String,
        #[cfg(feature = "gltf-extras")]
        #[cfg_attr(
            feature = "serde",
            serde(default),
            serde(skip_serializing_if = "serde_json::Value::is_null")
        )]
        extras: Extras,
        #[cfg(feature = "gltf-extensions")]
        #[cfg_attr(
            feature = "serde",
            serde(default),
            serde(skip_serializing_if = "Option::is_none")
        )]
        extensions: Option<Extensions>,
    },
    Perspective {
        perspective: CameraPerspective,
        #[cfg(feature = "gltf-name")]
        #[cfg_attr(
            feature = "serde",
            serde(default),
            serde(skip_serializing_if = "String::is_empty")
        )]
        name: String,
        #[cfg(feature = "gltf-extras")]
        #[cfg_attr(
            feature = "serde",
            serde(default),
            serde(skip_serializing_if = "serde_json::Value::is_null")
        )]
        extras: Extras,
        #[cfg(feature = "gltf-extensions")]
        #[cfg_attr(
            feature = "serde",
            serde(default),
            serde(skip_serializing_if = "Option::is_none")
        )]
        extensions: Option<Extensions>,
    },
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[repr(C)]
pub struct CameraOrthographic {
    pub xmag: Float,
    pub ymag: Float,
    pub zfar: Float,
    pub znear: Float,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase"),
    serde(default)
)]
#[repr(C)]
pub struct CameraPerspective {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub aspect_ratio: Option<Float>,
    pub yfov: Float,
    pub zfar: Float,
    pub znear: Float,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

impl Default for CameraPerspective {
    fn default() -> Self {
        Self {
            aspect_ratio: None,
            yfov: 0.,
            zfar: Float::INFINITY,
            znear: 0.,
            #[cfg(feature = "gltf-extras")]
            extras: Default::default(),
            #[cfg(feature = "gltf-extensions")]
            extensions: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase"),
    serde(default)
)]
#[repr(C)]
pub struct Image {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "String::is_empty"))]
    pub uri: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "String::is_empty"))]
    pub mime_type: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub buffer_view: Option<Id>,
    #[cfg(feature = "gltf-name")]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "String::is_empty"))]
    pub name: String,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase"),
    serde(default)
)]
#[repr(C)]
pub struct Material {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub pbr_metallic_roughness: Option<PbrMetallicRoughness>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub normal_texture: Option<NormalTextureInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub occlusion_texture: Option<OcclusionTextureInfo>,
    pub emissive_factor: [Float; 3],
    pub alpha_mode: AlphaMode,
    pub alpha_cutoff: Float,
    pub double_sided: bool,
    #[cfg(feature = "gltf-name")]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "String::is_empty"))]
    pub name: String,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            pbr_metallic_roughness: None,
            normal_texture: None,
            occlusion_texture: None,
            emissive_factor: [0., 0., 0.],
            alpha_mode: Default::default(),
            alpha_cutoff: 0.5,
            double_sided: false,
            #[cfg(feature = "gltf-name")]
            name: Default::default(),
            #[cfg(feature = "gltf-extras")]
            extras: Default::default(),
            #[cfg(feature = "gltf-extensions")]
            extensions: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase"),
    serde(default)
)]
#[repr(C)]
pub struct PbrMetallicRoughness {
    pub base_color_factor: [Float; 4],
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub base_color_texture: Option<TextureInfo>,
    pub metallic_factor: Float,
    pub roughness_factor: Float,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub metallic_roughness_texture: Option<TextureInfo>,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

impl Default for PbrMetallicRoughness {
    fn default() -> Self {
        Self {
            base_color_factor: [1., 1., 1., 1.],
            base_color_texture: None,
            metallic_factor: 1.,
            roughness_factor: 1.,
            metallic_roughness_texture: None,
            #[cfg(feature = "gltf-extras")]
            extras: Default::default(),
            #[cfg(feature = "gltf-extensions")]
            extensions: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase"),
    serde(default)
)]
#[repr(C)]
pub struct TextureInfo {
    pub index: Id,
    pub tex_coord: Size,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase"),
    serde(default)
)]
#[repr(C)]
pub struct NormalTextureInfo {
    pub index: Id,
    pub tex_coord: Size,
    pub scale: Float,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

impl Default for NormalTextureInfo {
    fn default() -> Self {
        Self {
            index: 0,
            tex_coord: 0,
            scale: 1.,
            #[cfg(feature = "gltf-extras")]
            extras: Default::default(),
            #[cfg(feature = "gltf-extensions")]
            extensions: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase"),
    serde(default)
)]
#[repr(C)]
pub struct OcclusionTextureInfo {
    pub index: Id,
    pub tex_coord: Size,
    pub strength: Float,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

impl Default for OcclusionTextureInfo {
    fn default() -> Self {
        Self {
            index: 0,
            tex_coord: 0,
            strength: 1.,
            #[cfg(feature = "gltf-extras")]
            extras: Default::default(),
            #[cfg(feature = "gltf-extensions")]
            extensions: Default::default(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "UPPERCASE")
)]
#[repr(u32)]
pub enum AlphaMode {
    Opaque,
    Mask,
    Blend,
}

impl Default for AlphaMode {
    fn default() -> Self {
        Self::Opaque
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[repr(C)]
pub struct Mesh {
    pub primitives: Vec<MeshPrimitive>,
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub weights: Vec<Float>,
    #[cfg(feature = "gltf-name")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "String::is_empty")
    )]
    pub name: String,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[repr(C)]
pub struct MeshPrimitive {
    pub attributes: Map<String, Id>,
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub indices: Option<Id>,
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub material: Option<Id>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub mode: PrimitiveTopology,
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub targets: Vec<Map<String, Id>>,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase"),
    serde(default)
)]
#[repr(C)]
pub struct Node {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub camera: Option<Id>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub children: Vec<Id>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub skin: Option<Id>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub matrix: Option<[Float; 16]>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub mesh: Option<Id>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub rotation: Option<[Float; 4]>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub scale: Option<[Float; 3]>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub translation: Option<[Float; 3]>,
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub weights: Vec<Float>,
    #[cfg(feature = "gltf-name")]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "String::is_empty"))]
    pub name: String,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase"),
    serde(default)
)]
#[repr(C)]
pub struct Sampler {
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub mag_filter: Option<FilterMode>,
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub min_filter: Option<MinFilterMode>,
    pub wrap_s: AddressMode,
    pub wrap_t: AddressMode,
    #[cfg(feature = "gltf-name")]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "String::is_empty"))]
    pub name: String,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

impl Default for Sampler {
    fn default() -> Self {
        Self {
            mag_filter: None,
            min_filter: None,
            wrap_s: AddressMode::Repeat,
            wrap_t: AddressMode::Repeat,
            #[cfg(feature = "gltf-name")]
            name: Default::default(),
            #[cfg(feature = "gltf-extras")]
            extras: Default::default(),
            #[cfg(feature = "gltf-extensions")]
            extensions: Default::default(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)
)]
#[repr(u32)]
pub enum MinFilterMode {
    Nearest = gl_const::NEAREST,
    Linear = gl_const::LINEAR,
    NearestMipmapNearest = gl_const::NEAREST_MIPMAP_NEAREST,
    NearestMipmapLinear = gl_const::NEAREST_MIPMAP_LINEAR,
    LinearMipmapNearest = gl_const::LINEAR_MIPMAP_NEAREST,
    LinearMipmapLinear = gl_const::LINEAR_MIPMAP_LINEAR,
}

impl Default for MinFilterMode {
    fn default() -> Self {
        Self::Nearest
    }
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase"),
    serde(default)
)]
#[repr(C)]
pub struct Scene {
    pub nodes: Vec<Id>,
    #[cfg(feature = "gltf-name")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "String::is_empty")
    )]
    pub name: String,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[repr(C)]
pub struct Skin {
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub inverse_bind_matrices: Option<Id>,
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub skeleton: Option<Id>,
    pub joints: Vec<Id>,
    #[cfg(feature = "gltf-name")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "String::is_empty")
    )]
    pub name: String,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase"),
    serde(default)
)]
#[repr(C)]
pub struct Texture {
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub sampler: Option<Id>,
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub source: Option<Id>,
    #[cfg(feature = "gltf-name")]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "String::is_empty"))]
    pub name: String,
    #[cfg(feature = "gltf-extras")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "serde_json::Value::is_null")
    )]
    pub extras: Extras,
    #[cfg(feature = "gltf-extensions")]
    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub extensions: Option<Extensions>,
}

/// Checks if size is 0
#[cfg(feature = "serde")]
#[inline]
fn is_zero(size: &Size) -> bool {
    *size == 0
}

/// Checks if bool is false
#[cfg(feature = "serde")]
#[inline]
fn is_false(b: &bool) -> bool {
    !*b
}
