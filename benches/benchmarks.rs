use async_std::task;
use cacache;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tempfile;

fn read_hash(c: &mut Criterion) {
    let tmp = tempfile::tempdir().unwrap();
    let cache = tmp.path().to_owned();
    let data = b"hello world".to_vec();
    let sri = cacache::put::data(&cache, "hello", data).unwrap();
    c.bench_function("read_hash", move |b| {
        b.iter(|| cacache::get::read_hash(black_box(&cache), black_box(&sri)).unwrap())
    });
}

fn read(c: &mut Criterion) {
    let tmp = tempfile::tempdir().unwrap();
    let cache = tmp.path().to_owned();
    let data = b"hello world".to_vec();
    cacache::put::data(&cache, "hello", data).unwrap();
    cacache::get::read(&cache, "hello").unwrap();
    c.bench_function("read", move |b| {
        b.iter(|| cacache::get::read(black_box(&cache), black_box(String::from("hello"))).unwrap())
    });
}

fn async_read_hash(c: &mut Criterion) {
    let tmp = tempfile::tempdir().unwrap();
    let cache = tmp.path().to_owned();
    let data = b"hello world".to_vec();
    let sri = cacache::put::data(&cache, "hello", data).unwrap();
    c.bench_function("async_read_hash", move |b| {
        b.iter(|| {
            task::block_on(cacache::async_get::read_hash(
                black_box(&cache),
                black_box(&sri)
            )).unwrap()
        })
    });
}

fn async_read(c: &mut Criterion) {
    let tmp = tempfile::tempdir().unwrap();
    let cache = tmp.path().to_owned();
    let data = b"hello world".to_vec();
    cacache::put::data(&cache, "hello", data).unwrap();
    c.bench_function("async_read", move |b| {
        b.iter(|| {
            task::block_on(cacache::async_get::read(
                black_box(&cache),
                black_box("hello")
            )).unwrap()
        })
    });
}

criterion_group!(benches, read_hash, read, async_read_hash, async_read);
// criterion_group!(benches, read_hash, async_read_hash);
criterion_main!(benches);
