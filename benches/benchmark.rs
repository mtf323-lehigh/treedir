use treedir::{Tree, generate_tree};
use std::path::Path;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn run_generate_tree() {
    let path = Path::new("/Users/michael/programming/treedir");
    generate_tree(path);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("generate_tree this cargo project",
		     |b| b.iter(|| run_generate_tree()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
