extern crate rand;

use std::clone::Clone;
use self::rand::Rng;

/// A reservoir sample maintains a sample of K elements, selected uniformly and
/// at random from a stream.
///
/// ```
/// use sketchy::ReservoirSample;
///
/// let mut res = ReservoirSample::new(2);
///
/// for v in ["one", "two", "three"].iter() {
///     res.insert(*v);
/// }
///
/// println!("elements: {:?}", res.elements());
/// ```
pub struct ReservoirSample<E> {
    count: usize,
    elements: Vec<E>,
}

impl<E: Copy + Clone> ReservoirSample<E> {
    /// Returns a new `ReservoirSample` of the given size.
    pub fn new(size: usize) -> ReservoirSample<E> {
        ReservoirSample::<E> {
            count: 0,
            elements: Vec::with_capacity(size),
        }
    }

    /// Inserts the given element into the sample.
    pub fn insert(&mut self, e: E) {
        if self.count < self.elements.capacity() {
            self.elements.push(e);
        } else {
            let idx = rand::thread_rng().gen_range(0, self.count);
            if idx < self.elements.capacity() {
                self.elements[idx] = e;
            }
        }
        self.count += 1;
    }

    /// Returns the recorded elements in the sample.
    pub fn elements(self) -> Vec<E> {
        self.elements.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert() {
        let mut sample = ReservoirSample::new(10);

        for i in 0..100i32 {
            sample.insert(i);
        }

        let elements = sample.elements();

        assert_eq!(elements.len(), 10);

        for &i in elements.iter() {
            assert!(i >= 0 && i < 100);
        }
    }
}
