pub mod interfaces;
pub mod error;
pub mod graph;
pub mod utils;

pub mod types;

pub struct Schema {
    fields: Vec<(String, String)>
}
