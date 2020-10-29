use super::RocksVertex;
use super::super::prelude::Vertex;
use crate::prelude::Type;
use std::io::Result;

impl Vertex for  RocksVertex {
    fn uuid(&self) -> Result<u64> {
        Ok(self.id)
    }

    fn get(&self, key: &String) -> Result<Option<dyn Type>> {
        self.engine.get(key.clone())
    }

    fn put(&self, key: String, value: &dyn Type) -> Result<()> {
        unimplemented!()
    }

    fn delete(&self, key: &String) -> Result<()> {
        unimplemented!()
    }

    fn out_edges(&self) -> Result<Vec<dyn Edge>> {
        unimplemented!()
    }

    fn in_edges(&self) -> Result<Vec<dyn Edge>> {
        unimplemented!()
    }

    fn schema(&self) -> u64 {
        unimplemented!()
    }
}