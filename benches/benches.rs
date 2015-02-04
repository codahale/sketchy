#![feature(test)]

extern crate sketchy;
extern crate test;

use sketchy::{BloomFilter, CountMinSketch, HyperLogLog, ReservoirSample};
use test::Bencher;

#[bench]
fn bloomf_insert(b: &mut Bencher) {
    let mut bf = BloomFilter::new(100_000, 0.01);

    b.iter(|| {
        bf.insert("this is the end")
    })
}

#[bench]
fn bloomf_merge(b: &mut Bencher) {
    let mut bf1 = BloomFilter::new(100_000, 0.01);
    bf1.insert("this is the end");

    let mut bf2 = BloomFilter::new(100_000, 0.01);
    bf2.insert("this is not the end");

    b.iter(|| {
        bf1.merge(&bf2)
    })
}

#[bench]
fn cms_add(b: &mut Bencher) {
    let mut cms = CountMinSketch::with_confidence(0.0001, 0.99);

    b.iter(|| {
        cms.add("this is the end")
    })
}

#[bench]
fn cms_add_n(b: &mut Bencher) {
    let mut cms = CountMinSketch::with_confidence(0.0001, 0.99);

    b.iter(|| {
        cms.add_n("this is the end", 100)
    })
}

#[bench]
fn cms_estimate(b: &mut Bencher) {
    let cms = CountMinSketch::with_confidence(0.0001, 0.99);

    b.iter(|| {
        cms.estimate("this is the end")
    })
}

#[bench]
fn cms_estimate_mean(b: &mut Bencher) {
    let cms = CountMinSketch::with_confidence(0.0001, 0.99);

    b.iter(|| {
        cms.estimate_mean("this is the end", 100)
    })
}

#[bench]
fn cms_merge(b: &mut Bencher) {
    let mut one = CountMinSketch::<u64>::new(10, 1000);
    let two = CountMinSketch::new(10, 1000);

    b.iter(|| {
        one.merge(&two)
    })
}

#[bench]
fn hll_insert(b: &mut Bencher) {
    let mut hll = HyperLogLog::new(0.05);

    b.iter(|| {
        hll.insert(100u32)
    })
}

#[bench]
fn res_insert(b: &mut Bencher) {
    let mut res = ReservoirSample::new(1000);

    b.iter(|| {
        res.insert(100u32)
    })
}
