use std::collections::{HashMap, BinaryHeap};
use std::borrow::{Borrow, BorrowMut};
use crate::graph as Graph;
use Graph::core::*;
use std::ops::Deref;
use std::cmp::Ordering;
use std::task::{Poll, Context, Waker};
use std::sync::Arc;
use crate::graph::storage::storage_core::GraphChiStorage;

type InEdge= Edge<f64>;
type DestVertex = Vertex;
type IntervalId = usize;
type IntervalDisk = (Vec<AdjacentShard>, EdgeDataShard);

#[derive(Clone)]
pub struct Shard<'a> {

    pub id: &'a usize,

    pub edges: BinaryHeap<InEdge>
}

impl <'a> PartialOrd for Shard<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(other.id)
    }
}

impl <'a> PartialEq for Shard<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone)]
pub struct Interval<'a> {

    pub shard: Arc<Shard<'a>>,

    pub vertices: Vec<DestVertex>,

    pub id: &'a IntervalId,

    s: GraphChiStorage
}

impl <'a> PartialEq for Interval<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl <'a> PartialOrd for Interval<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(other.id)
    }
}

impl <'a> Interval<'a> {

    pub fn new<'b> (storage: S, shard: Shard<'b>) -> Interval<'a> {
        Interval {
            shard: Arc::new(shard),
            vertices: vec![],
            id: 0.borrow(),
            s: storage
        }
    }

    pub fn load_interval_from_disk(
        interval_id: &'a IntervalId,
        storage: GraphChiStorage,
    ) -> Interval<'a> {
        storage.borrow()
            .get_interval(interval_id)
            .await
            .map (
            |x|
                Self::transform_into_interval(interval_id, storage, x)
            ).expect("load interval failed")
    }

    fn transform_into_interval (
        interval_id: &'a usize,
        storage: GraphChiStorage,
        interval: IntervalDisk
    ) -> Interval<'a> {
        let ref mut edge_data_shard = interval.1;
        let ref mut vertices: Vec<DestVertex> = vec![];
        let mut shard = Shard {
            id: interval_id,
            edges: BinaryHeap::<InEdge>::new()
        };
        interval.0.iter().for_each(
            |adj_shard|
                Self::add_vertex(vertices, adj_shard);
        );
        shard.edges.borrow().append(Self::extract_in_edges(edge_data_shard).borrow_mut());
        /// Sort vertices in order with id
        vertices.sort_by_key(|x| x.id.clone());
        Interval {
            shard: Arc::new(shard),
            vertices: vertices.to_vec(),
            id: interval_id,
            s: storage
        }
    }

    fn extract_in_edges(edge_shard: &mut EdgeDataShard) -> BinaryHeap<InEdge> {
        Self::get_shard_vertices_inedges(edge_shard)
    }

    fn add_vertex(vertices: &mut Vec<Vertex>, adj_shard: &AdjacentShard) {
        vertices.push(adj_shard.0.clone());
        vertices.push(adj_shard.2.clone());
    }

    fn get_shard_vertices_inedges(edge_shard: &mut EdgeDataShard) -> BinaryHeap<InEdge> {
        let mut edges = BinaryHeap::<InEdge>::new();
        edge_shard.sort_by_src_id();
        edge_shard.0.iter().for_each(|x| edges.push(x.clone()));
        edges
    }
}

