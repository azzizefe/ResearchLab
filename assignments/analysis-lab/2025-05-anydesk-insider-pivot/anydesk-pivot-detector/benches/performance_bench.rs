use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use anydesk_pivot_detector::analyzers::PivotDetector;

fn bench_high_load_analysis(c: &mut Criterion) {
    let detector = PivotDetector::new();
    let sample_log = "2025-05-04 12:00:00.000  1234  5678 L info: Incoming session from 192.168.1.50 (user: admin)";
    
    let mut group = c.benchmark_group("High Load Analysis");
    group.sample_size(10); // Smaller sample size for high-load simulation

    group.bench_function("1000_events_batch", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                let _ = detector.analyze_line(black_box(sample_log));
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_high_load_analysis);
criterion_main!(benches);
