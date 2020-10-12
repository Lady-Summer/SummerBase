pub mod vertex {
    use std::collections::{HashMap, HashSet};
    use super::super as all_components;
    use all_components::types::*;
    use std::borrow::Borrow;
    use all_components::definition::Components;
    use actix::prelude::*;
    use std::task::Context;
    use log::info;
    use std::cmp::Ordering;
    use std::hash::{Hash, Hasher};


    #[derive(Clone)]
    pub struct Vertex {
        metadata: Metadata,
        properties: HashMap<String, String>,
        adj_list: HashSet<(Delta, EdgeId)>,
        state: VertexState,
        compressed_adj: Vec<u8>,
        labels: Vec<String>,
        in_edges: HashSet<EdgeId>
    }

    impl PartialOrd for Vertex {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            unimplemented!()
        }
    }

    impl Hash for Vertex {
        fn hash<H: Hasher>(&self, state: &mut H) {
            unimplemented!()
        }
    }

    impl PartialEq for Vertex {
        fn eq(&self, other: &Self) -> bool {
            self.metadata.id == other.metadata.id
                && self.metadata.partition_id == other.metadata.partition_id
                && self.is_schema_eq(other)
        }
    }


    impl Components for Vertex {
        fn new() -> Vertex {
            Vertex {
                metadata: Metadata {
                    id: 0,
                    uuid: 0,
                    schema: Schema::new(),
                    partition_id: 0
                },
                properties: Default::default(),
                adj_list: Default::default(),
                state: VertexState::INACTIVE,
                compressed_adj: vec![],
                labels: vec![],
                in_edges: HashSet::new()
            }
        }
    }

    impl Vertex {
        fn is_schema_eq(&self, other: &Self) -> bool {
            self.metadata.schema.keys()
                .map(|x|
                    self.metadata.schema.get(x) == other.metadata.schema.get(x)
                )
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

        fn get_labels(&self) -> &[String] {
            self.labels.as_slice()
        }

    }

    impl Actor for Vertex {
        type Context = ();

        fn start(self) -> Addr<Self> where
            Self: Actor<Context=Context>, {
            unimplemented!()
        }

        fn create<F>(f: F) -> Addr<Self> where
            Self: Actor<Context=Context>,
            F: FnOnce(&mut Context) -> Self {
            unimplemented!()
        }
    }
}

pub mod edge {
    use super::super as all_components;
    use all_components::types::EdgeId;
    use all_components::definition::Components;
    use std::collections::{HashMap, HashSet};
    use std::collections::hash_map::Iter;
    use actix::prelude::Message;
    use std::hash::{Hash, Hasher};
    use all_components::types::Metadata;
    use crate::components::types::Schema;
    use std::borrow::Borrow;

    #[derive(Clone)]
    pub struct Edge {
        src: u64,
        dest: u64,
        weight: f64,
        directed: bool,
        metadata: Metadata
    }

    impl PartialEq for Edge {
        fn eq(&self, other: &Self) -> bool {
            self.src.clone() == other.src.clone()
                && self.dest.clone() == other.dest.clone()
                && self.weight.clone() == other.weight.clone()
                && self.directed.clone() == other.directed.clone()
        }
    }

    impl Eq for Edge {}

    impl Hash for Edge {
        fn hash<H: Hasher>(&self, state: &mut H) {
            unimplemented!()
        }
    }

    impl Components for Edge {
        fn new() -> Edge {
            Edge {
                src: 0,
                dest: 0,
                weight: 1.0,
                directed: false,
                metadata: Metadata {
                    id: 0,
                    uuid: 0,
                    schema: Schema::new(),
                    partition_id: 0
                }
            }
        }
    }
}

pub mod pool {
    use std::collections::HashSet;
    use super::super as all_components;
    use all_components::definition::Components;
    use all_components::core::vertex::Vertex;
    use all_components::core::edge::Edge;

    #[derive(Clone)]
    pub struct VertexPool<'a> {
        vertex_map: HashSet<&'a Vertex>,
    }


    impl <'a> Components for VertexPool<'a> {
        fn new() -> VertexPool {
            VertexPool {
                vertex_map: Default::default(),
            }
        }
    }

    impl <'a> VertexPool<'a> {
        pub fn set(&mut self, v: Vertex) {

        }
        pub fn remove(&mut self, id: &u64) {
            match self.vertex_map.remove(id) {
                _ => {}
            }
        }

        pub fn get(&self, id: &u64) -> Option<&'a Vertex> {
            self.vertex_map.get(id).copied()
        }

        pub fn with_capacity(capacity: usize) -> VertexPool {
            VertexPool {
                vertex_map: HashSet::with_capacity(capacity)
            }
        }
    }

    #[derive(Clone)]
    pub struct EdgePool<'a> {
        edge_set: HashSet<&'a Edge>,
    }

    impl <'a> EdgePool<'a> {
        fn add(&mut self, e: Edge) {
            match self.edge_set.insert(&e) {
                _ => {}
            }
        }
        pub fn get(self, e: EdgeId) -> Option<Edge> {
            self.edge_set[e]
        }

        pub fn with_capacity(capacity: usize) -> EdgePool {
            EdgePool {
                edge_set: HashSet::with_capacity(capacity)
            }
        }
    }

    impl<'a> Components for EdgePool<'a> {
        fn new() -> EdgePool<'a> {
            EdgePool {
                edge_set: Default::default()
            }
        }
    }
}
