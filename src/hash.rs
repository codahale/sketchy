use std::hash::{Hash,Hasher,SipHasher};
use std::iter::Iterator;

/// Returns an iterator of indexes for the given element with a maximum
/// size. This uses [double
/// hashing](http://www.eecs.harvard.edu/~kirsch/pubs/bbbf/esa06.pdf), allowing
/// for multiple indexes to be created from only two full runs through
/// SipHash2-4.
pub fn indexes<E: Hash<SipHasher>>(e: E, max: usize) -> Index {
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

struct Index {
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
        Some(((self.h1 + self.i * self.h2) % self.max) as usize)
    }
}
