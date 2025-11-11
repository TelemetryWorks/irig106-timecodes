use criterion::{black_box, criterion_group, criterion_main, Criterion};
use irig106_timecodes::Timecode;

fn parse_bench(c: &mut Criterion) {
    let sample = vec![0x11u8; 16];
    c.bench_function("parse_timecode_sample", |b| {
        b.iter(|| {
            let _ = Timecode::from_bytes_lossy(black_box(&sample));
        })
    });
}

criterion_group!(benches, parse_bench);
criterion_main!(benches);
