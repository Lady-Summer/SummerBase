pub mod interfaces;
pub mod error;
pub mod storage;
pub mod graph;

pub mod types;

pub struct Schema {
    fields: Vec<(String, String)>
}
