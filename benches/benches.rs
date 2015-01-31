#![allow(unstable)]
extern crate sketchy;
extern crate test;

use sketchy::{BloomFilter, CountMinSketch};
use test::Bencher;

#[bench]
fn bloomf_insert(b: &mut Bencher) {
    let mut bf = BloomFilter::new(1000, 0.001);

    b.iter(|| {
        bf.insert("this is the end")
    })
}

#[bench]
fn cms_add(b: &mut Bencher) {
    let mut cms = CountMinSketch::<_, u64>::with_confidence(0.0001, 0.99);

    b.iter(|| {
        cms.add("this is the end")
    })
}

#[bench]
fn cms_add_n(b: &mut Bencher) {
    let mut cms = CountMinSketch::<_, u64>::with_confidence(0.0001, 0.99);

    b.iter(|| {
        cms.add_n("this is the end", 100)
    })
}

#[bench]
fn cms_estimate(b: &mut Bencher) {
    let cms = CountMinSketch::<_, u64>::with_confidence(0.0001, 0.99);

    b.iter(|| {
        cms.estimate("this is the end")
    })
}

#[bench]
fn cms_merge(b: &mut Bencher) {
    let mut one = CountMinSketch::<u64, u64>::new(10, 1000);
    let two = CountMinSketch::<u64, u64>::new(10, 1000);

    b.iter(|| {
        one.merge(&two)
    })
}
