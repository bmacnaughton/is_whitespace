use std::time::{Duration};

use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
    BenchmarkId,
    black_box,
};

use ws_lib::map_lookup::white_space::lookup as mapped;
use ws_lib::map_lookup::match_ws as match_ws;
use ws_lib::map_lookup::mapped_if;

const ISSUE_38: &str = include_str!("issue-38.json");
const ADDRESSES: &str = include_str!("500-random-us-addresses.json");


// Benchmarks the get_input_trait trait collection for inputs
// cargo bench --bench get-traits -- "get-traits/Trait collection"
// cargo bench --bench get-traits -- "get-traits/Trait collection/astronomy"
fn unicode_whitespace(c: &mut Criterion) {
    let first_128 = (0..0x80).map(|c| char::from_u32(c).unwrap()).collect::<String>();
    let first_256 = (0..0x100).map(|c| char::from_u32(c).unwrap()).collect::<String>();
    let first_16384 = (0..0x4000).map(|c| char::from_u32(c).unwrap()).collect::<String>();
    let thirty_k_blanks = (0..30_000).map(|_| ' ').collect::<String>();
    let inputs: Vec<(&str, &str)> = vec![
        ("issue-38", ISSUE_38),
        ("addresses", ADDRESSES),
        ("first-128", &first_128),
        ("first-256", &first_256),
        ("first-4000", &first_16384),
        ("30K-blanks", &thirty_k_blanks),
        ("single-space", " "),
        ("single-a", "a"),
        ("single-tab", "\t"),
    ];

    let mut group = c.benchmark_group("ws");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(3));

    for (name, string) in inputs {
        let id1 = BenchmarkId::new("is_whitespace", name);
        let id2 = BenchmarkId::new("is_ascii", name);
        let id3 = BenchmarkId::new("match", name);
        let id4 = BenchmarkId::new("mapped", name);
        let id5 = BenchmarkId::new("mapped-if", name);

        group.bench_with_input(id1, name, |b, _i|
                b.iter(|| string.chars().for_each(|c| {
                    black_box(c.is_whitespace());
                }))
        );

        group.bench_with_input(id2, name, |b, _i|
                b.iter(|| string.chars().for_each(|c| {
                    black_box(c.is_ascii_whitespace());
                }))
        );

        group.bench_with_input(id3, name, |b, _i|
                b.iter(|| string.chars().for_each(|c| {
                    black_box(match_ws(c));
                }))
        );

        group.bench_with_input(id4, name, |b, _i|
                b.iter(|| string.chars().for_each(|c| {
                    black_box(mapped(c));
                }))
        );

        group.bench_with_input(id5, name, |b, _i|
                b.iter(|| string.chars().for_each(|c| {
                    black_box(mapped_if(c));
                }))
        );

    }

    group.finish();
}

criterion_group!(benches, unicode_whitespace);
criterion_main!(benches);
