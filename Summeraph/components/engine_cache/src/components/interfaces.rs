use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::{Hasher, Hash};
use crate::components::types::Metadata;
use std::collections::hash_map::DefaultHasher;
use std::time::{SystemTime, UNIX_EPOCH};

pub trait Components {
    fn get_metadata() -> Metadata {
        Self::METADATA
    }
    fn get_id(&mut self) -> u32 {
        Self::get_metadata().id
    }
    fn get_uuid(&mut self) -> u64{
        Self::get_metadata().uuid
    }
    fn add_metadata_property(&mut self, key: String, value: String) -> Option<String> {
        Self::get_metadata().properties.insert(key, value)
    }
    fn get_property(key: String) -> &String {
        match Self::get_metadata().properties.get(key.borrow()) {
            Some(value) => value,
            None => String::from("").borrow()
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
                    match self.add_metadata_property(String::from(key), String::from(value)) {
                        _ => {}
                    }
                None => {}
            }
        });
    }
    const METADATA: Metadata = Metadata {
        id: 0,
        uuid: 0,
        properties: HashMap::new(),
        partition_id: 0
    };
    fn new() -> dyn Components;
}
