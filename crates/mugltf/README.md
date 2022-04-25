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
mugltf = "0.1"
```
Features:
- `std` - (default) enables `std` support.
- `serde` - (default) enables `serde` parsing of glTF assets
- `gltf-name` - enables the `name` field for all glTF nodes
- `gltf-extras` - enables the `extras` field for all glTF nodes
- `gltf-extensions` - enables the `extensions` field for all glTF nodes
- `file-loader` - enables `GltfResourceFileLoader` for loading glTF resources from file system
- `fetch-loader` - enables `GltfResourceFetchLoader` for loading glTF resources using fetch API for web WASM

## [Documentation](https://docs.rs/mugltf)
See Docs.rs: https://docs.rs/mugltf

## Usage

```rust
// Init a loader and set the base path (Use mugltf::GltfResourceFetchLoader for WASM web environment)
let mut loader = mugltf::GltfResourceFileLoader::default();
loader.set_path("./");

// Load a glTF JSON / GLB asset async
// You can set the last parameter to false to skip loading buffers and images
let asset = mugltf::GltfAsset.load(&loader, "./test.glb", true).await?;

// You can now read the glTF model and resources.
let gltf_model = asset.gltf;
let binary_chunk = asset.bin;
let buffers = asset.buffers;
let images = asset.images;
```

See [tests](./tests/) for more example usages.
