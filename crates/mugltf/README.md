<h1 align="center">μglTF</h1>
<h2 align="center">Minimal glTF 2.0 asset loader for Rust</h2>
<br />
<p align="center">
  <a href="./LICENSE"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT" /></a> 
  <a href="https://crates.io/crates/mugltf"><img src="https://img.shields.io/crates/v/mugltf.svg" alt="Crates.io" /></a> 
  <a href="https://docs.rs/mugltf"><img src="https://docs.rs/mugltf/badge.svg" alt="Docs.rs" /></a> 
</p>

## Overview
`mugltf` is a minimal implementation of [glTF 2.0](https://www.khronos.org/registry/glTF/specs/2.0/glTF-2.0.html) asset model loader in Rust. It uses `serde` for parsing the glTF JSON. 

## Install
```toml
[dependencies]
mugltf = "0.1.0"
```
Features:
- `std` - (default) enables `std` support.
- `serde` - (default) enables `serde` parsing of glTF assets
- `gltf-name` - enables the `name` field for all glTF nodes
- `gltf-extras` - enables the `extras` field for all glTF nodes
- `gltf-extensions` - enables the `extensions` field for all glTF nodes
- `file-loader` - enables `GltfResourceFileLoader` for loading glTF resources from file system

## [Documentation](https://docs.rs/mugltf)
See Docs.rs: https://docs.rs/mugltf

## Usage

```rust
// 1. Start from parsing a gltf / glb file
let asset = <mugltf::GltfAsset>::parse_gltf(include_str!("./test.gltf"));
let glb_asset = <mugltf::GltfAsset>::parse_glb(include_str!("./test.glb"));

// You can now read the glTF model and binry chunk (for glb file).
let gltf_model = glb_asset.gltf;
let binary_chunk = glb_asset.bin;

// 2. Init a loader to load resources (external/embedded buffers and images) async
let mut loader = mugltf::GltfResourceFileLoader::default();
loader.set_path("./");
glb_asset.load_resources(&loader).await?;

// Buffer and image resources are now populated
let buffers = glb_asset.buffers;
let images = glb_asset.images;
```

See [tests](./tests/) for more example usages.
