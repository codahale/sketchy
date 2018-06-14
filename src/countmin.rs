use std::f64::consts::E;
use std::hash::Hash;
use std::marker::PhantomData;

use hash::indexes;

/// A Count-Min Sketch is a probabilistic data structure which provides estimates of the frequency
/// of elements in a data stream. It is parameterized with the type of elements.
///
/// ```
/// use sketchy::CountMinSketch;
///
/// let mut cms = CountMinSketch::with_confidence(0.001, 0.99);
/// cms.insert("one hundred");
/// cms.insert_n("one hundred", 100);
///
/// assert_eq!(cms.estimate(&"one hundred"), 101);
/// ```
pub struct CountMinSketch<E> {
    depth: usize,
    width: usize,
    counters: Vec<Vec<u64>>,
    marker: PhantomData<E>,
}

impl<E: Hash> CountMinSketch<E> {
    /// Returns a `CountMinSketch` which provides frequency estimates where the error is within a
    /// factor of epsilon with the given confidence.
    pub fn with_confidence(epsilon: f64, confidence: f64) -> CountMinSketch<E> {
        let depth = (1.0 / (1.0 - confidence)).ln().ceil() as usize;
        let width = (E / epsilon).ceil() as usize;
        CountMinSketch::new(depth, width)
    }

    /// Returns a `CountMinSketch` with the given depth and width.
    pub fn new(depth: usize, width: usize) -> CountMinSketch<E> {
        CountMinSketch::<E> {
            depth,
            width,
            counters: vec![vec![0; width]; depth],
            marker: PhantomData,
        }
    }

    /// Adds a value to the sketch.
    pub fn insert(&mut self, e: E) {
        self.insert_n(e, 1)
    }

    /// Adds multiple instances of a value to the sketch.
    pub fn insert_n(&mut self, e: E, n: u64) {
        for (i, idx) in indexes(&e, self.width).take(self.depth).enumerate() {
            self.counters[i][idx] += n;
        }
    }

    /// Estimates the frequency of the given element.
    pub fn estimate(&self, e: &E) -> u64 {
        indexes(e, self.width)
            .take(self.depth)
            .enumerate()
            .map(|(i, idx)| self.counters[i][idx])
            .min()
            .unwrap()
    }

    /// Estimates the frequency of the given element using the [Count-Mean-Min
    /// algorithm](http://webdocs.cs.ualberta.ca/~fandeng/paper/cmm.pdf), which performs better on
    /// data sets which aren't highly skewed.
    pub fn estimate_mean(&self, e: E, n: u64) -> u64 {
        let mut values: Vec<u64> = indexes(&e, self.width)
            .take(self.depth)
            .enumerate()
            .map(|(i, idx)| {
                let v = self.counters[i][idx];
                let noise = (n - v) / (self.width - 1) as u64;
                v - noise
            })
            .collect();

        values.sort();
        if values.len() % 2 == 0 {
            (values[values.len() / 2] + values[(values.len() / 2) - 1]) / 2
        } else {
            values[values.len() / 2]
        }
    }

    /// Merges another `CountMinSketch` into `self`.
    pub fn merge(&mut self, v: &CountMinSketch<E>) {
        self.counters = self.counters
            .iter()
            .zip(v.counters.iter())
            .map(|(s, o)| s.iter().zip(o.iter()).map(|(&a, &b)| a + b).collect())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::collections::HashMap;

    use rand::thread_rng;
    use rand::distributions::{IndependentSample, Exp};

    #[test]
    fn with_confidence() {
        let cms = CountMinSketch::<u8>::with_confidence(0.0001, 0.99);

        assert_eq!(cms.counters.len(), 5);
        assert_eq!(cms.counters[0].len(), 27183);
    }

    #[test]
    fn insert_and_estimate() {
        let mut cms = CountMinSketch::new(100, 100);
        for i in 0..100 {
            cms.insert(i)
        }

        assert_eq!(cms.estimate(&20), 1);
    }

    #[test]
    fn insert_and_estimate_mean() {
        let mut cms = CountMinSketch::new(10, 100);
        cms.insert("one hundred");
        cms.insert("two hundred");
        cms.insert("three hundred");
        cms.insert("four hundred");
        cms.insert("five hundred");

        assert_eq!(cms.estimate(&"one hundred"), 1);
        assert_eq!(cms.estimate_mean(&"one hundred", 5), 1);
    }

    #[test]
    fn merge() {
        let mut one = CountMinSketch::new(10, 1000);
        one.insert("one hundred");

        let mut two = CountMinSketch::new(10, 1000);
        two.insert("two hundred");

        one.merge(&two);

        assert_eq!(one.estimate(&"two hundred"), 1);
    }

    #[test]
    fn accuracy() {
        let exp = Exp::new(2.0);
        let values: Vec<u32> = (0..1_000_000)
            .map(|_| (exp.ind_sample(&mut thread_rng()) * 1000.0) as u32)
            .collect();

        let mut actual: HashMap<u32, u64> = HashMap::new();
        let mut cms = CountMinSketch::with_confidence(0.0001, 0.99);

        for v in values.iter() {
            let n = actual.get(v).map_or(1, |x| x + 1);
            actual.insert(*v, n);
            cms.insert(*v);
        }

        for (v, &freq) in actual.iter() {
            assert_eq!(cms.estimate(v), freq);
        }
    }
}
