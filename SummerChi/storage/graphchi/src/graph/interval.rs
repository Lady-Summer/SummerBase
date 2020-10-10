use std::collections::HashMap;
use std::borrow::{Borrow, BorrowMut};
use crate::graph as Graph;
use Graph::core::*;
use Graph::storage::Storage;
use std::ops::Deref;

type InEdge= Edge<f64>;
type DestVertex = Vertex;
type IntervalId = usize;

#[derive(Clone, PartialOrd, PartialEq)]
pub struct Shard<'a> {

    pub id: &'a usize,

    /// dest -> inEdges
    pub edges: HashMap<VertexId, Vec<InEdge>>
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct Interval<'a, S: Storage> {

    pub shards: Vec<Shard<'a>>,

    pub vertices: Vec<DestVertex>,

    pub id: &'a IntervalId,

    shard_size: usize,

    s: &'a S
}

impl <'a, S: Storage> Interval<'a, S> {

    pub fn new(shards_num: &u64, s: S)
               -> Interval<'a, S> {
        Interval {
            shards: Vec::with_capacity(*shards_num as usize),
            vertices: vec![],
            id: 0.borrow(),
            shard_size: *shards_num as usize,
            s: &s
        }
    }

    pub fn load_interval_from_disk(
        &self,
        interval_id: &'a IntervalId,
        s: S
    ) -> Interval<'a, S> {
        match s.get_interval(interval_id) {
            Ok(interval) => {
                let mut edge_data_shard = interval.1;
                let mut shards = vec![];
                let ref mut vertices: Vec<DestVertex> = vec![];
                interval.0.iter().for_each(
                    move |adj_shard|
                        shards.push(
                            self.transform_shards(
                                interval_id,
                                vertices,
                                adj_shard,
                                edge_data_shard.borrow_mut()
                            )
                        )
                );
                /// Sort vertices in order with id
                vertices.sort_by_key(|x| x.id.clone());
                Interval {
                    shards,
                    vertices: vertices.to_vec(),
                    id: interval_id,
                    shard_size: 0,
                    s: &s
                }
            },
            Err(e) => panic!("Error occurs when load interval from disk: {:?}", e)
        }

    }

    fn transform_shards(
        &self,
        interval_id: &IntervalId,
        vertices: &mut Vec<Vertex>,
        adj_shard: &AdjacentShard,
        edge_shard: &mut EdgeDataShard
    ) -> Shard<'a> {
        vertices.push(adj_shard.0.clone());
        Shard {
            id: interval_id.clone().borrow(),
            edges: self.get_shard_vertices_inedges(adj_shard, edge_shard)
        }
    }

    fn get_shard_vertices_inedges(
        &self,
        adj_shard: &AdjacentShard,
        edge_shard: &mut EdgeDataShard
    ) -> HashMap<u64, Vec<InEdge>> {
        let mut edges: HashMap<VertexId, Vec<InEdge>> = HashMap::new();
        edge_shard.sort_by_edge_id();
        edges.insert(
            adj_shard.2.id.clone(),
            adj_shard.1.iter().map(
                move |x| {
                    /// Insert
                    match edge_shard.find_by_edge_id(x) {
                        Some(edge) => edge.clone(),
                        None => panic!("Invalid edge with id {}", x)
                    }
                }).collect()
        );
        edges
    }
    //
    // fn parallel_udf_executor<F, U, T>(&self, f: F) -> Vec<_>
    // where F: Fn(U) -> T
    //
    // {
    //     self.vertices.iter().map(|v| async {Ok(f(v))?}).collect()
    // }
    //
    // fn update_shard(&mut self) {
    //     match self.shards.get(self.id) {
    //         Some(shard) => {
    //             shard.borrow().update_fully();
    //             for x in 0..self.shard_size {
    //                 if x != self.id {
    //                     self.shards.get(x).unwrap().update_last_window_to_disk();
    //                 }
    //             }
    //         }
    //         None => Err(ShardIndexOutOfBound {})
    //     }
    // }

}
//
// impl <'a, S> PartialEq for Interval<'a, S> where S: Storage {
//     fn eq(&self, other: &Self) -> bool {
//         (self.shards == other.shards.borrow()) &&
//             (self.vertices == other.vertices.borrow()) &&
//             (self.id == other.id)
//     }
// }
