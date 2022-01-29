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

fn external_regex_multi_star_max_recursion_comparison(c: &mut Criterion) {
    let pattern = "a*b*c*d*e*f*g*h*i*j*k*l*m*n*o*";
    let string = "aaaaaaaaaaaaaaaaaaaa";

    c.bench_function(
        "external regex multi-star with max recursion for comparison",
        |b| {
            b.iter(|| {
                Regex::new(black_box(pattern))
                    .unwrap()
                    .is_match(black_box(string))
            })
        },
    );
}

criterion_group!(
    benches,
    external_regex_multi_star,
    external_regex_multi_star_max_recursion_comparison
);
criterion_main!(benches);
