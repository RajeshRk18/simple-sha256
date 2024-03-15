use simple_sha256::{Sha256Hasher, Hasher};
use criterion::{Criterion, criterion_group, criterion_main, black_box};
use rand::rngs::OsRng;
use rand::RngCore;

fn sha256_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("SHA256");
    group.sample_size(100);

    let input_sizes = [16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 9192, 65536];

    for size in input_sizes {
        let mut data = vec![0u8; size];
        OsRng.fill_bytes(&mut data);

        group.bench_with_input(format!("My implementation with size_{}", size), &data, |b, data| {
            b.iter(|| {
                let mut hasher = Sha256Hasher::new();
                black_box(hasher.update(black_box(data)));
                black_box(hasher.finish());
            })
        });
    }

    group.finish();
}

criterion_group!(benches, sha256_benchmark);
criterion_main!(benches);