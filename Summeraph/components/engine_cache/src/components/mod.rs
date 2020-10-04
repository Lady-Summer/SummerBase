pub mod graph;
pub mod vertex;
pub mod types;
pub mod interfaces;
pub mod edge;

macro_rules! vertexpool {
    () => {};
}

macro_rules! vertex {
    () => (
        $vertex::Vertex::new()
    );
}
