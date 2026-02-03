use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("validate_valid", |b| {
        b.iter(|| {
            // Simulated validation
            let email = black_box("user.name+tag@example.com");
            // Fast check only
            let _ = email.contains('@');
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
