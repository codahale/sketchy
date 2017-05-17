use std::hash::{Hash,Hasher,SipHasher};
use std::iter::Iterator;

/// Returns an iterator of indexes for the given element with a maximum size. This uses [double
/// hashing](https://www.eecs.harvard.edu/~michaelm/postscripts/tr-02-05.pdf), allowing for multiple indexes
/// to be created from only two full runs through SipHash2-4.
pub fn indexes<E: Hash>(e: &E, max: usize) -> Index {
    let mut h = SipHasher::new();
    e.hash(&mut h);
    let hash1 = h.finish();

    h = SipHasher::new_with_keys(0, hash1);
    e.hash(&mut h);
    let hash2 = h.finish();

    Index {
        h1: hash1,
        h2: hash2,
        max: max as u64,
        i: 0,
    }
}

pub struct Index {
    h1: u64,
    h2: u64,
    max: u64,
    i: u64,
}

impl Iterator for Index {
    type Item = usize;

    #[inline(always)]
    fn next(&mut self) -> Option<usize> {
        self.i += 1;
        Some((self.h1.wrapping_add(self.i.wrapping_mul(self.h2)) % self.max) as usize)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn double_hashing() {
        let v: Vec<usize> = indexes(&"whee", 100).take(10).collect();

        assert_eq!(v, vec![17, 92, 83, 58, 33, 8, 99, 74, 49, 40]);
    }
}
