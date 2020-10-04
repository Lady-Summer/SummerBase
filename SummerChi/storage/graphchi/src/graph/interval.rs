use std::collections::{HashSet, HashMap};
use crate::interfaces::*;
use std::error::Error;
use crate::error::ShardIndexOutOfBound;
use std::borrow::Borrow;
use crate::storage::{Storage, AdjacentShard};
use std::ops::Deref;

type InEdge = dyn Edge;
type DestVertex = dyn Vertex;

#[derive(Clone, PartialOrd, PartialEq)]
pub struct Shard<S: Storage> {

    pub id: usize,

    pub edges: HashMap<usize, Vec<InEdge>>
}

impl <S: Storage> Shard<S> {

    pub fn update_fully(&mut self) {
    }

    pub fn update_last_window_to_disk(&mut self) {
    }
}


#[derive(Clone, PartialOrd, PartialEq)]
pub struct Interval<S: Storage> {

    pub shards: Vec<Shard<S>>,

    pub vertices: Vec<DestVertex>,

    pub id: usize,

    shard_size: usize,

    s: S
}

impl <S: Storage> Interval<S> {

    pub fn new(shards_num: &u64, s: S)
               -> Interval<S> {
        Interval {
            shards: Vec::with_capacity(shards_num.to_usize()),
            vertices: vec![],
            id: 0,
            shard_size: shards_num.to_usize(),
            s
        }
    }

    pub fn load_interval_from_disk(interval_id: &usize, s: S) -> Interval<S> {
        match s.get_interval(interval_id) {
            Ok(interval) => {
                let edge_data_shard = interval.1;
                let mut shards = vec![];
                let ref mut vertices: Vec<DestVertex> = vec![];
                interval.0.iter().for_each(move |adj_shard|
                    shards.push(Self::load_shards(interval_id, vertices, adj_shard))
                );
                Interval {
                    shards,
                    vertices: vertices.deref().to_vec(),
                    id: interval_id.clone(),
                    shard_size: 0,
                    s
                }
            },
            Err(e) => panic!("Error occurs when load interval from disk: {:?}", e)
        }

    }

    fn load_shards(interval_id: &usize, vertices: &mut Vec<dyn Vertex>, adj_shard: &AdjacentShard)
        -> Shard<S>
    {
        let mut edges: HashMap<usize, Vec<InEdge>> = HashMap::new();
        vertices.push(adj_shard.0.borrow());
        edges.insert(interval_id.clone(),
                     adj_shard.1.iter().map(move |x| {

                     }).collect());
        Shard {
            id: interval_id.clone(),
            edges
        }
    }

    pub fn load_subgraph(&self) -> Self {
    }

    fn parallel_udf_executor<F, U: Vertex, T>(&self, f: F) -> Vec<_>
    where F: Fn(U) -> T

    {
        self.vertices.iter().map(|v| async {Ok(f(v))?}).collect()
    }

    fn update_shard(&mut self) {
        match self.shards.get(self.id) {
            Some(shard) => {
                shard.borrow().update_fully();
                for x in 0..self.shard_size {
                    if x != self.id {
                        self.shards.get(x).unwrap().update_last_window_to_disk();
                    }
                }
            }
            None => Err(ShardIndexOutOfBound {})
        }
    }

}

impl PartialEq for Interval<S> where S: Storage {
    fn eq(&self, other: &Self) -> bool {
        (self.shards == other.shards.borrow()) &&
            (self.vertices == other.vertices.borrow()) &&
            (self.id == other.id)
    }
}
