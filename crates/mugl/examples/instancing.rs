#![cfg_attr(target_family = "wasm", no_main)]

mod app;
mod common;

main!(app::InstancingExample);
