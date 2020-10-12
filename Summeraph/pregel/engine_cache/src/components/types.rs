use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::collections::hash_map::Keys;

pub type EdgeId = u64;
pub type Delta = u64;

#[derive(Clone)]
pub enum VertexState {
    ACTIVE, INACTIVE
}

#[derive(Clone)]
pub enum Hotpot {
    YES, NO
}

#[derive(Clone)]
pub struct Metadata {
    pub id: u64,
    pub uuid: u64,
    pub schema: Schema,
    pub partition_id: u16
}

pub trait Type: From<TypeId> {

    /// Type's id
    fn type_id(&self) -> TypeId;

    /// Type's name
    fn type_name(&self) -> String;
}

#[derive(Clone)]
pub struct Schema {
    value: HashMap<String, &'static TypeId>
}

impl Schema {
    pub fn keys(&self) -> Keys<'_, String, &'static TypeId> {
        self.value.keys()
    }

    pub fn get(&self, key: &String) -> Option<&'static TypeId> {
        self.value.get(key).copied()
    }

    pub fn new() -> Schema {
        Schema {
            value: Default::default()
        }
    }
}

impl PartialEq for Schema {
    fn eq(&self, other: &Self) -> bool {
        self.value.keys()
            .map(|k| self.get(k) == other.get(k))
            .fold(true, |pre, post| pre & post)
    }
}






