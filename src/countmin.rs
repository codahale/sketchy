use std::f64::consts::E;
use std::hash::{Hash,SipHasher};
use std::iter::repeat;
use std::num::Float;

use hash::indexes;

/// A Count-Min Sketch is a probabilistic data structure which provides
/// estimates of the frequency of elements in a data stream. It is parameterized
/// with the type of elements and the type of counter to use.
///
/// ```
/// use sketchy::CountMinSketch;
///
/// let mut cms = CountMinSketch::with_confidence(0.001, 0.99);
/// cms.add("one hundred");
/// cms.add_n("one hundred", 100);
///
/// println!("how many? {}", cms.estimate("one hundred"));
/// ```
pub struct CountMinSketch<E> {
    depth: usize,
    width: usize,
    counters: Vec<Vec<u64>>,
}

impl<E: Hash<SipHasher>> CountMinSketch<E> {
    /// Returns a CountMinSketch which provides frequency estimates where the
    /// error is within a factor of epsilon with the given confidence.
    pub fn with_confidence(epsilon: f64, confidence: f64) -> CountMinSketch<E> {
        let depth = (1.0 / (1.0 - confidence)).ln().ceil() as usize;
        let width = (E / epsilon).ceil() as usize;
        CountMinSketch::new(depth, width)
    }

    /// Returns a CountMinSketch with the given depth and width.
    pub fn new(depth: usize, width: usize) -> CountMinSketch<E> {
        CountMinSketch::<E> {
            depth: depth,
            width: width,
            counters: repeat({
                repeat(0).take(width).collect()
            }).take(depth).collect(),
        }
    }

    /// Registers the occurrence of a single element.
    pub fn add(&mut self, e: E) {
        self.add_n(e, 1)
    }

    /// Registers multiple occurrences of a element.
    pub fn add_n(&mut self, e: E, n: u64) {
        for (i, idx) in indexes(e, self.width).take(self.depth).enumerate() {
            self.counters[i][idx] = self.counters[i][idx] + n;
        }
    }

    /// Estimates the frequency of the given element.
    pub fn estimate(&self, e: E) -> u64 {
        let mut max: u64 = 0;
        for (i, idx) in indexes(e, self.width).take(self.depth).enumerate() {
            let v = self.counters[i][idx];
            if v > max {
                max = v
            }
        }
        max
    }

    /// Merges another Count-Min Sketch into self.
    pub fn merge(&mut self, v: &CountMinSketch<E>) {
        self.counters = self.counters.iter().zip(v.counters.iter()).map(|(s, o)| {
            s.iter().zip(o.iter()).map(|(&a, &b)| a + b).collect()
        }).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn with_confidence() {
        let cms = CountMinSketch::<u8>::with_confidence(0.0001, 0.99);

        assert_eq!(cms.counters.len(), 5);
        assert_eq!(cms.counters[0].len(), 27183);
    }

    #[test]
    fn add_and_estimate() {
        let mut cms = CountMinSketch::new(10, 10);
        cms.add("one hundred");
        cms.add_n("one hundred", 100);

        assert_eq!(cms.estimate("one hundred"), 101);
    }

    #[test]
    fn merge() {
        let mut one = CountMinSketch::new(10, 1000);
        one.add("one hundred");

        let mut two = CountMinSketch::new(10, 1000);
        two.add("two hundred");

        one.merge(&two);

        assert_eq!(one.estimate("two hundred"), 1);
    }
}
