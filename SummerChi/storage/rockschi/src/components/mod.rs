use std::collections::HashMap;
use std::any::Any;
use std::sync::Arc;
use super::rocks_engine::RocksEngine;

pub mod vertex;

type SchemaId = u64;
type VertexId = u64;
type EdgeId = u64;

#[derive(Clone)]
pub struct RocksVertex {
    id: VertexId,
    schema: SchemaId,
    engine: &'static RocksEngine
}

#[derive(Clone)]
pub struct RocksEdge {
    id: EdgeId,
    label: String,
    weight: f64,
    engine: &'static RocksEngine
}


