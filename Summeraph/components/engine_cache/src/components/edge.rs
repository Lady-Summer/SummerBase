use crate::components::types::{BaseIter, EdgeId};
use crate::components::interfaces::Components;
use std::collections::{HashSet, HashMap};
use std::collections::hash_map::Iter;
use actix::prelude::Message;

#[derive(Eq, PartialEq, Hash, Components)]
pub struct Relation {
    src: u64, dest: u64,
    directed: bool,
}

#[derive(Eq, PartialEq, Hash, Components)]
pub struct Edge {
    pub label: String,
    endpoints: HashSet<Relation>,
    compressed_relations: Vec<u8>,
    compressed_label: Vec<u8>
}

impl Components for Edge {
    fn new() -> Edge {
        Edge {
            label: "".to_string(),
            endpoints: Default::default(),
            compressed_relations: vec![],
            compressed_label: vec![]
        }
    }
}


#[derive(Components)]
pub struct EdgePool<'a> {
    edge_set: HashSet<Edge>,
}

impl <'a> EdgePool<'a> {
    fn add(&mut self, e: Edge) {
        match self.edge_set.insert(e) {
            _ => {}
        }
    }
    pub fn get(self, e: EdgeId) -> Option<Edge> {
        self.edge_set[e]
    }
}

