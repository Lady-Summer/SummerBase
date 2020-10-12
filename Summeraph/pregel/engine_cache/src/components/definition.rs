use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::{Hasher, Hash};
use super::types::Metadata;
use std::collections::hash_map::DefaultHasher;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::components::types::Schema;
use std::any::TypeId;

pub trait Components: Hash {
    fn get_metadata() -> Metadata {
        Self::METADATA
    }
    fn get_id(&mut self) -> u64 {
        Self::get_metadata().id.clone()
    }
    fn get_uuid(&mut self) -> u64{
        Self::get_metadata().uuid
    }

    fn add_metadata_property(
        &mut self,
        key: String,
        value: String
    ) -> Option<String> {
        Self::get_metadata().schema.insert(key, value)
    }
    fn get_field_type(key: String) -> Option<TypeId> {
        match Self::get_metadata().schema.get(key.borrow()) {
            Some(value) => Some(value.clone()),
            None =>  None
        }
    }

    fn uuid(&mut self) -> u64 {
        // TODO Implement unique hash algorithm
        let hasher = DefaultHasher::new();
        self.hash(&hasher);
        let mut prefix = hasher.finish();
        let mut result = prefix << 32;
        result + SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
    fn set_metadata(
        &mut self,
        id: u64,
        name: String,
        properties: HashMap<String, String>
    ) {
        self.set_id(id);
        self.set_name(name);
        properties.keys().for_each(|key| {
            match properties.get(key) {
                Some(value) =>
                    match self.add_metadata_property(key.clone(), value.clone()) {
                        _ => {}
                    }
                None => {}
            }
        });
    }
    const METADATA: Metadata = Metadata {
        id: 0,
        uuid: 0,
        schema: Schema::new(),
        partition_id: 0
    };
    fn new() -> dyn Components;
}
