use std::collections::HashMap;
use crate::components::interfaces::Components;
use crate::components::edge::EdgePool;
use crate::components::vertex::{Vertex, VertexPool};
use actix::prelude::*;

pub trait Graph<E, V> {
    fn add_vertex(&mut self, v: &Vertex<V>);
}

#[derive(Graph, Components)]
pub struct DefaultGraph<'a, E, V> {
    partition_id: u32,
    properties: HashMap<String, String>,
    vertex_pool: VertexPool<'a, V>,
    edge_pool: EdgePool<'a, E>,
}

impl <'a, E, V> Actor for DefaultGraph<'a, E, V> {
    type Context = ();

    fn started(&mut self, ctx: &mut Self::Context) {
        unimplemented!()
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        unimplemented!()
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        unimplemented!()
    }

    fn start(self) -> Addr<Self> where
        Self: Actor<Context=Context<Self>>, {
        unimplemented!()
    }

    fn create<F>(f: F) -> Addr<Self> where
        Self: Actor<Context=Context<Self>>,
        F: FnOnce(&mut Context<Self>) -> Self, {
        unimplemented!()
    }
}

