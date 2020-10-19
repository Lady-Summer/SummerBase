use std::collections::HashMap;
use std::any::Any;
use std::sync::Arc;

type SchemaId = u64;
type VertexId = u64;
type EdgeId = u64;

pub struct RocksVertex {
    id: VertexId,
    schema: SchemaId,
}

pub struct RocksEdge {
    id: EdgeId,
    label: String,
    weight: f64
}