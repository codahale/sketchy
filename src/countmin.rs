#![allow(unstable)]

use std::f64::consts::E;
use std::hash::{Hash,Hasher,SipHasher};
use std::iter::repeat;
use std::num::{Float, Int};

/// A Count-Min Sketch is a probabilistic data structure which provides
/// estimates of the frequency of elements in a data stream. It is parameterized
/// with the type of elements and the type of counter to use.
///
/// ```
/// use sketchy::CountMinSketch;
///
/// let mut cms = CountMinSketch::<_, u64>::with_confidence(0.001, 0.99);
/// cms.add("one hundred");
/// cms.add_n("one hundred", 100);
///
/// println!("how many? {}", cms.estimate("one hundred"));
/// ```
pub struct CountMinSketch<E, C>{
    counters: Vec<Vec<C>>,
}

impl<E: Hash<SipHasher>, C: Copy + Int> CountMinSketch<E, C> {
    /// Returns a CountMinSketch which provides frequency estimates where the
    /// error is within a factor of epsilon with the given confidence.
    pub fn with_confidence(epsilon: f64, confidence: f64) -> CountMinSketch<E, C> {
        let width = (E / epsilon).ceil() as usize;
        let depth = (1.0 / (1.0 - confidence)).ln().ceil() as usize;
        CountMinSketch::new(depth, width)
    }

    /// Returns a CountMinSketch with the given depth and width.
    pub fn new(depth: usize, width: usize) -> CountMinSketch<E, C> {
        CountMinSketch::<E, C>{
            counters: repeat({
                repeat(Int::zero()).take(width).collect()
            }).take(depth).collect(),
        }
    }

    /// Registers the occurrence of a single element.
    pub fn add(&mut self, e: E) {
        self.add_n(e, Int::one())
    }

    /// Registers multiple occurrences of a element.
    pub fn add_n(&mut self, e: E, n: C) {
        let (h1, h2) = hash(e);

        for (i, c) in self.counters.iter_mut().enumerate() {
            let idx = index(h1, h2, i, c.len());
            c[idx] = c[idx] + n;
        }
    }

    /// Estimates the frequency of the given element.
    pub fn estimate(&self, e: E) -> C {
        let (h1, h2) = hash(e);

        let mut max: C = Int::zero();
        for (i, c) in self.counters.iter().enumerate() {
            let idx = index(h1, h2, i, c.len());
            let v = c[idx];
            if v > max {
                max = v
            }
        }
        max
    }

    /// Merges another Count-Min Sketch into self.
    pub fn merge(&mut self, v: CountMinSketch<E, C>) {
        self.counters = self.counters.iter().zip(v.counters.iter()).map(|(s, o)| {
            s.iter().zip(o.iter()).map(|(&a, &b)| a + b).collect()
        }).collect()
    }
}

#[inline(always)]
fn hash<K: Hash<SipHasher>>(k: K) -> (u64, u64) {
    let mut h = SipHasher::new();
    k.hash(&mut h);
    let hash1 = h.finish();

    h = SipHasher::new_with_keys(0, hash1);
    k.hash(&mut h);
    let hash2 = h.finish();

    (hash1, hash2)
}

#[inline(always)]
fn index(h1: u64, h2: u64, i: usize, len: usize) -> usize {
    ((h1 + i as u64 * h2) % len as u64) as usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn with_confidence() {
        let cms = CountMinSketch::<u8, u8>::with_confidence(0.0001, 0.99);

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
        let mut one = CountMinSketch::<_, u64>::new(10, 1000);
        one.add("one hundred");

        let mut two = CountMinSketch::new(10, 1000);
        two.add("two hundred");

        one.merge(two);

        assert_eq!(one.estimate("two hundred"), 1);
    }
}
