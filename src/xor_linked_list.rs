pub(crate) struct XorNode<T> {
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
pub(crate) type XorLink = usize;

pub(crate) fn get_next_ptr<T>(prev_ptr: XorLink, curr: XorLink) -> XorLink {
    let node = unsafe { &*(curr as *const XorNode<T>) };
    let xor_pointer = node.xor_pointer;
    prev_ptr ^ xor_pointer
}

pub(crate) fn get_element_at_ptr<'a, T>(ptr: XorLink) -> &'a T {
    unsafe {
        let node = &*(ptr as *const XorNode<T>);
        &node.elem
    }
}

pub(crate) fn consume_element_at_ptr<T>(ptr: XorLink) -> T {
    unsafe {
        let boxed_node = Box::from_raw(ptr as *mut XorNode<T>);
        boxed_node.elem
    }
}

pub(crate) fn get_element_at_ptr_mut<'a, T>(ptr: XorLink) -> &'a mut T {
    unsafe {
        let node = &mut *(ptr as *mut XorNode<T>);
        &mut node.elem
    }
}

pub(crate) fn point_a_to_b<T>(a: XorLink, b: XorLink) {
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

pub(crate) fn _length_from_a_to_b(a: XorLink, b: XorLink, prev_a: XorLink) -> usize {
    if a == 0 {
        return 0;
    }
    let mut prev_ptr = prev_a;
    let mut curr_ptr = a;
    let mut count = 1;

    while curr_ptr != b {
        let next_ptr = get_next_ptr::<()>(prev_ptr, curr_ptr);
        prev_ptr = curr_ptr;
        curr_ptr = next_ptr;
        count += 1;
    }
    count
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

        let next_ptr = get_next_ptr::<T>(self.prev_ptr, self.curr_ptr);
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
        let next_ptr = get_next_ptr::<T>(self.prev_ptr, self.curr_ptr);
        self.prev_ptr = self.curr_ptr;
        self.curr_ptr = next_ptr;

        Some(element)
    }
}

#[derive(Default)]
pub struct XorLinkedList<T> {
    //  allocator?
    pub(crate) begin: XorLink,
    pub(crate) end: XorLink,
    phantom_data: std::marker::PhantomData<T>,
    #[cfg(feature = "parallel_sized")]
    pub(crate) length: usize,
}

impl<T> XorLinkedList<T> {
    pub fn new() -> Self {
        Self {
            begin: 0,
            end: 0,
            phantom_data: std::marker::PhantomData,
            #[cfg(feature = "parallel_sized")]
            length: 0,
        }
    }

    pub fn append(&mut self, other: &mut XorLinkedList<T>) {
        while let Some(elem) = other.pop_front() {
            self.push_back(elem);
        }
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
        #[cfg(feature = "parallel_sized")]
        {
            self.length
        }
        #[cfg(not(feature = "parallel_sized"))]
        {
            self.iter().count()
        }
    }

    pub fn push_back(&mut self, element: T) {
        let _ = self.push_back_mut(element);
    }

    pub fn push_back_mut(&mut self, element: T) -> &mut T {
        let node = Box::new(XorNode::new(element));
        let ptr = Box::into_raw(node);

        #[cfg(feature = "parallel_sized")]
        {
            self.length += 1;
        }

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

        #[cfg(feature = "parallel_sized")]
        {
            self.length -= 1;
        }

        if self.begin == self.end {
            // only one element
            let elem = consume_element_at_ptr::<T>(self.end);
            self.begin = 0;
            self.end = 0;
            return Some(elem);
        }

        let old_end = self.end;
        let prev_ptr = get_next_ptr::<T>(0, self.end);
        point_a_to_b::<T>(prev_ptr, self.end); // clean current end
        self.end = prev_ptr;

        Some(consume_element_at_ptr::<T>(old_end))
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.begin == 0 {
            return None;
        }

        if self.end == 0 {
            panic!("Invalid state: begin is set but end is not");
        }

        #[cfg(feature = "parallel_sized")]
        {
            self.length -= 1;
        }

        if self.begin == self.end {
            // only one element
            let elem = consume_element_at_ptr::<T>(self.begin);
            self.begin = 0;
            self.end = 0;
            return Some(elem);
        }

        let old_begin = self.begin;
        let next_ptr = get_next_ptr::<T>(0, self.begin);
        point_a_to_b::<T>(next_ptr, self.begin); // clean current begin
        self.begin = next_ptr;

        Some(consume_element_at_ptr::<T>(old_begin))
    }
}

impl<T> Drop for XorLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_back().is_some() {}
    }
}
