pub mod components;
pub mod rocks_engine;
pub mod prelude {

    use std::io::Result;
    use std::any::{TypeId, Any};

    pub trait Vertex {

        fn uuid(&self) -> Result<u64>;

        fn get(&self, key: &String) -> Result<Option<dyn Type>>;

        fn put(&self, key: String, value: &dyn Type) -> Result<()>;

        fn delete(&self, key: &String) -> Result<()>;

        fn out_edges(&self) -> Result<Vec<dyn Edge>>;

        fn in_edges(&self) -> Result<Vec<dyn Edge>>;

        fn schema(&self) -> u64;

    }

    pub trait Edge {
        fn uuid(&self) -> u64;

        fn label(&self) -> Result<String>;

        fn weight(&self) -> Result<f64>;

        fn set_weight(&self, w: f64) -> Result<()>;

        fn set_label(&self, label: String) -> Result<()>;
    }

    pub trait Type {
        fn type_id() -> TypeId;
    }

}
