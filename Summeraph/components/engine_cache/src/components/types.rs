use std::collections::HashMap;
use std::any::Any;
use std::intrinsics::type_id;

pub type EdgeId = u64;
pub type Delta = u64;

pub enum VertexState {
    ACTIVE, INACTIVE
}

pub enum Hotpot {
    YES, NO
}


pub struct Metadata {
    pub id: u32,
    pub uuid: u64,
    pub properties: HashMap<String, String>,
    pub partition_id: u16
}

pub trait Types {
}

pub struct BaseIter<'a, V> {
}

macro_rules! id_generator {
    () => {};
}

macro_rules! create_uuid {
    ($id:ident, $partition_id:ident) => {

    };
}

