use std::collections::BitVec;
use std::hash::Hash;
use std::marker::PhantomData;

use hash::indexes;

/// A Bloom filter is a space-efficient probabilistic data structure that is
/// used to test whether an element is a member of a set. False positive matches
/// are possible, but false negatives are not.
///
/// ```
/// use sketchy::BloomFilter;
///
/// // Create a filter which can handle 100K elements with a 1% maximum
/// // probability of a false positive.
/// let mut filter = BloomFilter::new(100_000, 0.01);
///
/// filter.insert("one");
/// filter.insert("two");
///
/// assert!(filter.contains(&"one"));
/// ```
pub struct BloomFilter<E> {
    k: usize,
    bits: BitVec,
    marker: PhantomData<E>,
}

impl<E: Hash> BloomFilter<E> {
    /// Creates a new `BloomFilter` instance, tuned for a population of `n`
    /// elements with the given upper bound of the probability of false
    /// positives.
    pub fn new(n: usize, max_false_pos_prob: f64) -> BloomFilter<E> {
        let (buckets, k) = best_buckets_and_k(max_false_pos_prob);
        BloomFilter::<E> {
            k: k,
            bits: BitVec::from_elem(n * buckets + 20, false),
            marker: PhantomData,
        }
    }

    /// Adds a value to the set.
    pub fn insert(&mut self, e: E) {
        for i in indexes(&e, self.bits.len()).take(self.k) {
            self.bits.set(i, true);
        }
    }

    /// Returns `true` if the set probably contains the given element.
    pub fn contains(&mut self, e: &E) -> bool {
        for i in indexes(e, self.bits.len()).take(self.k) {
            if !self.bits.get(i).unwrap() {
                return false
            }
        }
        true
    }

    /// Merges the contents of the given `BloomFilter` into `self`. Both
    /// filters must have the same parameters. Returns true if self changed.
    ///
    /// # Panics
    ///
    /// Panics if the bloom filters have different parameters.
    pub fn merge(&mut self, other: &BloomFilter<E>) -> bool {
        assert_eq!(self.k, other.k);
        self.bits.union(&other.bits)
    }
}

fn best_buckets_and_k(max_false_pos_prob: f64) -> (usize, usize) {
    // Handle the trivial cases
    if max_false_pos_prob >= PROBS[MIN_BUCKETS][MIN_K] {
        return (2, OPT_K[2])
    }

    if max_false_pos_prob < PROBS[MAX_BUCKETS][MAX_K] {
        return (MAX_K, MAX_BUCKETS)
    }

    // First find the minimal required number of buckets:
    let mut buckets = 2;
    let mut k = OPT_K[2];
    while PROBS[buckets][k] > max_false_pos_prob {
        buckets += 1;
        k = OPT_K[buckets];
    }

    // Now that the number of buckets is sufficient, see if we can relax K
    // without losing too much precision.
    while PROBS[buckets][k-1] <= max_false_pos_prob {
        k -= 1;
    }

    (buckets, k)
}

static MAX_BUCKETS: usize = 15;
static MIN_BUCKETS: usize = 2;
static MIN_K: usize = 1;
static MAX_K: usize = 8;

static OPT_K: [usize; 21] = [
    1, // dummy K for 0 buckets per element
    1, // dummy K for 1 buckets per element
    1, 2, 3, 3, 4, 5, 5, 6, 7, 8, 8, 9, 10, 10, 11, 12, 12, 13, 14,
    ];

static PROBS: [&'static [f64]; 21] = [
    &[1.0], // dummy row representing 0 buckets per element
    &[1.0, 1.0], // dummy row representing 1 buckets per element
    &[1.0, 0.393, 0.400],
    &[1.0, 0.283, 0.237, 0.253],
    &[1.0, 0.221, 0.155, 0.147, 0.1600],
    &[1.0, 0.181, 0.109, 0.092, 0.092, 0.101], // 5
    &[1.0, 0.154, 0.0804, 0.0609, 0.0561, 0.0578, 0.0638],
    &[1.0, 0.133, 0.0618, 0.0423, 0.0359, 0.0347, 0.0364],
    &[1.0, 0.118, 0.0489, 0.0306, 0.024, 0.0217, 0.0216, 0.0229],
    &[1.0, 0.105, 0.0397, 0.0228, 0.0166, 0.0141, 0.0133, 0.0135, 0.0145],
    &[1.0, 0.0952, 0.0329, 0.0174, 0.0118, 0.00943, 0.00844, 0.00819, 0.00846], // 10
    &[1.0, 0.0869, 0.0276, 0.0136, 0.00864, 0.0065, 0.00552, 0.00513, 0.00509],
    &[1.0, 0.08, 0.0236, 0.0108, 0.00646, 0.00459, 0.00371, 0.00329, 0.00314],
    &[1.0, 0.074, 0.0203, 0.00875, 0.00492, 0.00332, 0.00255, 0.00217, 0.00199, 0.00194],
    &[1.0, 0.0689, 0.0177, 0.00718, 0.00381, 0.00244, 0.00179, 0.00146, 0.00129, 0.00121, 0.0012],
    &[1.0, 0.0645, 0.0156, 0.00596, 0.003, 0.00183, 0.00128, 0.001, 0.000852, 0.000775, 0.000744], // 15
    &[1.0, 0.0606, 0.0138, 0.005, 0.00239, 0.00139, 0.000935, 0.000702, 0.000574, 0.000505, 0.00047, 0.000459],
    &[1.0, 0.0571, 0.0123, 0.00423, 0.00193, 0.00107, 0.000692, 0.000499, 0.000394, 0.000335, 0.000302, 0.000287, 0.000284],
    &[1.0, 0.054, 0.0111, 0.00362, 0.00158, 0.000839, 0.000519, 0.00036, 0.000275, 0.000226, 0.000198, 0.000183, 0.000176],
    &[1.0, 0.0513, 0.00998, 0.00312, 0.0013, 0.000663, 0.000394, 0.000264, 0.000194, 0.000155, 0.000132, 0.000118, 0.000111, 0.000109],
    &[1.0, 0.0488, 0.00906, 0.0027, 0.00108, 0.00053, 0.000303, 0.000196, 0.00014, 0.000108, 8.89e-05, 7.77e-05, 7.12e-05, 6.79e-05, 6.71e-05], // 20
    ];


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert_and_query() {
        let mut bf = BloomFilter::new(100, 0.01);
        bf.insert(100);
        bf.insert(400);

        assert_eq!(bf.contains(&100), true);
    }

    #[test]
    fn merge() {
        let mut bf1 = BloomFilter::new(100, 0.01);
        bf1.insert(100);

        let mut bf2 = BloomFilter::new(100, 0.01);
        bf2.insert(400);

        if !bf1.merge(&bf2) {
            panic!("merge made no changes");
        }

        assert_eq!(bf1.contains(&400), true);
    }
}
