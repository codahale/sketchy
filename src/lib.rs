//! Sketchy is a Rust library of probabilistic data structures.

#![feature(collections)]
#![feature(core)]
#![feature(hash)]
#![feature(rand)]

mod bloomfilter;
mod countmin;
mod hash;
mod reservoir;

pub use bloomfilter::BloomFilter;
pub use countmin::CountMinSketch;
pub use reservoir::ReservoirSample;
