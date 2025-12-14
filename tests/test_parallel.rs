#[cfg(all(test, feature = "parallel"))]
mod tests {
    use rayon::prelude::*;
    use xor_linked_list::XorLinkedList;

    #[test]
    fn test_parallel_empty_list() {
        let list: XorLinkedList<i32> = XorLinkedList::new();
        let length: usize = list.into_par_iter().len();
        assert_eq!(length, 0);
    }

    #[test]
    fn test_parallel_iteration() {
        let mut list = XorLinkedList::new();
        for i in 0..1_000 {
            list.push_back(i);
        }

        let sum: i32 = list.par_iter().sum();
        let expected_sum: i32 = (0..1_000).sum();

        assert_eq!(sum, expected_sum);
    }

    #[test]
    fn test_parallel_map() {
        let mut list = XorLinkedList::new();
        for i in 0..100 {
            list.push_back(i);
        }

        let doubled: Vec<i32> = list.par_iter().map(|&x| x * 2).collect();
        let expected: Vec<i32> = (0..100).map(|x| x * 2).collect();

        assert_eq!(doubled, expected);
    }

    #[test]
    fn test_parallel_filter() {
        let mut list = XorLinkedList::new();
        for i in 0..100 {
            list.push_back(i);
        }

        let evens: Vec<&i32> = list.par_iter().filter(|&&x| x % 2 == 0).collect();
        let expected: Vec<i32> = (0..100).filter(|x| x % 2 == 0).collect();

        assert_eq!(evens.len(), expected.len());
        for (i, &&val) in evens.iter().enumerate() {
            assert_eq!(val, expected[i]);
        }
    }

    #[test]
    fn test_parallel_find_any() {
        let mut list = XorLinkedList::new();
        for i in 0..1_000 {
            list.push_back(i);
        }

        let found = list.par_iter().find_any(|&&x| x == 500);
        assert_eq!(found, Some(&500));

        let not_found = list.par_iter().find_any(|&&x| x == 10_000);
        assert_eq!(not_found, None);
    }

    #[test]
    fn test_parallel_max_min() {
        let mut list = XorLinkedList::new();
        for i in 0..100 {
            list.push_back(i);
        }

        let max = list.par_iter().max();
        assert_eq!(max, Some(&99));

        let min = list.par_iter().min();
        assert_eq!(min, Some(&0));
    }

    #[test]
    fn test_parallel_all_any() {
        let mut list = XorLinkedList::new();
        for i in 0..100 {
            list.push_back(i);
        }

        let all_less_than_200 = list.par_iter().all(|&x| x < 200);
        assert!(all_less_than_200);

        let any_equals_50 = list.par_iter().any(|&x| x == 50);
        assert!(any_equals_50);

        let any_equals_1000 = list.par_iter().any(|&x| x == 1000);
        assert!(!any_equals_1000);
    }

    #[test]
    fn test_parallel_fold_reduce() {
        let mut list = XorLinkedList::new();
        for i in 1..=10 {
            list.push_back(i);
        }

        let product = list
            .par_iter()
            .fold(|| 1, |acc, &x| acc * x)
            .reduce(|| 1, |a, b| a * b);
        let expected: i32 = (1..=10).product();

        assert_eq!(product, expected);
    }

    #[test]
    fn test_parallel_large_list() {
        let mut list = XorLinkedList::new();
        for i in 0..10_000 {
            list.push_back(i);
        }

        let sum: i64 = list.par_iter().map(|&x| x as i64).sum();
        let expected_sum: i64 = (0..10_000i64).sum();

        assert_eq!(sum, expected_sum);
    }

    #[test]
    fn test_parallel_single_element() {
        let mut list = XorLinkedList::new();
        list.push_back(42);

        let sum: i32 = list.par_iter().sum();
        assert_eq!(sum, 42);

        let vec: Vec<&i32> = list.par_iter().collect();
        assert_eq!(vec, vec![&42]);
    }
}
