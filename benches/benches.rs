#[macro_use]
extern crate criterion;
extern crate sketchy;

use criterion::Criterion;
use sketchy::{BloomFilter, CountMinSketch, HyperLogLog, ReservoirSample, TopK};

fn bloomf_insert(c: &mut Criterion) {
    let mut bf = BloomFilter::new(100_000, 0.01);
    c.bench_function("BloomFilter::insert", move |b| {
        b.iter(|| bf.insert("this is the end"))
    });
}

fn bloomf_merge(c: &mut Criterion) {
    let mut bf1 = BloomFilter::new(100_000, 0.01);
    bf1.insert("this is the end");

    let mut bf2 = BloomFilter::new(100_000, 0.01);
    bf2.insert("this is not the end");

    c.bench_function("BloomFilter::merge", move |b| {
        b.iter(|| bf1.merge(&bf2))
    });
}

fn cms_insert(c: &mut Criterion) {
    let mut cms = CountMinSketch::with_confidence(0.0001, 0.99);

    c.bench_function("CountMinSketch::insert", move |b| {
        b.iter(|| cms.insert("this is the end"))
    });
}

fn cms_insert_n(c: &mut Criterion) {
    let mut cms = CountMinSketch::with_confidence(0.0001, 0.99);

    c.bench_function("CountMinSketch::insert_n", move |b| {
        b.iter(|| cms.insert_n("this is the end", 100))
    });
}

fn cms_estimate(c: &mut Criterion) {
    let cms = CountMinSketch::with_confidence(0.0001, 0.99);

    c.bench_function("CountMinSketch::estimate", move |b| {
        b.iter(|| cms.estimate(&"this is the end"))
    });
}

fn cms_estimate_mean(c: &mut Criterion) {
    let cms = CountMinSketch::with_confidence(0.0001, 0.99);

    c.bench_function("CountMinSketch::estimate_mean", move |b| {
        b.iter(|| cms.estimate_mean(&"this is the end", 100))
    });
}

fn cms_merge(c: &mut Criterion) {
    let mut one = CountMinSketch::<u64>::new(10, 1000);
    let two = CountMinSketch::new(10, 1000);

    c.bench_function("CountMinSketch::merge", move |b| {
        b.iter(|| one.merge(&two))
    });
}

fn hll_insert(c: &mut Criterion) {
    let mut hll = HyperLogLog::new(0.05);

    c.bench_function("HyperLogLog::insert", move |b| {
    b.iter(|| hll.insert(100u32))
    });
}

fn res_insert(c: &mut Criterion) {
    let mut res = ReservoirSample::new(1000);

    c.bench_function("ReservoirSample::insert", move |b| {
    b.iter(|| res.insert(100u32))
    });
}

fn topk_insert(c: &mut Criterion) {
    let cms = CountMinSketch::with_confidence(0.001, 0.99);
    let mut topk = TopK::new(5, 0.05, cms);

    c.bench_function("TopK::insert", move |b| {
    b.iter(|| topk.insert(100u32))
    });
}

criterion_group!(
    benches,
    bloomf_insert,
    bloomf_merge,
    cms_insert,
    cms_insert_n,
    cms_estimate,
    cms_estimate_mean,
    cms_merge,
    hll_insert,
    res_insert,
    topk_insert
);
criterion_main!(benches);

