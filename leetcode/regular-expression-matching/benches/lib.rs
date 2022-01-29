use criterion::{black_box, criterion_group, criterion_main, Criterion};
use regular_expression_matching::regex;

fn regex_multi_star(c: &mut Criterion) {
    let pattern = "a*b*c*";
    let string = "aaaaabccc";

    c.bench_function("crate regex multi-star", |b| {
        b.iter(|| regex(black_box(pattern), black_box(string)))
    });
}

fn regex_multi_star_max_recursion(c: &mut Criterion) {
    let pattern = "a*b*c*d*e*f*g*h*i*j*k*l*m*n*o*";
    let string = "aaaaaaaaaaaaaaaaaaaa";

    c.bench_function("crate regex multi-star with max recursion", |b| {
        b.iter(|| regex(black_box(pattern), black_box(string)))
    });
}

criterion_group!(benches, regex_multi_star, regex_multi_star_max_recursion);
criterion_main!(benches);
