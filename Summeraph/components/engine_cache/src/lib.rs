use crate::components::interfaces::Components;
use std::hash::Hash;

pub mod components;
pub mod index;

pub struct CacheEngine<V, T>
    where V: Eq + PartialEq + Hash + Components,
          T: Eq +PartialEq + PartialOrd {

}

pub struct IndexEngine {}
