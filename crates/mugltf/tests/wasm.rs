#![cfg(all(feature = "serde", feature = "fetch-loader"))]

use mugltf::{GltfAsset, GltfResourceFetchLoader, GltfResourceLoader};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use web_sys::HtmlImageElement;

wasm_bindgen_test_configure!(run_in_browser);

const GLTF_SAMPLE_PATH: &str = "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/63f026b2aa957d3e8207f6dd798608993e33fb0d/2.0/";

#[wasm_bindgen_test]
async fn test_load_gltf() {
    let mut loader = GltfResourceFetchLoader::default();
    loader.set_path(&[GLTF_SAMPLE_PATH, "SimpleMorph/glTF/"].concat());

    let asset = GltfAsset::load(&loader, "SimpleMorph.gltf", true)
        .await
        .unwrap();

    assert_gltf_res_loaded(&asset);
}

#[wasm_bindgen_test]
async fn test_load_glb() {
    let mut loader = GltfResourceFetchLoader::default();
    loader.set_path(&[GLTF_SAMPLE_PATH, "InterpolationTest/glTF-Binary/"].concat());

    let asset = GltfAsset::load(&loader, "InterpolationTest.glb", true)
        .await
        .unwrap();

    assert_gltf_res_loaded(&asset);
}

fn assert_gltf_res_loaded(asset: &GltfAsset<HtmlImageElement>) {
    assert!(asset.bin.as_ref().is_empty());
    assert_eq!(asset.gltf.buffers.len(), asset.buffers.len());
    assert_eq!(asset.gltf.images.len(), asset.images.len());
}
