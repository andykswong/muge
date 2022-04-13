<h1 align="center">🅼🆄🅽🆄🅼</h1>
<h2 align="center">Rust Micro Library for 3D Math</h2>
<br />
<p align="center">
  <a href="./LICENSE"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT" /></a> 
  <a href="https://crates.io/crates/munum"><img src="https://img.shields.io/crates/v/munum.svg" alt="Crates.io" /></a> 
  <a href="https://docs.rs/munum"><img src="https://docs.rs/munum/badge.svg" alt="Docs.rs" /></a> 
  <a href="https://github.com/andykswong/muge/actions"><img src="https://github.com/andykswong/muge/actions/workflows/build.yaml/badge.svg" alt="build" /></a>
</p>

## Overview
`munum` is a minimalistic numerical library for high-performance 3D math. This is a pure Rust no_std compatible port of the JavaScript / AssemblyScript version of munum: https://github.com/andykswong/munum

## Install
```toml
[dependencies]
munum = "0.1.0"
```
Features:
- `std` - enables `std` support. enabled by default.
- `libm` - can be used instead of `std` to enable trigonometry related functions in `no_std` environment
- `serde` - enables `serde` serialize/deserialize implementations

## [Documentation](https://docs.rs/munum)
See Docs.rs: https://docs.rs/munum

## Usage
Sample usage to build a perspective camera view-projection matrix:

```rust
use core::f32::{consts::PI, INFINITY};
use munum::{transform, vec3};

let eye = vec3(0_f32, 2., 0.);
let target = vec3(0., 0.6, 0.);
let up = vec3(0., 0., -1.);
let view = transform::look_at(eye, target, up);

let proj = transform::perspective(2., PI/2., 1., INFINITY);

let view_proj = proj * view;
```

## License
This repository and the code inside it is licensed under the MIT License. Read [LICENSE](./LICENSE) for more information.
