use std::borrow::Borrow;
use std::iter::FromIterator;
use std::cmp::Ordering;

pub type EdgeId = u64;
pub type VertexId = u64;
pub type IntervalId = usize;

pub type ShardId = usize;

#[derive(Clone, PartialOrd, PartialEq)]
pub struct Vertex {
    pub id: u64,
    in_edges: Vec<EdgeId>,
    out_edges: Vec<EdgeId>
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct Edge<T: Sized> {
    pub id: EdgeId,
    src: VertexId,
    dest: VertexId,
    weight: T
}

impl <T: Sized> Edge<T> {
    pub fn new(id: EdgeId, src: VertexId, dest: VertexId, weight: T) -> Edge<T> {
        Edge {
            id,
            src,
            dest,
            weight
        }
    }

    pub fn get_id(&self) -> &u64 {
        self.id.borrow()
    }
}

impl Ord for Edge<f64> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.partial_cmp(other.weight.borrow()).unwrap()
    }
}

impl Eq for Edge<f64> {}

impl <T: Sized + 'static> FromIterator<&'static Edge<T>> for Vec<Edge<T>> {
    fn from_iter<U: IntoIterator<Item=&'static Edge<T>>>(iter: U) -> Self {
        unimplemented!()
    }
}

/// src -> edge -> dest
/// multi-relation
#[derive(Clone)]
pub struct AdjacentShard(pub Vertex, pub Vec<EdgeId>, pub Vertex);

#[derive(Clone)]
pub struct EdgeDataShard(pub Vec<Edge<f64>>);

impl EdgeDataShard {
    pub fn find_by_edge_id(
        &mut self,
        edge_id: &u64
    ) -> Option<&Edge<f64>> {
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
        self.0.sort_by_key(|x| x.id.clone())
    }

    pub fn sort_by_src_id(&mut self) {
        self.0.sort_by_key(|x| x.src.clone())
    }
}

