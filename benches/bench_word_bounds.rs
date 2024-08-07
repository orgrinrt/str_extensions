use criterion::{black_box, criterion_group, criterion_main, Criterion};

use str_extensions::resolver::impls::{Charwalk, FancyRegex, Regex, WordBoundResolverImpl};
use str_extensions::resolver::rules::DefaultRules;
use str_extensions::resolver::WordBoundResolver;

fn word_bounds_fancy_regex(s: &str) -> Vec<String> {
    WordBoundResolver::<FancyRegex, DefaultRules>::resolve(s)
}

fn word_bounds_regex(s: &str) -> Vec<String> {
    WordBoundResolver::<Regex, DefaultRules>::resolve(s)
}

fn word_bounds_charwalk(s: &str) -> Vec<String> {
    WordBoundResolver::<Charwalk, DefaultRules>::resolve(s)
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

    c.bench_function("word_bounds_charwalk", |b| {
        b.iter(|| word_bounds_charwalk(black_box(input)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
