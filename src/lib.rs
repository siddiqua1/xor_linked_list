use std::ptr;

struct XorNode<T> {
    elem: T,
    xor_pointer: usize,
}

impl<T> XorNode<T> {
    pub fn new(element: T) -> Self {
        Self {
            elem: element,
            xor_pointer: 0,
        }
    }
}

fn get_next_ptr(prev_ptr: usize, curr: usize) -> usize {
    let node = unsafe { &*(curr as *const XorNode<()>) };
    let xor_pointer = node.xor_pointer;
    prev_ptr ^ xor_pointer
}

fn get_element_at_ptr<'a, T>(ptr: usize) -> &'a T {
    unsafe {
        let node = &*(ptr as *const XorNode<T>);
        &node.elem
    }
}

fn consume_element_at_ptr<T>(ptr: usize) -> T {
    unsafe {
        let boxed_node = Box::from_raw(ptr as *mut XorNode<T>);
        boxed_node.elem
    }
}

fn get_element_at_ptr_mut<'a, T>(ptr: usize) -> &'a mut T {
    unsafe {
        let node = &mut *(ptr as *mut XorNode<T>);
        &mut node.elem
    }
}

fn point_a_to_b<T>(a: usize, b: usize) {
    assert!(a != 0);
    unsafe {
        let a_node = &mut *(a as *mut XorNode<T>);

        a_node.xor_pointer ^= b;
        if b == 0 {
            return;
        }
        let b_node = &mut *(b as *mut XorNode<T>);
        b_node.xor_pointer ^= a;
    }
}

pub struct XorIter<'a, T>
where
    T: 'a,
{
    prev_ptr: usize,
    curr_ptr: usize,
    _phantom_data: &'a std::marker::PhantomData<T>,
}

impl<T> XorIter<'_, T> {
    fn new(list: &XorLinkedList<T>, reverse: bool) -> XorIter<'_, T> {
        XorIter {
            prev_ptr: 0,
            curr_ptr: if reverse { list.end } else { list.begin },
            _phantom_data: &std::marker::PhantomData,
        }
    }
}

impl<'a, T> std::iter::Iterator for XorIter<'a, T>
where
    T: 'a,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_ptr == 0 {
            return None;
        }

        let element = get_element_at_ptr::<T>(self.curr_ptr);

        let next_ptr = get_next_ptr(self.prev_ptr, self.curr_ptr);
        self.prev_ptr = self.curr_ptr;
        self.curr_ptr = next_ptr;

        Some(element)
    }
}

pub struct XorIterMut<'a, T>
where
    T: 'a,
{
    prev_ptr: usize,
    curr_ptr: usize,
    _phantom_data: &'a std::marker::PhantomData<T>,
}

impl<T> XorIterMut<'_, T> {
    fn new(list: &XorLinkedList<T>, reverse: bool) -> XorIterMut<'_, T> {
        XorIterMut {
            prev_ptr: 0,
            curr_ptr: if reverse { list.end } else { list.begin },
            _phantom_data: &std::marker::PhantomData,
        }
    }
}

impl<'a, T> std::iter::Iterator for XorIterMut<'a, T>
where
    T: 'a,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_ptr == 0 {
            return None;
        }

        let element = get_element_at_ptr_mut::<T>(self.curr_ptr);
        let next_ptr = get_next_ptr(self.prev_ptr, self.curr_ptr);
        self.prev_ptr = self.curr_ptr;
        self.curr_ptr = next_ptr;

        Some(element)
    }
}

type XorLink = usize;

#[derive(Default)]
pub struct XorLinkedList<T> {
    //  allocator?
    begin: XorLink,
    end: XorLink,
    phantom_data: std::marker::PhantomData<T>,
}

impl<T> XorLinkedList<T> {
    pub fn new() -> Self {
        Self {
            begin: 0,
            end: 0,
            phantom_data: std::marker::PhantomData,
        }
    }

    pub fn append(&mut self, other: &mut XorLinkedList<T>) {
        todo!();
    }

    pub fn iter(&self) -> XorIter<'_, T> {
        XorIter::new(self, false)
    }

    pub fn iter_rev(&self) -> XorIter<'_, T> {
        XorIter::new(self, true)
    }

    pub fn iter_mut(&self) -> XorIterMut<'_, T> {
        XorIterMut::new(self, false)
    }

    pub fn iter_mut_rev(&self) -> XorIterMut<'_, T> {
        XorIterMut::new(self, true)
    }

    pub fn is_empty(&self) -> bool {
        self.begin == 0
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    pub fn push_back(&mut self, element: T) {
        let _ = self.push_back_mut(element);
    }

    pub fn push_back_mut(&mut self, element: T) -> &mut T {
        let node = Box::new(XorNode::new(element));
        let ptr = Box::into_raw(node);

        if self.begin == 0 {
            // empty list
            self.begin = ptr as usize;
            self.end = ptr as usize;
            return get_element_at_ptr_mut(ptr as usize);
        }

        if self.end == 0 {
            panic!("Invalid state: begin is set but end is not");
        }
        point_a_to_b::<T>(self.end, ptr as usize);
        self.end = ptr as usize;

        get_element_at_ptr_mut(ptr as usize)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.end == 0 {
            return None;
        }

        if self.begin == 0 {
            panic!("Invalid state: end is set but begin is not");
        }

        if self.begin == self.end {
            // only one element
            let elem = consume_element_at_ptr::<T>(self.end);
            self.begin = 0;
            self.end = 0;
            return Some(elem);
        }

        let old_end = self.end;
        let prev_ptr = get_next_ptr(0, self.end);
        point_a_to_b::<T>(prev_ptr, self.end); // clean current end
        self.end = prev_ptr;

        Some(consume_element_at_ptr::<T>(old_end))
    }
}

impl<T> Drop for XorLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_back().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            std::mem::size_of::<usize>() * 2
        );
    }
}
