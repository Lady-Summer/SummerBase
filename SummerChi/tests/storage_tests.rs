#[cfg(test)]
mod core_tests {
    use graphchi::graph::core::{EdgeDataShard, Edge};
    use mockall::*;
    use std::borrow::{BorrowMut, Borrow};
    const MOCK_EDGES: Vec<Edge<f64>> = vec![
        Edge::<f64>::new(2, 2, 3, 4.0),
        Edge::<f64>::new(0, 0, 1, 2.0),
        Edge::<f64>::new(4, 5, 2, 8.0),
        Edge::<f64>::new(3, 2, 4, 1.0),
        Edge::<f64>::new(1, 1, 2, 3.0),
    ];

    #[automock]
    impl  EdgeDataShard {
        fn test_sort_by_id(&mut self) -> &EdgeDataShard<'a> {
            self.sort_by_edge_id();
            self
        }

        fn test_find_by_id(&mut self, edge_id: &u64) -> Option<&Edge<f64>> {
            self.find_by_edge_id(edge_id)
        }
    }

    #[test]
    fn test_edge_shard_sort() {
        let mock_edge_shard = MockEdgeDataShard::new();
        let mut mock_edge_sorted = MOCK_EDGES.to_vec();
        mock_edge_shard.expect_test_sort_by_id()
            .returning(
                |x: &EdgeDataShard|
                    &EdgeDataShard(MOCK_EDGES.to_vec())
            );
        mock_edge_sorted.sort_by_key(|x| x.id.borrow());
        assert_eq!(mock_edge_shard, &EdgeDataShard(mock_edge_sorted))
    }

    #[test]
    fn test_find_by_id() {
        let mock_edge_shard = MockEdgeDataShard::new();
        mock_edge_shard.expect_test_find_by_id()
            .withf(|x: &u64| *x == 2)
            .returning(|x: Option<&Edge<f64>>| x);
        let mut mock_edge_sorted = MOCK_EDGES.to_vec();
        mock_edge_sorted.sort_by_key(|x| x.id.borrow());
        assert_eq!(mock_edge_shard, EdgeDataShard(mock_edge_sorted))
    }
}
