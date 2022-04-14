extern crate gen_nested_iter_yield;

#[cfg(test)]
mod tests {
    use futures::StreamExt;
    use gen_nested_iter_yield::nested_iter_yield;

    #[tokio::test]
    #[cfg_attr(not(feature = "futures03"), ignore)]
    async fn vector_deref() {
        let input_vector: Vec<i32> = vec![1, 2, 3];
        let expected_output: Vec<Vec<i32>> = vec![
            vec![1, 1],
            vec![1, 2],
            vec![1, 3],
            vec![2, 1],
            vec![2, 2],
            vec![2, 3],
            vec![3, 1],
            vec![3, 2],
            vec![3, 3],
        ];
        assert_eq!(
            nested_iter_yield!(input_vector.iter(), 2, *)
                .collect::<Vec<_>>()
                .await,
            expected_output
        );
    }

    #[tokio::test]
    #[cfg_attr(not(feature = "futures03"), ignore)]
    async fn vector_clone() {
        let input_vector: Vec<i32> = vec![1, 2, 3];
        let expected_output: Vec<Vec<i32>> = vec![
            vec![1, 1],
            vec![1, 2],
            vec![1, 3],
            vec![2, 1],
            vec![2, 2],
            vec![2, 3],
            vec![3, 1],
            vec![3, 2],
            vec![3, 3],
        ];
        assert_eq!(
            nested_iter_yield!(input_vector.iter(), 2, .clone())
                .collect::<Vec<_>>()
                .await,
            expected_output
        );
    }
}
