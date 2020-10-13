pub mod psw;

mod engine_core {

    pub struct GraphChiEngine {
        intervals: Vec<usize>,
    }

    impl GraphChiEngine {
        pub fn new(partition_num: usize) -> Self {
            GraphChiEngine {
                intervals: Vec::with_capacity(partition_num)
            }
        }
    }
}
