use crate::interfaces::{Vertex, Edge};
use std::error::Error;
use std::io::Result;

pub mod engine_chi;

pub type ShardId = usize;
pub type VertexId = u64;
pub type EdgeId = usize;

pub struct AdjacentShard(pub dyn Vertex, pub Vec<EdgeId>, pub VertexId);

pub struct EdgeDataShard(pub Vec<dyn Edge>);

pub trait Storage {

    fn get_interval(&self, interval_id: &usize) -> Result<(Vec<AdjacentShard>, EdgeDataShard)>;

    fn get_interval_num(&self) -> Result<usize>;

    fn update_interval(&self, adj_shard: AdjacentShard, edge_shard: EdgeDataShard) -> Result<()>;
}

macro_rules! interval {
    () => {};
}
