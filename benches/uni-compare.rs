use std::time::{Duration};

use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
    BenchmarkId,
    black_box,
};

use ws_lib::map_lookup::white_space::lookup as uni_ws;

const ISSUE_38: &str = include_str!("issue-38.json");

// Benchmarks the get_input_trait trait collection for inputs
// cargo bench --bench get-traits -- "get-traits/Trait collection"
// cargo bench --bench get-traits -- "get-traits/Trait collection/astronomy"
fn unicode_whitespace(c: &mut Criterion) {
    let mut group = c.benchmark_group("whitespace");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(3));


    let id = BenchmarkId::new("is_whitespace", "extended");
    group.bench_function(id, |b| b.iter(|| {
        for i in 0..0x4000 {
            let ch = char::from_u32(i).unwrap();
            let _  = char::is_whitespace(black_box(ch));
        }
    }));

    let id = BenchmarkId::new("uni_ws", "extended");
    group.bench_function(id, |b| b.iter(|| {
        for i in 0..0x4000 {
            let ch = char::from_u32(i).unwrap();
            uni_ws(black_box(ch));
        }
    }));

    let id = BenchmarkId::new("is_whitespace", "ascii");
    group.bench_function(id, |b| b.iter(|| {
        for i in 0..=0x7f {
            let ch = char::from_u32(i).unwrap();
            let _ = char::is_whitespace(black_box(ch));
        }
    }));

    let id = BenchmarkId::new("uni_ws", "ascii");
    group.bench_function(id, |b| b.iter(|| {
        for i in 0..=0x7f {
            let ch = char::from_u32(i).unwrap();
            uni_ws(black_box(ch));
        }
    }));

    let id = BenchmarkId::new("is_ascii_whitespace", "ascii");
    group.bench_function(id, |b| b.iter(|| {
        for i in 0..=0x7f {
            let ch = char::from_u32(i).unwrap();
            let _ = char::is_ascii_whitespace(black_box(&ch));
        }
    }));

    let id = BenchmarkId::new("is_whitespace", "first-256");
    group.bench_function(id, |b| b.iter(|| {
        for i in 0..=0xff {
            let ch = char::from_u32(i).unwrap();
            let _ = char::is_whitespace(black_box(ch));
        }
    }));

    let id = BenchmarkId::new("uni_ws", "first-256");
    group.bench_function(id, |b| b.iter(|| {
        for i in 0..=0xff {
            let ch = char::from_u32(i).unwrap();
            uni_ws(black_box(ch));
        }
    }));

    let id = BenchmarkId::new("is_whitespace", "issue-38");
    group.bench_function(id, |b| b.iter(|| {
        for ch in ISSUE_38.chars() {
            let _ = char::is_whitespace(black_box(ch));
        }
    }));

    let id = BenchmarkId::new("uni_ws", "issue-38");
    group.bench_function(id, |b| b.iter(|| {
        for ch in ISSUE_38.chars() {
            uni_ws(black_box(ch));
        }
    }));

    let id = BenchmarkId::new("is_whitespace", "single-space");
    group.bench_function(id, |b| b.iter(|| {
        let _ = char::is_whitespace(black_box(' '));
    }));

    let id = BenchmarkId::new("uni_ws", "single-space");
    group.bench_function(id, |b| b.iter(|| {
        uni_ws(black_box(' '));
    }));


    let id = BenchmarkId::new("is_whitespace", "single-a");
    group.bench_function(id, |b| b.iter(|| {
        let _ = char::is_whitespace(black_box('a'));
    }));

    let id = BenchmarkId::new("uni_ws", "single-a");
    group.bench_function(id, |b| b.iter(|| {
        uni_ws(black_box('a'));
    }));

    let id = BenchmarkId::new("is_whitespace", "single-tab");
    group.bench_function(id, |b| b.iter(|| {
        let _ = char::is_whitespace(black_box('a'));
    }));

    let id = BenchmarkId::new("uni_ws", "single-tab");
    group.bench_function(id, |b| b.iter(|| {
        uni_ws(black_box('a'));
    }));

    group.finish();
}

criterion_group!(benches, unicode_whitespace);
criterion_main!(benches);
