<h1 align="center">〰ᗰᑌᗪᔕ〰</h1>
<h2 align="center">Minimalistic Data Structures and Entity-Component-System Library</h2>
<br />
<p align="center">
  <a href="./LICENSE"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT" /></a> 
  <a href="https://crates.io/crates/muds"><img src="https://img.shields.io/crates/v/muds.svg" alt="Crates.io" /></a> 
  <a href="https://docs.rs/muds"><img src="https://docs.rs/muds/badge.svg" alt="Docs.rs" /></a> 
</p>

## Overview
`muds` is a minimalistic data structure library for data-oriented design in high-performance, realtime applications such as games. It provides a collections library with data structures that support [generational indices](./src/genindex.rs) as key, and an [entity-component-system](https://en.wikipedia.org/wiki/Entity_component_system) (ECS) library built on top of it.

## Highlights
- Flexible and extensible generational-index ECS model
  - Supports multiple entity types akin to a relational database, with type-safe entity IDs
  - Custom storage type for entities and components
- Cross-platform; compatible with `no_std` and WASM build. Defaults to f64 generational indices for interop with JS from WASM.
- Minimal size, minimal dependencies, good performance
- Library, not framework. Unlike other ECS with `System` scheduler / dispatcher, `muds` does not take control of your program flow. 
- Uses [cons](https://en.wikipedia.org/wiki/Cons) to perform type-level recursion elegantly, instead of using macros to implement for tuples of fixed lengths, as seen in other ECS libraries.

## Install
```toml
[dependencies]
muds = "0.1"
```
Features:
- `std` - enables `std` support. enabled by default.
- `serde` - enables `serde` serialize/deserialize implementations of collections and indices
- `derive` - enables `#[derive(Entity)]` and `#[derive(Component)]` macros
- `index-u64` - uses `IndexU64` as the generational index type for ECS instead of the default `IndexF64`

## [Documentation](https://docs.rs/muds)
See Docs.rs: https://docs.rs/muds

## Usage
Below is a sample usage of the ECS library. See [benches](./benches/ecs/) for more examples. 

```rust
use muds::prelude::*;
use muds::ecs::storage::SparseSetStorage;

// 1. Defines the entity and components.

#[derive(Entity, Debug)]
struct Ent;

#[derive(Component, Debug)]
struct Pos(u32, u32);

// `#[storage(S)]` can be used to customize the entity / component storage type.
#[derive(Component, Debug)]
#[storage(SparseSetStorage)]
struct Vel(u32, u32);

// 2. Registers the entity-components archetype to registry.
let mut registry = Registry::default();
registry.register_archetype::<Ent, Cons!(Pos, Vel)>();

// 3. Insert entities/components to mut storage.
// registry.storage returns cons of collection types.
// Each entity/component storage can be retrieved either as immutable (&C) or mutable (&mut C).
{
  let cons!(mut ent, mut pos, mut vel) = registry.storage::<&mut Ent, Cons!(&mut Pos, &mut Vel)>();
  for i in 0..10 {
      let eid = ent.insert(E);
      pos.insert(eid, Pos(i * 2, i * 2 + 1));
      vel.insert(eid, Vel(i, i + 1));
  }
}

// 4. Storages are just standard Map types that can be iterated.
// Use MapJoin trait to jointly iterate components as cons.
{
  let cons!(_ent, mut pos, vel) = registry.storage::<&Ent, Cons!(&mut Pos, &Vel)>();
  for cons!(_eid, v, p) in pos.iter_mut().cons().map_join(&*vel) {
      p.0 += v.0;
      p.1 += v.1;
  }
}
```

## License
This repository and the code inside it is licensed under the MIT License. Read [LICENSE](./LICENSE) for more information.
