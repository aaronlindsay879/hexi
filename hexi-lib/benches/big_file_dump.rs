use std::os::unix::prelude::MetadataExt;

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use hexi_lib::{hexi::Hexi, options::Options};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("dump file");

    group.sample_size(10);
    group.throughput(Throughput::Bytes(
        std::fs::metadata("../target/debug/hexi").unwrap().size(),
    ));
    group.bench_function("hexi debug", |b| {
        b.iter(|| {
            black_box(
                Hexi::with_options(Options {
                    file: String::from("../target/debug/hexi"),
                    section_length: 8,
                    sections_per_line: 2,
                    chunk_size: 1,
                })
                .unwrap()
                .dump_file()
                .collect::<Vec<_>>(),
            )
        })
    });

    group.sample_size(100);
    group.throughput(Throughput::Bytes(
        std::fs::metadata("test.data").unwrap().size(),
    ));
    group.bench_function("small file debug", |b| {
        b.iter(|| {
            black_box(
                Hexi::with_options(Options {
                    file: String::from("test.data"),
                    section_length: 8,
                    sections_per_line: 2,
                    chunk_size: 1,
                })
                .unwrap()
                .dump_file()
                .collect::<Vec<_>>(),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
