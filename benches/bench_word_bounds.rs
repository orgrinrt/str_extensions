use criterion::{black_box, criterion_group, criterion_main, Criterion};

use str_extensions::resolver::impls::{
    WordBoundResolverFancyRegex, WordBoundResolverRegexless, WordBoundsResolverRegex,
};
use str_extensions::resolver::WordBoundResolverLike;

fn word_bounds_fancy_regex(s: &str) -> Vec<String> {
    WordBoundResolverFancyRegex::resolve(s)
}

fn word_bounds_regex(s: &str) -> Vec<String> {
    WordBoundsResolverRegex::resolve(s)
}

fn word_bounds_no_regex(s: &str) -> Vec<String> {
    WordBoundResolverRegexless::resolve(s)
}

fn criterion_benchmark(c: &mut Criterion) {
    let input = "This_is_SomeRandom_Text-to-split2";

    // The `black_box` function is used to prevent the compiler from optimizing the code in a way that might impact the benchmarking process.
    c.bench_function("word_bounds_regex", |b| {
        b.iter(|| word_bounds_regex(black_box(input)))
    });

    c.bench_function("word_bounds_fancy_regex", |b| {
        b.iter(|| word_bounds_fancy_regex(black_box(input)))
    });

    c.bench_function("word_bounds_no_regex", |b| {
        b.iter(|| word_bounds_no_regex(black_box(input)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
