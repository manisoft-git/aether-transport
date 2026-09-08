use aether_coder::FulcrumEncoder;
use aether_math::simd::vec_mul_add;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn bench_gf_vec_mul_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("gf256_vec_mul_add");
    for size in [64, 512, 1400].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        let src = vec![0x5Au8; *size];
        let mut dst = vec![0x3Cu8; *size];
        let coeff = 0x77u8;

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                vec_mul_add(black_box(&mut dst), black_box(&src), black_box(coeff));
            });
        });
    }
    group.finish();
}

fn bench_fulcrum_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("fulcrum_encoder");
    const K: usize = 16;
    const S: usize = 1400;

    group.throughput(Throughput::Bytes((K * S) as u64));
    let source_symbols: Vec<Vec<u8>> = (0..K).map(|i| vec![i as u8; S]).collect();
    let encoder = FulcrumEncoder::new(source_symbols, 2);
    let mut repair_out = vec![0u8; S];

    group.bench_function("generate_repair_1400B", |b| {
        let mut seed = 1u16;
        b.iter(|| {
            encoder.generate_repair(black_box(seed), black_box(&mut repair_out));
            seed = seed.wrapping_add(1);
        });
    });
    group.finish();
}

criterion_group!(benches, bench_gf_vec_mul_add, bench_fulcrum_encoding);
criterion_main!(benches);
