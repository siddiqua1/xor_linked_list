#[cfg(test)]
mod tests {
    use xor_linked_list::XorLinkedList;

    fn get_count() -> usize {
        #[cfg(feature = "parallel_sized")]
        {
            3
        }
        #[cfg(not(feature = "parallel_sized"))]
        {
            2
        }
    }

    #[test]
    fn test_new() {
        let _ = XorLinkedList::<i32>::new();
    }

    #[test]
    fn test_pop_back() {
        let mut list = XorLinkedList::<i32>::new();
        assert_eq!(None, list.pop_back());
        list.push_back(1);
        assert_eq!(list.len(), 1);
        assert_eq!(Some(1), list.pop_back());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_pop_back2() {
        let mut list = XorLinkedList::<i32>::new();
        assert_eq!(None, list.pop_back());
        list.push_back(1);
        assert_eq!(Some(1), list.pop_back());
        assert_eq!(None, list.pop_back());
        list.push_back(1);
        list.push_back(2);
        assert_eq!(Some(2), list.pop_back());
        assert_eq!(Some(1), list.pop_back());
        assert_eq!(None, list.pop_back());
    }

    #[test]
    fn test_pop_front() {
        let mut list = XorLinkedList::<i32>::new();
        assert_eq!(None, list.pop_front());
        list.push_back(1);
        assert_eq!(list.len(), 1);
        assert_eq!(Some(1), list.pop_front());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_pop_front2() {
        let mut list = XorLinkedList::<i32>::new();
        assert_eq!(None, list.pop_front());
        list.push_back(1);
        assert_eq!(Some(1), list.pop_front());
        assert_eq!(None, list.pop_front());
        list.push_back(1);
        list.push_back(2);
        assert_eq!(Some(1), list.pop_front());
        assert_eq!(Some(2), list.pop_front());
        assert_eq!(None, list.pop_front());
    }

    #[test]
    fn test_push_back() {
        let mut list = XorLinkedList::<i32>::new();
        list.push_back(1);
    }

    #[test]
    fn test_push_back2() {
        let mut list = XorLinkedList::<i32>::new();
        list.push_back(1);
        list.push_back(2);
    }

    #[test]
    fn test_len() {
        let mut list = XorLinkedList::<i32>::new();
        assert_eq!(0, list.len());
        list.push_back(1);
        assert_eq!(1, list.len());
        list.push_back(2);
        assert_eq!(2, list.len());
    }

    #[test]
    fn test_iter() {
        let mut list = XorLinkedList::<i32>::new();
        list.push_back(1);
        list.push_back(2);
        let mut iter = list.iter();
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_iter_rev() {
        let mut list = XorLinkedList::<i32>::new();
        list.push_back(1);
        list.push_back(2);
        let mut iter = list.iter_rev();
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_iter_mut() {
        let mut list = XorLinkedList::<i32>::new();
        for i in 0..5 {
            list.push_back(i);
        }
        for elem in list.iter_mut() {
            *elem += 10;
        }

        for (i, elem) in list.iter().enumerate() {
            assert_eq!(*elem, i as i32 + 10);
        }

        for elem in list.iter_mut_rev() {
            *elem += 10;
        }

        for (i, elem) in list.iter().enumerate() {
            assert_eq!(*elem, i as i32 + 20);
        }
    }

    #[test]
    fn test_is_empty() {
        let mut list = XorLinkedList::<i32>::new();
        assert!(list.is_empty());
        list.push_back(1);
        assert!(!list.is_empty());
        list.pop_back();
        assert!(list.is_empty());
    }

    #[test]
    fn test_append() {
        let mut list1 = XorLinkedList::<i32>::new();
        list1.push_back(1);
        list1.push_back(2);

        let mut list2 = XorLinkedList::<i32>::new();
        list2.push_back(3);
        list2.push_back(4);

        list1.append(&mut list2);

        let mut iter = list1.iter();
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&3), iter.next());
        assert_eq!(Some(&4), iter.next());
        assert_eq!(None, iter.next());

        assert!(list2.is_empty());
    }

    #[test]
    fn test_push_back_mut() {
        let mut list = XorLinkedList::<i32>::new();
        let end = list.push_back_mut(1);
        assert_eq!(*end, 1);
        *end += 1;
        assert_eq!(*end, 2);

        assert_eq!(Some(2), list.pop_back());
    }

    #[test]
    fn test_size_on_stack() {
        let mut list = XorLinkedList::<i32>::new();
        // Print the size of the list struct itself
        println!(
            "Size of XorLinkedList<i32>: {}",
            std::mem::size_of_val(&list)
        );
        // add elements and check size again
        list.push_back(1);
        println!(
            "Size of XorLinkedList<i32> after adding an element: {}",
            std::mem::size_of_val(&list)
        );

        // add one more element
        list.push_back(2);
        println!(
            "Size of XorLinkedList<i32> after adding another element: {}",
            std::mem::size_of_val(&list)
        );

        assert_eq!(
            std::mem::size_of_val(&list),
            std::mem::size_of::<usize>() * get_count()
        );
    }

    #[test]
    fn test_vec_i32() {
        let mut list = XorLinkedList::<Vec<i32>>::new();
        list.push_back(vec![1, 2, 3]);
        list.push_back(vec![4, 5, 6]);

        let mut iter = list.iter();
        assert_eq!(Some(&vec![1, 2, 3]), iter.next());
        assert_eq!(Some(&vec![4, 5, 6]), iter.next());
        assert_eq!(None, iter.next());

        for vec in list.iter_mut() {
            vec.push(10);
        }
        let mut iter = list.iter();
        assert_eq!(Some(&vec![1, 2, 3, 10]), iter.next());
        assert_eq!(Some(&vec![4, 5, 6, 10]), iter.next());
        assert_eq!(None, iter.next());

        assert_eq!(
            std::mem::size_of_val(&list),
            std::mem::size_of::<usize>() * get_count()
        );

        let mut list2 = XorLinkedList::<Vec<i32>>::new();
        list2.push_back(vec![7, 8, 9]);
        list.append(&mut list2);
        let mut iter = list.iter();
        assert_eq!(Some(&vec![1, 2, 3, 10]), iter.next());
        assert_eq!(Some(&vec![4, 5, 6, 10]), iter.next());
        assert_eq!(Some(&vec![7, 8, 9]), iter.next());
        assert_eq!(None, iter.next());

        assert_eq!(
            std::mem::size_of_val(&list),
            std::mem::size_of::<usize>() * get_count()
        );
        assert_eq!(
            std::mem::size_of_val(&list2),
            std::mem::size_of::<usize>() * get_count()
        );
    }

    #[test]
    fn test_array_large() {
        let mut list = XorLinkedList::<[u8; 1024]>::new();
        list.push_back([0u8; 1024]);
        list.push_back([1u8; 1024]);

        let mut iter = list.iter();
        assert_eq!(&[0u8; 1024], iter.next().unwrap());
        assert_eq!(&[1u8; 1024], iter.next().unwrap());
        assert_eq!(None, iter.next());

        for array in list.iter_mut() {
            for byte in array.iter_mut() {
                *byte += 1;
            }
        }
        let mut iter = list.iter();
        assert_eq!(&[1u8; 1024], iter.next().unwrap());
        assert_eq!(&[2u8; 1024], iter.next().unwrap());
        assert_eq!(None, iter.next());

        assert_eq!(
            std::mem::size_of_val(&list),
            std::mem::size_of::<usize>() * get_count()
        );

        let mut list2 = XorLinkedList::<[u8; 1024]>::new();
        list2.push_back([3u8; 1024]);
        list.append(&mut list2);
        let mut iter = list.iter();
        assert_eq!(&[1u8; 1024], iter.next().unwrap());
        assert_eq!(&[2u8; 1024], iter.next().unwrap());
        assert_eq!(&[3u8; 1024], iter.next().unwrap());
        assert_eq!(None, iter.next());

        assert_eq!(
            std::mem::size_of_val(&list),
            std::mem::size_of::<usize>() * get_count()
        );
        assert_eq!(
            std::mem::size_of_val(&list2),
            std::mem::size_of::<usize>() * get_count()
        );
    }
}
