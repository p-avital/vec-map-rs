#![feature(test)]
extern crate test;

use std::iter::FromIterator;

/// A std::vec::Vec based Map, motivated by the fact that, for some key types,
/// iterating over a vector can be faster than other methods for small maps.
///
/// Most of the operations on this map implementation work in O(n), including
/// some of the ones that are O(1) in HashMap. However, optimizers can work magic with
/// contiguous arrays like Vec, and so for small sets (up to 256 elements for integer keys,
/// for example), iterating through a vector actually yields better performance than the
/// less branch- and cache-predictable hash maps.
///
/// To keep item removal fast, this container doesn't form guaranties on item ordering,
/// nor on the stability of the ordering.
///
/// The good news about that is that you're free to mutate keys if your use-case requires that,
/// though I wouldn't recommend it: the underlying vector can be accessed through the unsafe part
/// of the API, in hopes to discourage you from using it.
///
/// Checking equality between maps is defined as "both maps are the same set", and performs worst
/// for maps that are permutations of each other.
#[derive(Clone)]
pub struct VecMap<K: PartialEq, V> {
    /// This member is left visible to allow for un-boxed iteration
    inner: Vec<(K, V)>,
}

impl<K: PartialEq + std::fmt::Debug, V: std::fmt::Debug> std::fmt::Debug for VecMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_map().entries(self.inner.iter().map(|e|(&e.0, &e.1))).finish()
    }
}

impl<K: PartialEq, V> Default for VecMap<K, V> {
    fn default() -> Self {
        VecMap { inner: Vec::new() }
    }
}

impl<K: PartialEq, V: PartialEq> PartialEq for VecMap<K, V> {
    fn eq(&self, rhs: &Self) -> bool {
        // O(1) equality rejection if both sets aren't equally sized
        if self.inner.len() != rhs.inner.len() {
            return false;
        }
        // O(n) equality acceptance, with short-circuiting rejection
        // if both vectors aren't exactly identical
        if self
            .inner
            .iter()
            .zip(rhs.inner.iter())
            .all(|(lhs, rhs)| lhs == rhs)
        {
            return true;
        }
        // O(n^2) equality assertion, with short-circuiting rejection
        // if a key is completely missing or has different associated values
        self.inner.iter().all(|left| {
            if let Some(right) = rhs.get(&left.0) {
                right == &left.1
            } else {
                false
            }
        })
    }
}

impl<K: PartialEq, V> From<Vec<(K, V)>> for VecMap<K, V> {
    fn from(inner: Vec<(K, V)>) -> Self {
        VecMap { inner }
    }
}

impl<K: PartialEq, V> Into<Vec<(K, V)>> for VecMap<K, V> {
    fn into(self) -> Vec<(K, V)> {
        self.inner
    }
}

impl<K: PartialEq, V> VecMap<K, V> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_capacity(capacity: usize)->Self {
        VecMap {inner: Vec::with_capacity(capacity)}
    }
    pub unsafe fn inner(&self) -> &Vec<(K, V)> {
        &self.inner
    }
    pub unsafe fn inner_mut(&mut self) -> &mut Vec<(K, V)> {
        &mut self.inner
    }
    pub fn insert(&mut self, key: K, mut value: V) -> Option<V> {
        if let Some(slot) = self.get_mut(&key) {
            std::mem::swap(&mut value, slot);
            Some(value)
        } else {
            self.inner.push((key, value));
            None
        }
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        self.inner.iter().find(|e| &e.0 == key).map(|e| &e.1)
    }
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.inner
            .iter_mut()
            .find(|e| &e.0 == key)
            .map(|e| &mut e.1)
    }
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.inner
            .iter()
            .position(|e| &e.0 == key)
            .map(|position| self.inner.swap_remove(position).1)
    }
    pub fn keys<'l>(&'l self) -> Box<dyn Iterator<Item = &'l K> + 'l> {
        Box::new(self.inner.iter().map(|e| &e.0))
    }
    pub fn iter<'l>(&'l self) -> Box<dyn Iterator<Item = (&'l K, &'l V)> + 'l> {
        Box::new(self.inner.iter().map(|e| (&e.0, &e.1)))
    }
    pub fn iter_mut<'l>(&'l mut self) -> Box<dyn Iterator<Item = (&'l K, &'l mut V)> + 'l> {
        Box::new(self.inner.iter_mut().map(|e| (&e.0, &mut e.1)))
    }
}

