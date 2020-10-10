pub mod interval;
pub mod core;
pub mod storage;

mod constants {
    pub const ROOT_PATH: String = String::from("/data/summerchi");
    pub const ENTER: u8 = '\n' as u8;
}

pub macro_rules! edge_shard {
    () => {
        $graphchi::graph::core::EdgeDataShard($crate::vec::Vec::new())
    };
}
