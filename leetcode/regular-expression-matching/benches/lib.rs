use criterion::{black_box, criterion_group, criterion_main, Criterion};
use regular_expression_matching::regex;

fn regex_multi_star(c: &mut Criterion) {
    let pattern = "a*b*c*";
    let string = "aaaaabccc";

    c.bench_function("crate regex multi-star", |b| {
        b.iter(|| regex(black_box(pattern), black_box(string)))
    });
}

criterion_group!(benches, regex_multi_star);
criterion_main!(benches);
