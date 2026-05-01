use criterion::{black_box, criterion_group, criterion_main, Criterion};
use anydesk_pivot_detector::parsers::parse_trace_file;
use std::io::Write;
use tempfile::NamedTempFile;

fn bench_parser_10k(c: &mut Criterion) {
    let mut file = NamedTempFile::new().unwrap();
    for i in 0..10000 {
        writeln!(file, "2025-05-01 10:00:00.000  info anydesk - Event {} from 123 456 789", i).unwrap();
    }
    let path = file.path().to_path_buf();
    
    c.bench_function("parse_10k_lines", |b| b.iter(|| {
        parse_trace_file(black_box(&path)).unwrap()
    }));
}

fn bench_parser_100k(c: &mut Criterion) {
    let mut file = NamedTempFile::new().unwrap();
    for i in 0..100000 {
        writeln!(file, "2025-05-01 10:00:00.000  info anydesk - Event {} from 123 456 789", i).unwrap();
    }
    let path = file.path().to_path_buf();
    
    // Limit iterations for 100k to save time in CI/tests
    let mut group = c.benchmark_group("LargeLog");
    group.sample_size(10);
    group.bench_function("parse_100k_lines", |b| b.iter(|| {
        parse_trace_file(black_box(&path)).unwrap()
    }));
    group.finish();
}

criterion_group!(benches, bench_parser_10k, bench_parser_100k);
criterion_main!(benches);
