use std::collections::{HashMap, HashSet};
use crate::components::types::{Metadata, Delta, EdgeId, VertexState, Hotpot, Types};
use std::borrow::Borrow;
use crate::components::interfaces::Components;
use actix::prelude::*;
use std::task::Context;
use log::info;



#[derive(Eq, PartialEq, Hash, Clone, Components)]
pub struct Vertex {
    metadata: Metadata,
    properties: HashMap<String, String>,
    adj_list: HashSet<(Delta, EdgeId)>,
    state: VertexState,
    compressed_adj: Vec<u8>,
    pub labels: Vec<String>
}


impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.metadata.id == other.metadata.id &&
            (self.metadata.labels == other.metadata.labels)
            && self.metadata.partition_id == other.metadata.partition_id
            && self.is_metadata_properties_eq(other)
    }
}


impl Components for Vertex {
    fn new() -> Vertex {
        Vertex {
            metadata: Metadata {},
            properties: Default::default(),
            adj_list: Default::default(),
            state: VertexState::INACTIVE,
            compressed_adj: vec![],
            labels: vec![]
        }
    }
}

impl Vertex {
    fn is_metadata_properties_eq(&self, other: &Self) -> bool {
        self.metadata.properties.keys()
            .map(|x| self.metadata.properties.get(x) == other.metadata.properties.get(x))
            .fold(true, |s, &a| s & a)
    }


    pub fn change_state(&mut self) {
        if self.state == VertexState::ACTIVE { self.state = VertexState::INACTIVE }
        else { self.state = VertexState::ACTIVE }
    }

    pub fn is_terminate(&self) -> bool {
        self.state == VertexState::INACTIVE
    }

    pub fn get_property(&self, key: &String) -> Option<&String> {
        match self.properties.get(key) {
            Some(value) => Some(value),
            None => {}
        }
    }

    fn get_name(&self) -> String {
    }

}

impl Actor for Vertex {
    type Context = ();


    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        unimplemented!()
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        unimplemented!()
    }

    fn start(self) -> Addr<Self> where
        Self: Actor<Context=Context>, {
        unimplemented!()
    }

    fn create<F>(f: F) -> Addr<Self> where
        Self: Actor<Context=Context<Self>>,
        F: FnOnce(&mut Context) -> Self {
        unimplemented!()
    }
}

#[derive(Iterator, Components)]
pub struct VertexPool {
    vertex_map: HashMap<u64, Vertex>,
}


impl Components for VertexPool {
    fn new() -> VertexPool {
        VertexPool {
            vertex_map: Default::default(),
        }
    }
}

impl VertexPool {
     pub fn set(&mut self, mut v: Vertex) {
         match self.vertex_map.insert(v.get_id(), v) {
             _ => {}
         }
     }
    pub fn remove(&mut self, id: &u64) {
        match self.vertex_map.remove(id) {
            _ => {}
        }
    }

    pub fn get(&self, id: &u64) -> Option<&Vertex> {
        self.vertex_map.get(id)
    }
}


