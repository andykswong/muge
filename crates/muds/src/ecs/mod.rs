//! Entity-Component-Resource registry.

mod entity_component;
mod registry_traits;
mod resource;

pub mod archetype;
pub mod registry;
pub mod storage;

/// Commonly used types.
pub mod prelude {
    pub use super::archetype::Archetypes;
    pub use super::entity_component::*;
    pub use super::registry::{Registry, RegistryKey};
    pub use super::resource::*;

    #[cfg(feature = "muds-derive")]
    pub use muds_derive::{Component, Entity};
}

pub use prelude::*;
