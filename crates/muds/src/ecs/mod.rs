//! Entity-component registry.

mod entity_component;
mod registry;

pub mod archetype;
pub mod resource;
pub mod storage;

/// Commonly used types.
pub mod prelude {
    pub use super::archetype::Archetypes;
    pub use super::entity_component::*;
    pub use super::registry::*;
    pub use super::resource::{Entities, Components, Resources, ResourceLocator};

    #[cfg(feature = "muds-derive")]
    pub use muds_derive::{Component, Entity};
}

pub use prelude::*;
