use crate::xor_linked_list::{XorLink, XorLinkedList, get_element_at_ptr, get_next_ptr};
use rayon::iter::plumbing::Consumer;
use rayon::iter::plumbing::Producer;
use rayon::iter::plumbing::ProducerCallback;
use rayon::iter::plumbing::UnindexedConsumer;
use rayon::iter::plumbing::bridge;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

pub struct ParXorIter<'a, T>
where
    T: 'a,
{
    prev_start: XorLink,
    start: XorLink,
    end: XorLink,
    prev_end: XorLink,
    length: usize,
    _phantom_data: &'a std::marker::PhantomData<T>,
}

impl<'a, T> ParXorIter<'a, T>
where
    T: 'a,
{
    fn new(list: &XorLinkedList<T>) -> ParXorIter<'a, T> {
        ParXorIter {
            prev_start: 0,
            start: list.begin,
            end: list.end,
            prev_end: 0,
            length: list.len(),
            _phantom_data: &std::marker::PhantomData,
        }
    }
    fn len(&self) -> usize {
        self.length
    }
}

struct XorSeqIter<'a, T> {
    prev_start: XorLink,
    start: XorLink,
    end: XorLink,
    prev_end: XorLink,
    length: usize,
    _phantom_data: &'a std::marker::PhantomData<T>,
}

impl<'a, T> XorSeqIter<'a, T>
where
    T: 'a,
{
    fn new(prod: &XorProducer<T>) -> XorSeqIter<'a, T> {
        XorSeqIter {
            prev_start: prod.prev_start,
            start: prod.start,
            end: prod.end,
            prev_end: prod.prev_end,
            length: prod.length,
            _phantom_data: &std::marker::PhantomData,
        }
    }
}

impl<'a, T> ExactSizeIterator for XorSeqIter<'a, T> {
    fn len(&self) -> usize {
        self.length
    }
}

impl<'a, T> Iterator for XorSeqIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == 0 || self.start == self.prev_end {
            return None;
        }

        let element = get_element_at_ptr::<T>(self.start);

        let next_ptr = get_next_ptr::<T>(self.prev_start, self.start);
        self.prev_start = self.start;
        self.start = next_ptr;

        Some(element)
    }
}

impl<'a, T> DoubleEndedIterator for XorSeqIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end == 0 || self.end == self.prev_start {
            return None;
        }

        let element = get_element_at_ptr::<T>(self.end);

        let next_ptr = get_next_ptr::<T>(self.prev_end, self.end);
        self.prev_end = self.end;
        self.end = next_ptr;

        Some(element)
    }
}

struct XorProducer<'a, T> {
    prev_start: XorLink,
    start: XorLink,
    end: XorLink,
    prev_end: XorLink,
    length: usize,
    _phantom_data: &'a std::marker::PhantomData<T>,
}

impl<'a, T: std::marker::Sync> Producer for XorProducer<'a, T> {
    type Item = &'a T;
    // has to implement Iterator
    // this means we need separate struct from ParXorIter
    // otherwise we cannot disambigious ParallelIterator and Iterator
    type IntoIter = XorSeqIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        XorSeqIter::new(&self)
    }

    fn split_at(self, mid: usize) -> (Self, Self) {
        let length = self.length;
        assert!(mid <= length);
        // first half [0, mid)
        let first_prev_start = self.prev_start;
        let first_start = self.start;

        // we need pointer before and at mid
        let mut prev_ptr = self.prev_start;
        let mut curr_ptr = self.start;
        for _ in 0..mid {
            let next_ptr = get_next_ptr::<T>(prev_ptr, curr_ptr);
            prev_ptr = curr_ptr;
            curr_ptr = next_ptr;
        }
        let prev_mid = prev_ptr;
        let mid_ptr = curr_ptr;

        let first_half = XorProducer {
            prev_start: first_prev_start,
            start: first_start,
            end: prev_mid,
            prev_end: mid_ptr,
            length: mid,
            _phantom_data: self._phantom_data,
        };

        // second half [mid, length)
        let second_end = self.end;
        let second_prev_end = self.prev_end;

        let second_half = XorProducer {
            prev_start: prev_mid,
            start: mid_ptr,
            end: second_end,
            prev_end: second_prev_end,
            length: length - mid,
            _phantom_data: self._phantom_data,
        };

        (first_half, second_half)
    }
}

impl<'a, T> From<ParXorIter<'a, T>> for XorProducer<'a, T> {
    fn from(iterator: ParXorIter<'a, T>) -> Self {
        Self {
            prev_start: iterator.prev_start,
            start: iterator.start,
            end: iterator.end,
            prev_end: iterator.prev_end,
            length: iterator.length,
            _phantom_data: iterator._phantom_data,
        }
    }
}

impl<'a, T: std::marker::Sync> ParallelIterator for ParXorIter<'a, T> {
    type Item = &'a T;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<'a, T: std::marker::Sync> IndexedParallelIterator for ParXorIter<'a, T> {
    fn with_producer<CB: ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
        let producer = XorProducer::from(self);
        callback.callback(producer)
    }

    fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.len()
    }
}

impl<'a, T: std::marker::Sync> IntoParallelIterator for &'a XorLinkedList<T> {
    type Iter = ParXorIter<'a, T>;
    type Item = &'a T;

    fn into_par_iter(self) -> Self::Iter {
        ParXorIter::new(self)
    }
}
