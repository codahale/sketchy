#![allow(unstable)]
extern crate sketchy;
extern crate test;

use sketchy::CountMinSketch;
use test::Bencher;

#[bench]
fn bench_add(b: &mut Bencher) {
    let mut cms = CountMinSketch::<_, u64>::with_confidence(0.0001, 0.99);

    b.iter(|| {
        cms.add("this is the end")
    })
}

#[bench]
fn bench_add_n(b: &mut Bencher) {
    let mut cms = CountMinSketch::<_, u64>::with_confidence(0.0001, 0.99);

    b.iter(|| {
        cms.add_n("this is the end", 100)
    })
}

#[bench]
fn bench_estimate(b: &mut Bencher) {
    let cms = CountMinSketch::<_, u64>::with_confidence(0.0001, 0.99);

    b.iter(|| {
        cms.estimate("this is the end")
    })
}

#[bench]
fn merge(b: &mut Bencher) {
    let mut one = CountMinSketch::<u64, u64>::new(10, 1000);
    let two = CountMinSketch::<u64, u64>::new(10, 1000);

    b.iter(|| {
        one.merge(&two)
    })
}
