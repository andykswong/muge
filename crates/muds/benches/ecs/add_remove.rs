#![cfg(feature = "derive")]

use muds::{
    cons,
    ecs::{Archetypes, Component, Components, Entity, Registry},
    Cons,
};

const ENTITIES: usize = 10_000;

#[derive(Entity, Debug)]
struct E;

#[derive(Component, Debug)]
struct A(f32);

#[derive(Component, Debug)]
struct B(f32);

pub struct Benchmark(Registry);

impl Benchmark {
    pub fn new() -> Self {
        let mut registry = Registry::default();
        registry.register_archetype::<E, Cons!(A, B)>();

        {
            let cons!(mut e, mut a) = registry.storage::<&mut E, Cons!(&mut A)>();

            e.reserve(ENTITIES);
            a.map_mut().reserve(ENTITIES);
            registry
                .components_mut::<E, B>()
                .map_mut()
                .reserve(ENTITIES);

            for _i in 0..ENTITIES {
                let id = e.insert(E);
                a.insert(id, A(0.0));
            }
        }

        Self(registry)
    }

    pub fn run(&mut self) {
        let cons!(e, mut b) = self.0.storage::<&E, Cons!(&mut B)>();

        for (id, _) in e.iter() {
            b.insert(*id, B(0.0));
        }

        for (id, _) in e.iter() {
            b.remove(id);
        }
    }
}
