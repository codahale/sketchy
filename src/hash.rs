#![allow(unstable)]

use std::hash::{Hash,Hasher,SipHasher};

#[inline(always)]
pub fn hashes<E: Hash<SipHasher>>(e: E) -> (u64, u64) {
    let mut h = SipHasher::new();
    e.hash(&mut h);
    let hash1 = h.finish();

    h = SipHasher::new_with_keys(0, hash1);
    e.hash(&mut h);
    let hash2 = h.finish();

    (hash1, hash2)
}

#[inline(always)]
pub fn index(h1: u64, h2: u64, i: usize, len: usize) -> usize {
    ((h1 + i as u64 * h2) % len as u64) as usize
}
