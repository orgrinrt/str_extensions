//! The examples are built by `cargo test` and never run by it, so they are run here.
//!
//! An example that compiles and then panics, or prints the wrong thing, is an example that
//! lies to whoever copies it. Each one is run in the feature selection it is written for,
//! and its output checked against what it claims.

use std::process::Command;

/// Runs one example and returns what it printed, failing with what cargo said if it did not
/// exit zero.
fn run_example(name: &str, features: &[&str]) -> String {
    let mut command = Command::new(env!("CARGO"));
    command
        .args(["run", "-q", "--example", name])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        // A target directory of its own, so the outer run's build lock is not held against
        // this one.
        .env("CARGO_TARGET_DIR", concat!(env!("CARGO_MANIFEST_DIR"), "/target/examples"));
    if !features.is_empty() {
        command.args(["--no-default-features", "--features", &features.join(",")]);
    }

    let output = command
        .output()
        .unwrap_or_else(|e| panic!("could not run example {name}: {e}"));

    assert!(
        output.status.success(),
        "example {name} exited {}\n--- stderr\n{}",
        output.status,
        String::from_utf8_lossy(&output.stderr),
    );

    String::from_utf8(output.stdout).expect("example printed something that is not utf-8")
}

#[test]
fn case_conversions_shows_all_six_and_the_segmentation_behind_them() {
    let out = run_example("case_conversions", &[]);

    // One line per conversion, with the answer, so a wrong conversion fails here rather
    // than only a missing one.
    for (label, answer) in [
        ("to_snake_case", "some_http_request_id"),
        ("to_camel_case", "someHttpRequestId"),
        ("to_pascal_case", "SomeHttpRequestId"),
        ("to_kebab_case", "some-http-request-id"),
        ("to_human_readable", "some http request id"),
        ("to_title_case", "Some Http Request Id"),
    ] {
        assert!(out.contains(label), "no {label} line in:\n{out}");
        assert!(out.contains(answer), "{label} did not produce {answer:?} in:\n{out}");
    }

    // The point the second half makes: five spellings reach one answer, so the answer has
    // to appear more than once.
    assert!(
        out.matches("some_http_request").count() >= 6,
        "the five spellings did not converge in:\n{out}",
    );

    // And the third: a separator with nothing alphanumeric in it is dropped rather than
    // doubled.
    assert!(out.contains("prepended_underscore"), "the drop filter did not fire in:\n{out}");
    assert!(!out.contains("_prepended_underscore"), "a doubled separator survived in:\n{out}");
}

#[test]
fn the_no_allocator_example_converts_and_refuses() {
    let out = run_example("no_allocator_at_all", &["no_alloc", "full_format"]);

    // Three cases per field, from one reused stack buffer.
    for expected in [
        "user_id",
        "userId",
        "USER_ID",
        "http_status_code",
        "httpStatusCode",
        "HTTP_STATUS_CODE",
        // The leading underscore is dropped here as well, which is what says the lending
        // conversion carries the same filter as the allocating one.
        "internal_flag",
    ] {
        assert!(out.contains(expected), "missing {expected:?} in:\n{out}");
    }

    // The refusal, carrying both numbers, and the retry that the numbers make possible.
    assert!(
        out.contains("refused: wanted at least 9 bytes, had 8"),
        "no refusal in:\n{out}",
    );
    assert!(out.contains("at 32 bytes: http status code"), "no retry in:\n{out}");
}
