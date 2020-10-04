use std::collections::HashMap;
use std::fmt::{Formatter, Debug};

pub enum QueryEventKind {
    Read,
    Update,
    Delete,
    Create
}

#[derive(Debug)]
pub struct Metadata(u64, u64, u32, HashMap<String, String>);

impl Debug for Metadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("metadata")
            .field("uuid", self.0)
            .field("id", self.1)
            .field("partition_id", self.2)
            .field("properties", self.3)
            .finish()
    }
}

#[derive(Debug)]
pub struct ComponentData(String, HashMap<String, String>);

#[derive(Debug)]
pub struct ActorEvent(QueryEventKind, Metadata, ComponentData);