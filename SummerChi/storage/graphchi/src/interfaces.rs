use std::cmp::Ordering;
use crate::types::Type;

pub trait Edge {

    fn get_weight(&self) -> f64;

    fn set_id(&mut self, edge_id: &usize);

    fn edge_id(&self) -> u64;

    fn shard_id(&self) -> usize;

    fn get_src(&self) -> u64;

    fn get_dest(&self) -> u64;

    fn set_src(&mut self, src_id: &u64);
}

pub trait Vertex {

    fn get_value(&self, key: &String) -> dyn Type;

    fn set_value(&self, key: &String, value: &dyn Type);

    fn vertex_id(&self) -> u64;

    fn interval_id(&self) -> usize;

    fn get_schema(&self, name: &String) -> super::Schema;

    fn set_schema(&self, name: &super::Schema);

    fn next_vertices(&self) -> Vec<&'static dyn Vertex>;

    fn add_in_edge(&self, dest_id: &u64, edge: &dyn Edge);
}
//
// impl PartialOrd for dyn Vertex {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         unimplemented!()
//     }
// }
//
// impl PartialEq for dyn Vertex {
//     fn eq(&self, other: &Self) -> bool {
//         self.vertex_id() == other.vertex_id() &&
//             self.interval_id() == other.interval_id()
//     }
// }
//
// impl PartialEq for dyn Edge {
//     fn eq(&self, other: &Self) -> bool {
//         self.edge_id() == other.edge_id() &&
//             self.shard_id() == other.shard_id()
//     }
// }
