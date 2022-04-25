#![cfg(all(feature = "serde", feature = "file-loader"))]

use mugltf::{GltfAsset, GltfResourceFileLoader, GltfResourceLoader};
use std::{error::Error, path::PathBuf};

#[test]
fn test_load_gltf() -> Result<(), Box<dyn Error>> {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests");
    root.push("model/SimpleMorph/glTF/");

    let mut loader = GltfResourceFileLoader::default();
    loader.set_path(root.to_str().expect("invalid root path"));

    let asset = pollster::block_on(GltfAsset::load(&loader, "SimpleMorph.gltf", true))?;

    assert_gltf_res_loaded(&asset);

    Ok(())
}

#[test]
fn test_load_gltf_embedded() -> Result<(), Box<dyn Error>> {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests");
    root.push("model/SimpleMorph/glTF-Embedded/");

    let mut loader = GltfResourceFileLoader::default();
    loader.set_path(root.to_str().expect("invalid root path"));

    let asset = pollster::block_on(GltfAsset::load(&loader, "SimpleMorph.gltf", true))?;

    assert_gltf_res_loaded(&asset);

    Ok(())
}

#[test]
fn test_load_glb() -> Result<(), Box<dyn Error>> {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests");
    root.push("model/InterpolationTest/glTF-Binary/");

    let mut loader = GltfResourceFileLoader::default();
    loader.set_path(root.to_str().expect("invalid root path"));

    let asset = pollster::block_on(GltfAsset::load(&loader, "InterpolationTest.glb", true))?;

    assert_gltf_res_loaded(&asset);

    Ok(())
}

fn assert_gltf_res_loaded(asset: &GltfAsset) {
    assert!(asset.bin.as_ref().is_empty());
    assert_eq!(asset.gltf.buffers.len(), asset.buffers.len());
    assert_eq!(asset.gltf.images.len(), asset.images.len());
}
