#![cfg(feature = "derive")]

use muds::{
    collections::{IterableMapMut, MapJoin},
    cons,
    ecs::{Archetypes, Component, Entity, Registry},
    Cons,
};
use munum::{transform, vec3, Mat4, Vec3};

const ENTITIES: usize = 10_000;

#[derive(Entity, Debug)]
struct Node;

#[derive(Component, Debug)]
struct Transform(Mat4<f32>);

#[derive(Component, Debug)]
struct Position(Vec3<f32>);

#[derive(Component, Debug)]
struct Rotation(Vec3<f32>);

#[derive(Component, Debug)]
struct Velocity(Vec3<f32>);

pub struct InsertBenchmark;

impl InsertBenchmark {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&mut self) {
        create_insert_registry();
    }
}

pub struct IterBenchmark(Registry);

impl IterBenchmark {
    pub fn new() -> Self {
        Self(create_insert_registry())
    }

    pub fn run(&mut self) {
        let cons!(_e, mut pos, vel) = self.0.storage::<&Node, Cons!(&mut Position, &Velocity)>();

        for cons!(_id, v, p) in pos.iter_mut().cons().map_join(&*vel) {
            p.0 += v.0;
        }
    }
}

#[inline]
fn create_insert_registry() -> Registry {
    let mut registry = Registry::default();
    registry.register_archetype::<Node, Cons!(Transform, Position, Rotation, Velocity)>();

    {
        let cons!(mut e, mut trans, mut pos, mut rot, mut vel) = registry.storage::<&mut Node, Cons!(&mut Transform, &mut Position, &mut Rotation, &mut Velocity)>();

        e.reserve(ENTITIES);
        trans.map_mut().reserve(ENTITIES);
        pos.map_mut().reserve(ENTITIES);
        rot.map_mut().reserve(ENTITIES);
        vel.map_mut().reserve(ENTITIES);

        for _i in 0..ENTITIES {
            let id = e.insert(Node);
            trans.insert(id, Transform(transform::scaling(vec3(1.0, 1.0, 1.0))));
            pos.insert(id, Position(vec3(1.0, 0.0, 0.0)));
            rot.insert(id, Rotation(vec3(1.0, 0.0, 0.0)));
            vel.insert(id, Velocity(vec3(1.0, 0.0, 0.0)));
        }
    }

    registry
}
