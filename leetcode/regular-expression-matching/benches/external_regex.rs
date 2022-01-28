use criterion::{black_box, criterion_group, criterion_main, Criterion};
use regex::Regex;

/// This is for comparing this implementation with the regex crate.
/// Note that this is not a fair comparison and just for curiousity.
fn external_regex_multi_star(c: &mut Criterion) {
    let pattern = "a*b*c*";
    let string = "aaaaabccc";

    c.bench_function("external regex multi-star", |b| {
        b.iter(|| {
            Regex::new(black_box(pattern))
                .unwrap()
                .is_match(black_box(string))
        })
    });
}

criterion_group!(benches, external_regex_multi_star);
criterion_main!(benches);
