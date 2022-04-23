#![cfg(feature = "derive")]

use muds::{
    cons,
    ecs::{Archetypes, Component, Components, Entities, Entity, Registry},
    Cons,
};

macro_rules! create_entities {
    ($registry:ident; $( $variants:ident ),*) => {
        $(
            #[derive(Component, Debug)]
            struct $variants(f32);
            $registry.register_component::<Data, $variants>();
            {
                let cons!(mut entities, mut variants) = $registry.storage::<&mut Data, Cons!(&mut $variants)>();

                for _i in 0..20 {
                    let id = entities.insert(Data(1.0));
                    variants.insert(id, $variants(0.0));
                }
            }
        )*
    };
}

#[derive(Entity, Debug)]
struct Data(f32);

pub struct Benchmark(Registry);

impl Benchmark {
    pub fn new() -> Self {
        let mut registry = Registry::default();
        registry.register_entity::<Data>();

        create_entities!(registry; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

        Self(registry)
    }

    pub fn run(&mut self) {
        let mut entities = self.0.entities_mut::<Data>();

        for (_, data) in entities.iter_mut() {
            data.0 *= 2.0;
        }
    }
}
