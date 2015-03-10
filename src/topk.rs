use std::collections::HashSet;
use std::hash::Hash;
use countmin::CountMinSketch;

/// A Top-K heap is a probabilistic data structure which uses a Count-Min Sketch to calculate the
/// top K elements in a data stream with the highest frequency.
///
/// ```
/// use sketchy::{CountMinSketch, TopK};
///
/// let cms = CountMinSketch::with_confidence(0.0001, 0.99);
/// let mut topk = TopK::new(5, 0.05, cms);
///
/// for i in 1..10000 {
///     topk.insert(i % 1000); // an uncommon item
///     topk.insert(-100); // a common item
/// }
///
/// assert_eq!(topk.elements(), vec![-100]);
/// ```
pub struct TopK<E> {
    k: usize,
    min: f64,
    n: u64,
    cms: CountMinSketch<E>,
    elements: HashSet<E>,
}

impl<E: Eq + Hash + Copy> TopK<E> {
    /// Returns a TopK which will track `k` elements with at least `min` frequency (`(0,1)`) using
    /// the given CountMinSketch.
    pub fn new(k: usize, min: f64, cms: CountMinSketch<E>) -> TopK<E> {
        TopK::<E> {
            k: k,
            min: min,
            n: 0,
            cms: cms,
            elements: HashSet::with_capacity(k),
        }
    }

    /// Adds a value to the heap.
    pub fn insert(&mut self, e: E) {
        self.cms.insert(e);
        self.n += 1;

        if self.is_top(&e) {
            self.elements.insert(e);
        }
    }

    /// Returns a vector of the top K elements, in reverse order of frequency.
    pub fn elements(&mut self) -> Vec<E> {
        let mut v: Vec<E> = self.elements.iter()
            .filter(|e| self.is_top(e))
            .map(|&e| e)
            .collect();
        v.sort_by(|a, b| self.cms.estimate(b).cmp(&self.cms.estimate(a)));
        v.into_iter().take(self.k).collect()
    }

    /// Shrinks the heap as much as possible while still retaining the top K elements. Should be
    /// called periodically to filter out false positives as the data stream changes.
    pub fn shrink_to_fit(&mut self) {
        if self.elements.len() > self.k {
            self.elements = self.elements().into_iter().collect();
        }
    }

    fn is_top(&self, e: &E) -> bool {
        let freq = self.cms.estimate(e) as f64 / self.n as f64;
        freq > self.min
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use countmin::CountMinSketch;

    #[test]
    fn insert_and_query() {
        let cms = CountMinSketch::with_confidence(0.0001, 0.99);
        let mut topk = TopK::new(5, 0.05, cms);

        for i in 1..10000 {
            topk.insert(i % 1000);
            topk.insert(-100);
        }

        assert_eq!(topk.elements(), vec![-100]);
    }
}
