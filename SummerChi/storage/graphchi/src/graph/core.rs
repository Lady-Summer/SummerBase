use std::borrow::Borrow;

pub type EdgeId = u64;
pub type VertexId = u64;
pub type IntervalId = usize;

pub type ShardId = usize;

#[derive(Clone, PartialOrd, PartialEq)]
pub struct Vertex {
    id: u64,
    in_edges: Vec<EdgeId>,
    out_edges: Vec<EdgeId>
}

impl Vertex {
    pub fn get_id(&self) -> &u64 {
        self.id.borrow()
    }
}

#[derive(PartialOrd, PartialEq)]
pub struct Edge<'a, T:?Sized> {
    id: &'a EdgeId,
    src: VertexId,
    dest: VertexId,
    weight: T
}

impl <'a, T:?Sized> Edge<'a, T> {
    pub fn new(id: &EdgeId, src: &VertexId, dest: &VertexId, weight: &T) -> &'a Edge<'a, T> {
        &Edge {
            id,
            src: src.clone(),
            dest: dest.clone(),
            weight: weight.clone()
        }
    }

    pub fn get_id(&self) -> &u64 {
        self.id
    }
}

/// src -> edge -> dest
/// multi-relation
pub struct AdjacentShard<'a>(pub Vertex, pub Vec<&'a EdgeId>, pub Vertex);

#[derive(PartialEq)]
pub struct EdgeDataShard<'a>(pub Vec<&'a Edge<'a, f64>>);

impl <'a> EdgeDataShard<'a> {
    pub fn find_by_edge_id(
        &mut self,
        edge_id: &u64
    ) -> Option<&Edge<'a, f64>> {
        match self.0
            .binary_search_by_key(
                edge_id,
                |x| x.id.clone()
            ) {
            Ok(index) => self.0.get(index),
            Err(e) => None
        }
    }

    pub fn sort_by_edge_id(&mut self) {
        self.0.sort_by_key(|x| x.id)
    }
}

mod engine_core {

    pub struct GraphEngine {
        intervals: Vec<usize>,
    }

    impl GraphEngine {
        pub fn new(partition_num: usize) -> Self {
            GraphEngine {
                intervals: Vec::with_capacity(partition_num)
            }
        }
        //
        // pub fn start(graph_id: &u32) {
        //
        // }
        //
        // pub fn parallel_slide_window<F, T>(&self, f: F) -> Result<T>
        //     where F: Fn(Vertex) -> T {
        //
        // }
    }
}
