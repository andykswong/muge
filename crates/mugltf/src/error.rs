use super::Id;
use alloc::boxed::Box;
use core::fmt;

cfg_if::cfg_if! {
if #[cfg(feature = "std")] {
    /// Error type.
    pub type Error = dyn std::error::Error;
} else {
    /// Error type.
    pub type Error = dyn core::any::Any;
}
}

/// Error when parsing a glTF / GLB file.
#[derive(Debug, Default)]
pub struct ParseGltfError {
    kind: ParseGltfErrorKind,
    #[allow(unused)]
    error: Option<Box<Error>>,
}

impl ParseGltfError {
    /// Creates a new `ParseGLBError`.
    #[inline]
    pub fn new<E: Into<Box<Error>>>(kind: ParseGltfErrorKind, error: E) -> Self {
        Self {
            kind,
            error: Some(error.into()),
        }
    }
}

impl fmt::Display for ParseGltfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ParseGltfErrorKind::InvalidHeader => write!(f, "invalid GLB header"),
            ParseGltfErrorKind::UnsupportedVersion => write!(f, "unsupported glTF version"),
            ParseGltfErrorKind::InvalidChunkHeader => write!(f, "invalid GLB chunk header"),
            ParseGltfErrorKind::InvalidChunk => write!(f, "invalid GLB chunk data"),
            ParseGltfErrorKind::MissingJson => write!(f, "missing glTF JSON content"),
            _ => write!(f, "invalid GLB"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseGltfError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.error.as_ref()?.as_ref())
    }
}

/// The kind of glTF parsing error.
#[derive(Clone, Copy, Debug)]
pub enum ParseGltfErrorKind {
    InvalidHeader,
    UnsupportedVersion,
    InvalidChunkHeader,
    InvalidChunk,
    InvalidJson,
    MissingJson,
    Other,
}

impl Default for ParseGltfErrorKind {
    fn default() -> Self {
        Self::Other
    }
}

impl From<ParseGltfErrorKind> for ParseGltfError {
    fn from(kind: ParseGltfErrorKind) -> Self {
        Self { kind, error: None }
    }
}

/// Error when loading resources for a glTF file.
#[derive(Debug, Default)]
pub struct LoadGltfResourceError {
    kind: LoadGltfResourceErrorKind,
    #[allow(unused)]
    error: Option<Box<Error>>,
}

impl LoadGltfResourceError {
    /// Creates a new `LoadGltfResourceError`.
    #[inline]
    pub fn new<E: Into<Box<Error>>>(kind: LoadGltfResourceErrorKind, error: E) -> Self {
        Self {
            kind,
            error: Some(error.into()),
        }
    }
}

impl fmt::Display for LoadGltfResourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            LoadGltfResourceErrorKind::InvalidImage(id) => write!(f, "invalid image {}", id),
            _ => write!(f, "failed to load resource"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for LoadGltfResourceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.error.as_ref()?.as_ref())
    }
}

/// The kind of glTF resource loading error.
#[derive(Clone, Copy, Debug)]
pub enum LoadGltfResourceErrorKind {
    InvalidImage(Id),
    LoadError,
}

impl Default for LoadGltfResourceErrorKind {
    fn default() -> Self {
        Self::LoadError
    }
}

impl From<LoadGltfResourceErrorKind> for LoadGltfResourceError {
    fn from(kind: LoadGltfResourceErrorKind) -> Self {
        Self { kind, error: None }
    }
}
