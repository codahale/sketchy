//! Sketchy is a Rust library of probabilistic data structures, useful for measuring large or
//! unbounded streams of data by trading some accuracy for a whole lot of efficiency.

extern crate bit_vec;
extern crate rand;

mod bloomfilter;
mod countmin;
mod hash;
mod hyperloglog;
mod reservoir;
mod topk;

pub use bloomfilter::BloomFilter;
pub use countmin::CountMinSketch;
pub use hyperloglog::HyperLogLog;
pub use reservoir::ReservoirSample;
pub use topk::TopK;