impl<K: PartialEq, V> IntoIterator for VecMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'l, K: PartialEq, V> IntoIterator for &'l VecMap<K, V> {
    type Item = (&'l K, &'l V);
    type IntoIter = Box<dyn Iterator<Item = (&'l K, &'l V)> + 'l>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'l, K: PartialEq, V> IntoIterator for &'l mut VecMap<K, V> {
    type Item = (&'l K, &'l mut V);
    type IntoIter = Box<dyn Iterator<Item = (&'l K, &'l mut V)> + 'l>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K: PartialEq, V> FromIterator<(K, V)> for VecMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut this = Self::new();
        for (key, value) in iter {
            this.insert(key, value);
        }
        this
    }
}

#[cfg(test)]
mod bench {
    mod hash_map {
        use crate::test::Bencher;
        type Map<K, V> = std::collections::HashMap<K, V>;
        fn insertion(b: &mut Bencher, size: usize) {
            b.iter(|| {
                let mut map = Map::new();
                for (key, value) in (0..size).map(|x| (x, x + 2)) {
                    map.insert(key, value);
                }
            });
        }
        fn colliding_insertion(b: &mut Bencher, size: usize) {
            b.iter(|| {
                let mut map = Map::new();
                for (key, value) in (0..size).map(|x| (x % (size / 4), x + 2)) {
                    map.insert(key, value);
                }
            });
        }
        #[bench]
        fn insertion_16(b: &mut Bencher) {
            insertion(b, 16)
        }
        #[bench]
        fn insertion_128(b: &mut Bencher) {
            insertion(b, 128)
        }
        #[bench]
        fn insertion_256(b: &mut Bencher) {
            insertion(b, 256)
        }
        #[bench]
        fn insertion_1024(b: &mut Bencher) {
            insertion(b, 1024)
        }
        #[bench]
        fn colliding_insertion_16(b: &mut Bencher) {
            colliding_insertion(b, 16)
        }
        #[bench]
        fn colliding_insertion_128(b: &mut Bencher) {
            colliding_insertion(b, 128)
        }
        #[bench]
        fn colliding_insertion_1024(b: &mut Bencher) {
            colliding_insertion(b, 1024)
        }
        #[bench]
        fn colliding_insertion_16000(b: &mut Bencher) {
            colliding_insertion(b, 16000)
        }
    }
    mod vec_map {
        use crate::test::Bencher;
        type Map<K, V> = crate::VecMap<K, V>;
        fn insertion(b: &mut Bencher, size: usize) {
            b.iter(|| {
                let mut map = Map::new();
                for (key, value) in (0..size).map(|x| (x, x + 2)) {
                    map.insert(key, value);
                }
            });
        }
        fn colliding_insertion(b: &mut Bencher, size: usize) {
            b.iter(|| {
                let mut map = Map::new();
                for (key, value) in (0..size).map(|x| (x % (size / 4), x + 2)) {
                    map.insert(key, value);
                }
            });
        }
        #[bench]
        fn insertion_16(b: &mut Bencher) {
            insertion(b, 16)
        }
        #[bench]
        fn insertion_128(b: &mut Bencher) {
            insertion(b, 128)
        }
        #[bench]
        fn insertion_256(b: &mut Bencher) {
            insertion(b, 256)
        }
        #[bench]
        fn insertion_1024(b: &mut Bencher) {
            insertion(b, 1024)
        }
        #[bench]
        fn colliding_insertion_16(b: &mut Bencher) {
            colliding_insertion(b, 16)
        }
        #[bench]
        fn colliding_insertion_128(b: &mut Bencher) {
            colliding_insertion(b, 128)
        }
        #[bench]
        fn colliding_insertion_1024(b: &mut Bencher) {
            colliding_insertion(b, 1024)
        }
        #[bench]
        fn colliding_insertion_16000(b: &mut Bencher) {
            colliding_insertion(b, 16000)
        }
    }
}