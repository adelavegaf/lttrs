use criterion::{criterion_group, criterion_main, Criterion};
use lttrs::Game;
use rand::{rngs::StdRng, SeedableRng};

fn criterion_benchmark(c: &mut Criterion) {
    let seed = Default::default();
    let mut rng = StdRng::from_seed(seed);

    let vocab = std::fs::read_to_string("./data/twl06.txt")
        .expect("vocab file to exist")
        .split('\n')
        .map(String::from)
        .collect();

    c.bench_function("game 20", |b| b.iter(|| Game::new(&mut rng, &vocab)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
