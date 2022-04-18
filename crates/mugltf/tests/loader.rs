#![cfg(all(feature = "serde", feature = "file-loader"))]

use mugltf::{GltfAsset, GltfResourceFileLoader, GltfResourceLoader};
use std::{error::Error, path::PathBuf};

#[test]
fn test_load_gltf() -> Result<(), Box<dyn Error>> {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests");
    root.push("model/SimpleMorph/glTF");

    let mut loader = GltfResourceFileLoader::default();
    loader.set_path(root.to_str().expect("invalid root path"));
    let mut asset =
        <GltfAsset>::parse_gltf(include_str!("./model/SimpleMorph/glTF/SimpleMorph.gltf"))?;

    pollster::block_on(asset.load_resources(&loader))?;

    assert_gltf_res_loaded(&asset);

    Ok(())
}

#[test]
fn test_load_gltf_embedded() -> Result<(), Box<dyn Error>> {
    let loader = GltfResourceFileLoader::default();
    let mut asset = <GltfAsset>::parse_gltf(include_str!(
        "./model/SimpleMorph/glTF-Embedded/SimpleMorph.gltf"
    ))?;

    pollster::block_on(asset.load_resources(&loader))?;

    assert_gltf_res_loaded(&asset);

    Ok(())
}

#[test]
fn test_load_glb() -> Result<(), Box<dyn Error>> {
    let loader = GltfResourceFileLoader::default();
    let mut asset = <GltfAsset>::parse_glb(include_bytes!(
        "./model/InterpolationTest/glTF-Binary/InterpolationTest.glb"
    ))?;
    pollster::block_on(asset.load_resources(&loader))?;

    assert_gltf_res_loaded(&asset);

    Ok(())
}

fn assert_gltf_res_loaded(asset: &GltfAsset) {
    assert_eq!(asset.gltf.buffers.len(), asset.buffers.len());
    assert_eq!(asset.gltf.images.len(), asset.images.len());
    if asset.gltf.buffers[0].uri.is_empty() {
        assert_eq!(asset.bin, asset.buffers[0]);
    }
}
