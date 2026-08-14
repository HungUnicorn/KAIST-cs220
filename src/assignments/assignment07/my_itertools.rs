//! Implement your own minimal `itertools` crate.

use std::collections::HashSet;
use std::hash::Hash;

/// Iterator that iterates over the given iterator and returns only unique elements.
#[derive(Debug)]
pub struct Unique<I: Iterator> {
    iter: I,
    seen: HashSet<I::Item>,
}

impl<I: Iterator> Iterator for Unique<I>
where
    I::Item: Eq + Hash + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            if self.seen.insert(item.clone()) {
                return Some(item);
            }
        }
        None
    }
}

/// Iterator that chains two iterators together.
#[derive(Debug)]
pub struct Chain<I1: Iterator, I2: Iterator> {
    iter1: I1,
    iter2: I2,
}

impl<I1: Iterator, I2: Iterator<Item = I1::Item>> Iterator for Chain<I1, I2> {
    type Item = I1::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter1.next() {
            Some(item) => Some(item),
            None => self.iter2.next(),
        }
    }
}

/// Iterator that iterates over given iterator and enumerates each element.
#[derive(Debug)]
pub struct Enumerate<I: Iterator> {
    index: usize,
    iter: I,
}

impl<I: Iterator> Iterator for Enumerate<I> {
    type Item = (usize, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.iter.next();

        match current {
            Some(item) => {
                self.index += 1;
                Some((self.index - 1, item))
            }
            None => None,
        }
    }
}

/// Iterator that zips two iterators together.
///
/// If one iterator is longer than the other one, the remaining elements for the longer element
/// should be ignored.
#[derive(Debug)]
pub struct Zip<I1: Iterator, I2: Iterator> {
    iter1: I1,
    iter2: I2,
}

impl<I1: Iterator, I2: Iterator> Iterator for Zip<I1, I2> {
    type Item = (I1::Item, I2::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let pair1 = self.iter1.next();
        let pair2 = self.iter2.next();

        match (pair1, pair2) {
            (Some(pair1), Some(pair2)) => Some((pair1, pair2)),
            _ => None,
        }
    }
}

/// My Itertools trait.
pub trait MyIterTools: Iterator {
    /// Returns an iterator that iterates over the `self` and returns only unique elements.
    fn my_unique(self) -> Unique<Self>
    where
        Self: Sized,
    {
        Unique {
            iter: self,
            seen: HashSet::new(),
        }
    }

    /// Returns an iterator that chains `self` and `other` together.
    fn my_chain<I: Iterator>(self, other: I) -> Chain<Self, I>
    where
        Self: Sized,
    {
        Chain {
            iter1: self,
            iter2: other,
        }
    }

    /// Returns an iterator that iterates over `self` and enumerates each element.
    fn my_enumerate(self) -> Enumerate<Self>
    where
        Self: Sized,
    {
        Enumerate {
            index: 0,
            iter: self,
        }
    }

    /// Returns an iterator that zips `self` and `other` together.
    fn my_zip<I: Iterator>(self, other: I) -> Zip<Self, I>
    where
        Self: Sized,
    {
        Zip {
            iter1: self,
            iter2: other,
        }
    }

    /// Foldleft for `MyIterTools`
    fn my_fold<T, F>(mut self, init: T, mut f: F) -> T
    where
        Self: Sized,
        F: FnMut(Self::Item, T) -> T,
    {
        let mut accumulator = init;
        while let Some(item) = self.next() {
            accumulator = f(item, accumulator);
        }
        accumulator
    }
}

impl<T: ?Sized> MyIterTools for T where T: Iterator {}
