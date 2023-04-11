use criterion::{criterion_group, criterion_main, Criterion};
use pbix::parse_file;
use std::path::Path;

fn parse_benchmark(c: &mut Criterion) {
    let path = Path::new("Example.pbix");
    c.bench_function("parse", |b| b.iter(|| parse_file(&path).unwrap()));
}

criterion_group!(benches, parse_benchmark);
criterion_main!(benches);
