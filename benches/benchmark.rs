use std::{
    io::{BufRead, BufReader, Read},
    time::Duration,
};

use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

const FILENAME: &str = "data/3167.xz";

fn bench_decomp(c: &mut Criterion) {
    let mut group = c.benchmark_group("decomp");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(20));

    group.bench_function(BenchmarkId::new("xz2", FILENAME), |b| {
        b.iter_batched(
            || get_buffered_file_reader(FILENAME),
            |i| {
                let mut output = Vec::new();
                xz2::bufread::XzDecoder::new(i)
                    .read_to_end(&mut output)
                    .unwrap()
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function(BenchmarkId::new("lzma-rs", FILENAME), |b| {
        b.iter_batched_ref(
            || get_buffered_file_reader(FILENAME),
            |i| {
                let mut output = Vec::new();
                lzma_rs::xz_decompress(i, &mut output).unwrap();
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function(BenchmarkId::new("xz-decom", FILENAME), |b| {
        b.iter_batched(
            || std::fs::read(FILENAME).unwrap(),
            |i| {
                let _ = xz_decom::decompress(&i).unwrap();
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function(BenchmarkId::new("rust-lzma", FILENAME), |b| {
        b.iter_batched(
            || std::fs::read(FILENAME).unwrap(),
            |i| {
                let _ = lzma::decompress(&i).unwrap();
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

fn get_buffered_file_reader(filename: &str) -> impl BufRead {
    BufReader::new(std::fs::File::open(filename).expect("Couldn't open xz file"))
}

criterion_group!(benches, bench_decomp);
criterion_main!(benches);
