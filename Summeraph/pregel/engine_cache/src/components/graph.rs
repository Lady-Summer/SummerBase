use std::collections::HashMap;
use super as all_components;
use all_components::definition::Components;
use all_components::types::Metadata;
use all_components::core::vertex::Vertex;
use all_components::core::pool::*;
use all_components::core::edge::*;
use actix::prelude::*;

pub trait Graph<E, V> {

    fn add_vertex(&mut self, v: &Vertex);

    fn add_edge(&mut self, e: &Edge);

    fn delete_vertex(&mut self, id: &u64);

    fn delete_edge(&mut self, id: &u64);

    fn get_vertex(&self, id: &u64);

}

#[derive(Clone)]
pub struct DefaultGraph<'a, E, V> {
    gid: u32,
    partition_id: u32,
    vertex_pool: VertexPool<'a>,
    edge_pool: EdgePool<'a>,
}

impl <'a, E, V> Components for DefaultGraph<'a, E, V> {
    fn new() -> DefaultGraph<'a, E, V> {
        unimplemented!()
    }
}


