use std::ops::Range;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use criterion::measurement::WallTime;
use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion,
};
use string_alloc::StringRS;
use rand::distributions::{Alphanumeric, DistString};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

static SMALL_STRING_MAX: usize = 15;
static MEDIUM_STRING_MAX: usize = (1 << 14) - 1;

#[inline(never)]
fn reg_str_comp(string_a: String, string_b: String) -> bool {
    string_a.len() == string_b.len()
}

#[inline(never)]
fn al_str_comp(string_a: StringRS, string_b: StringRS) -> bool {
    string_a.len() == string_b.len()
}

fn normal_string_comp(string_a: &str, string_b: &str) -> bool {
    let string_a = String::from(string_a);
    let string_b = String::from(string_b);
    return reg_str_comp(string_a, string_b);
}

fn alloc_str_comp(string_a: &str, string_b: &str) -> bool {
    let string_a = StringRS::from(string_a);
    let string_b = StringRS::from(string_b);
    return al_str_comp(string_a, string_b);
}

fn bencher(group: &mut BenchmarkGroup<WallTime>, size: Range<usize>, seed: u64) {
    group.bench_function(BenchmarkId::new("StringRS", size.start), |b| {
        b.iter_custom(|iters| {
            let mut total_duration = Duration::new(0, 0);
            for _ in 0..iters {
                let mut rng = StdRng::seed_from_u64(seed);
                let range = rng.gen_range(size.clone());
                let string_a = Alphanumeric.sample_string(&mut rng, range.clone());
                let random_bool: bool = rng.gen_bool(0.9);
                let string_b = if random_bool {
                    Alphanumeric.sample_string(&mut rng, range.clone())
                } else {
                    string_a.clone()
                };
                let start = Instant::now();
                black_box(alloc_str_comp(&string_a, &string_b));
                total_duration += start.elapsed();
            }
            total_duration
        })
    });

    group.bench_function(BenchmarkId::new("Regular", size.start), |b| {
        b.iter_custom(|iters| {
            let mut total_duration = Duration::new(0, 0);
            for _ in 0..iters {
                let mut rng = StdRng::seed_from_u64(seed);
                let range = rng.gen_range(size.clone());
                let string_a = Alphanumeric.sample_string(&mut rng, range.clone());
                let random_bool: bool = rng.gen_bool(0.9);
                let string_b = if random_bool {
                    Alphanumeric.sample_string(&mut rng, range.clone())
                } else {
                    string_a.clone()
                };
                let start = Instant::now();
                black_box(normal_string_comp(&string_a, &string_b));
                total_duration += start.elapsed();
            }
            total_duration
        })
    });
}

pub fn strings_benchmark(c: &mut Criterion) {
    let ranges = vec![
        (SMALL_STRING_MAX..5000),
        (5000..10_000),
        (10_000..MEDIUM_STRING_MAX),
        // (MEDIUM_STRING_MAX + 1..MEDIUM_STRING_MAX + 5000),
        // (MEDIUM_STRING_MAX + 5000..MEDIUM_STRING_MAX + 10_000),
    ];
    let mut group = c.benchmark_group("Strings Alloc");
    for size in ranges {
        let seed: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        bencher(&mut group, size.clone(), seed);
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().significance_level(0.1).sample_size(500).measurement_time(Duration::from_secs(10));
    targets = strings_benchmark
}
criterion_main!(benches);
