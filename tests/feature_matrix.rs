//! Each feature flag selects its method, in both directions.
//!
//! The crate calls itself "extensively configurable by feature flags", and before this pass
//! the flags between them decided only whether a module existed: turning two off and one on
//! gave you all of the methods anyway. The flags select methods now, and this is what says
//! so.
//!
//! The positive direction is cheap: build the crate under each selection. The negative one
//! is the half that matters and the half that was missing, because a flag that selects
//! nothing passes every positive test. It cannot be a `trybuild` case: `trybuild` compiles
//! one configuration of this crate and hands it files, and the configuration *is* the
//! subject here. So a throwaway crate is written out, pointed at this one by path with a
//! chosen feature set, and built.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Builds this crate under one feature selection.
fn check(features: &str) -> (bool, String) {
    let mut command = Command::new(env!("CARGO"));
    command
        .args(["check", "--quiet", "--no-default-features", "--features", features])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env(
            "CARGO_TARGET_DIR",
            concat!(env!("CARGO_MANIFEST_DIR"), "/target/feature-matrix"),
        );
    let output = command.output().expect("cargo runs");
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

/// Builds a throwaway crate that uses `body`, against this crate at `features`.
///
/// Returns whether it compiled. The crate is written under this crate's own target
/// directory so it is cleaned with everything else.
fn consumer_compiles(name: &str, features: &str, body: &str) -> (bool, String) {
    let root = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/target/consumers")).join(name);
    fs::create_dir_all(root.join("src")).expect("the consumer directory");

    fs::write(
        root.join("Cargo.toml"),
        format!(
            r#"[package]
name = "{name}"
version = "0.0.0"
edition = "2021"

[dependencies.str_extensions]
path = "{crate_dir}"
default-features = false
features = [{features_list}]

[workspace]
"#,
            name = name,
            crate_dir = env!("CARGO_MANIFEST_DIR"),
            features_list = features
                .split(',')
                .filter(|f| !f.is_empty())
                .map(|f| format!("\"{f}\""))
                .collect::<Vec<_>>()
                .join(", "),
        ),
    )
    .expect("the consumer manifest");

    fs::write(
        root.join("src/main.rs"),
        format!("use str_extensions::prelude::*;\nfn main() {{\n{body}\n}}\n"),
    )
    .expect("the consumer source");

    let output = Command::new(env!("CARGO"))
        .args(["build", "--quiet"])
        .current_dir(&root)
        .output()
        .expect("cargo runs");
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

#[test]
fn every_building_selection_builds() {
    for selection in ["append", "prepend", "concat", "join", "full_building"] {
        let (ok, err) = check(selection);
        assert!(ok, "{selection} builds on its own:\n{err}");
    }
}

#[test]
fn the_prelude_survives_a_partial_selection() {
    // Each prelude re-export carries the same condition as the module it names. Without
    // that, any selection short of all of them failed with
    // `could not find 'cases' in the crate root`, so the configurability the crate
    // advertises could not be used at all.
    for selection in ["append", "to_snake_case", "as_cow"] {
        let (ok, err) =
            consumer_compiles(&format!("prelude_{selection}"), selection, "    let _ = 1;");
        assert!(
            ok,
            "a consumer can import the prelude at {selection}:\n{err}"
        );
    }
}

#[test]
fn a_method_whose_flag_is_off_is_not_there() {
    // The direction that matters, and the one a positive test cannot reach: with only
    // `append` selected, `prepend` must not exist. Before the flags selected methods, this
    // compiled, because every flag brought the whole module.
    let (ok, err) = consumer_compiles(
        "prepend_absent",
        "append",
        r#"    let _ = "a".prepend("b");"#,
    );
    assert!(
        !ok,
        "`prepend` is absent when its flag is off, so this must not compile"
    );
    assert!(
        err.contains("prepend"),
        "the error names the method that is not there:\n{err}"
    );
}

#[test]
fn the_method_whose_flag_is_on_is_there() {
    // The control for the test above. Without it, that one would pass just as well against
    // a crate where nothing compiles at all.
    let (ok, err) = consumer_compiles(
        "append_present",
        "append",
        r#"    assert_eq!("a".append("b"), "ab");"#,
    );
    assert!(ok, "`append` is present when its flag is on:\n{err}");
}

#[test]
fn join_is_absent_without_its_flag_and_present_with_it() {
    let (absent, err) = consumer_compiles(
        "join_absent",
        "append",
        r#"    let _ = "a".join(&["b"], ",");"#,
    );
    assert!(!absent, "`join` is absent when its flag is off");
    assert!(err.contains("join"), "the error names it:\n{err}");

    let (present, err) = consumer_compiles(
        "join_present",
        "join",
        r#"    assert_eq!("a".join(&["b"], ","), "a,b");"#,
    );
    assert!(present, "`join` is present when its flag is on:\n{err}");
}

#[test]
fn every_type_coercion_flag_reaches_its_method_on_its_own() {
    // `to_arc` was left out of the module's own `cfg` and out of the prelude's, so
    // selecting it alone compiled the module away and the method could not be reached from
    // anywhere. The two gates listed `as_cow` and `into_arc`, which is why the test above
    // covering `as_cow` passed while this hole stayed open.
    //
    // One case per flag rather than one for the group, because a group case would pass
    // against a gate that named any one of them.
    for (flag, body) in [
        ("as_cow", r#"    assert_eq!(&*"x".as_cow(), "x");"#),
        ("into_arc", r#"    assert_eq!(&**"x".into_arc(), "x");"#),
        ("to_arc", r#"    assert_eq!(&*"x".to_arc(), "x");"#),
    ] {
        let (ok, err) = consumer_compiles(&format!("coercion_{flag}"), flag, body);
        assert!(ok, "`{flag}` alone reaches its method:\n{err}");
    }
}

#[test]
fn a_type_coercion_method_whose_flag_is_off_is_not_there() {
    // The control. Without it the test above would pass against a crate whose three flags
    // all brought the whole module, which is the state the flag rework left the building
    // and case flags in and had to be corrected out of.
    for (selected, absent) in [("as_cow", "to_arc"), ("to_arc", "as_cow"), ("into_arc", "to_arc")] {
        let (ok, err) = consumer_compiles(
            &format!("coercion_{absent}_absent_at_{selected}"),
            selected,
            &format!(r#"    let _ = "x".{absent}();"#),
        );
        assert!(!ok, "`{absent}` is absent when only `{selected}` is on");
        assert!(err.contains(absent), "the error names `{absent}`:\n{err}");
    }
}

#[test]
fn the_allocation_axis_builds_under_every_selection() {
    // `no_std` swaps std for alloc throughout, here and in word_bounds. It is easy to break
    // without noticing, because the default selection has std and every mistake compiles
    // there: one `format!` through the wrong path, one `use std::` left behind.
    for selection in [
        "no_std",
        "no_alloc",
        "no_alloc,no_std",
        "no_std,full_format",
        "no_std,full_building",
        "no_std,full_type_coercion",
        "no_alloc,full_format,full_building,full_type_coercion",
    ] {
        let (ok, err) = check(selection);
        assert!(ok, "{selection} builds:\n{err}");
    }
}

#[test]
fn the_lending_conversion_is_absent_without_no_alloc() {
    // The direction a build check cannot reach: a feature that gates nothing passes every
    // positive test above.
    let (ok, err) = consumer_compiles(
        "write_case_absent",
        "full_format",
        r#"    let mut out = [0u8; 8];
    let _ = str_extensions::lending::write_case("a", str_extensions::lending::Case::Snake, &mut out);"#,
    );
    assert!(
        !ok,
        "`write_case` is absent when `no_alloc` is off, so this must not compile"
    );
    assert!(
        err.contains("lending"),
        "the error names the module that is not there:\n{err}"
    );
}

#[test]
fn the_lending_conversion_is_present_with_no_alloc() {
    // The control. Without it the test above would pass against a crate where nothing
    // compiles at all.
    let (ok, err) = consumer_compiles(
        "write_case_present",
        "no_alloc",
        r#"    let mut out = [0u8; 32];
    assert_eq!(
        str_extensions::lending::write_case("aB", str_extensions::lending::Case::Snake, &mut out).unwrap(),
        "a_b",
    );"#,
    );
    assert!(ok, "`write_case` is present when `no_alloc` is on:\n{err}");
}

#[test]
fn the_lending_suite_actually_runs_under_no_alloc() {
    // `tests/lending_parity.rs` is `#![cfg(feature = "no_alloc")]`, so under any other
    // selection it reports `running 0 tests`, which is what a suite that stopped compiling
    // reports too.
    let output = Command::new(env!("CARGO"))
        .args([
            "test",
            "--test",
            "lending_parity",
            "--no-default-features",
            "--features",
            "no_alloc,full_format",
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env(
            "CARGO_TARGET_DIR",
            concat!(env!("CARGO_MANIFEST_DIR"), "/target/feature-matrix"),
        )
        .output()
        .expect("cargo runs");

    let report = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "the lending suite passes:\n{report}"
    );

    let ran: usize = report
        .lines()
        .find_map(|line| line.strip_prefix("test result: ok. "))
        .and_then(|rest| rest.split(' ').next())
        .and_then(|count| count.parse().ok())
        .expect("the suite reported a result line");

    assert!(
        ran >= 5,
        "the lending suite ran {ran} cases, where it has at least 5. A suite that compiles \
         and executes nothing reports success just as loudly."
    );
}

/// Every case-conversion flag, with the method it must select.
const CASE_FLAGS: &[(&str, &str)] = &[
    (
        "to_snake_case",
        r#"    assert_eq!("aB".to_snake_case(), "a_b");"#,
    ),
    (
        "to_camel_case",
        r#"    assert_eq!("a_b".to_camel_case(), "aB");"#,
    ),
    (
        "to_pascal_case",
        r#"    assert_eq!("a_b".to_pascal_case(), "AB");"#,
    ),
    (
        "to_kebab_case",
        r#"    assert_eq!("aB".to_kebab_case(), "a-b");"#,
    ),
    (
        "to_human_readable",
        r#"    assert_eq!("aB".to_human_readable(), "a b");"#,
    ),
    (
        "to_title_case",
        r#"    assert_eq!("a_b".to_title_case(), "A B");"#,
    ),
];

#[test]
fn every_case_flag_reaches_its_method_on_its_own() {
    for (flag, body) in CASE_FLAGS {
        let (ok, err) = consumer_compiles(&format!("case_{flag}"), flag, body);
        assert!(ok, "`{flag}` alone reaches its method:\n{err}");
    }
}

#[test]
fn a_case_method_whose_flag_is_off_is_not_there() {
    // The direction that matters, and the one that was missing: `src/cases.rs` carried no
    // `#[cfg]` at all, so all six methods arrived whenever any one flag was on. Every
    // positive test passed, including the one above, because a flag that gates nothing
    // gates nothing in the direction nobody looks.
    //
    // One case per pair rather than one for the group, because a group case passes against
    // a gate naming any one of them, which is the shape that let this through twice.
    for (selected, _) in CASE_FLAGS {
        for (absent, _) in CASE_FLAGS {
            if selected == absent {
                continue;
            }
            let (ok, err) = consumer_compiles(
                &format!("case_{absent}_absent_at_{selected}"),
                selected,
                &format!(r#"    let _ = "x".{absent}();"#),
            );
            assert!(!ok, "`{absent}` is absent when only `{selected}` is on");
            assert!(err.contains(absent), "the error names `{absent}`:\n{err}");
        }
    }
}

#[test]
fn no_std_is_refused_with_a_regex_backend_rather_than_silently_walking() {
    // This asserted the opposite, and the comment it carried explained why: word_bounds
    // compiled the regex backends out under `no_std`, so the combination built and used the
    // character walker. That reads as composition and is a substitution. Different words come
    // out of the same input, with no error and nothing to grep for, and cargo unifies features
    // across a dependency graph, so the crate that asked for `no_std` is often not the crate
    // reading the result.
    //
    // Refused now, at both levels: word_bounds refuses its own pair, and this crate refuses
    // its own, because the message a reader can act on names the features they wrote.
    for selection in [
        "no_std,use_regex",
        "no_std,use_fancy_regex",
        "no_std,use_regex,use_fancy_regex",
        "no_alloc,use_regex",
        "no_std,full_format,use_regex",
    ] {
        let (ok, err) = check(selection);
        assert!(!ok, "{selection} must not build:\n{err}");
        // word_bounds' message rather than one of this crate's. A refusal here could never
        // fire: cargo builds dependencies first, so word_bounds refuses while this crate has
        // not been compiled. Its message names `cargo tree -e features`, which is how a
        // consumer finds out the backend came from this crate's defaults.
        assert!(
            err.contains("exclusive") && err.contains("cargo tree -e features"),
            "the refusal names the conflict and how to trace it:\n{err}",
        );
    }
}

#[test]
fn no_std_composes_with_everything_that_is_not_a_regex_backend() {
    // The control. Without it the test above passes on a crate where `no_std` refuses
    // everything, which is a different defect behind the same green.
    for selection in [
        "no_std",
        "no_std,full_format",
        "no_std,full_building",
        "no_std,full_type_coercion",
        "no_alloc,full_format",
    ] {
        let (ok, err) = check(selection);
        assert!(ok, "{selection} builds:\n{err}");
    }
}

#[test]
fn the_default_selection_plus_no_std_is_refused_and_says_what_to_do() {
    // The shape that surprises, spelled out: `--features no_std` on top of the defaults,
    // which include `use_regex`. Not `--no-default-features`, which is what every other case
    // here uses and is what hid the original defect. A consumer reaching for `no_std` this way
    // has not asked for a regex backend and has one, so the message has to say so.
    let output = Command::new(env!("CARGO"))
        .args(["check", "--quiet", "--features", "no_std"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env(
            "CARGO_TARGET_DIR",
            concat!(env!("CARGO_MANIFEST_DIR"), "/target/feature-matrix"),
        )
        .output()
        .expect("cargo runs");

    let err = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success(),
        "the default selection plus `no_std` must not build"
    );
    assert!(
        err.contains("exclusive"),
        "the refusal names the conflict:\n{err}",
    );
    assert!(
        err.contains("cargo tree -e features"),
        "and how to trace which crate enabled what:\n{err}",
    );
}

/// Runs the doc suite under one feature selection.
///
/// `check()` above runs `cargo check`, which does not compile doctests at all, so nothing
/// in this matrix reached them until this. The README is included as the crate's own docs,
/// so a code block in it is a doctest, and a block using a method that a narrowed selection
/// compiles out fails under that selection while every other test here passes.
fn doc_suite(features: &str) -> (bool, String) {
    let mut command = Command::new(env!("CARGO"));
    command
        .args(["test", "--doc", "--quiet", "--no-default-features"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env(
            "CARGO_TARGET_DIR",
            concat!(env!("CARGO_MANIFEST_DIR"), "/target/feature-matrix"),
        );
    if !features.is_empty() {
        command.args(["--features", features]);
    }
    let output = command.output().expect("cargo runs");
    (
        output.status.success(),
        format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ),
    )
}

#[test]
fn the_doc_suite_passes_under_every_selection_the_docs_name() {
    // Every selection this crate's own README or PR text tells a reader to run. A doctest
    // that fails under a documented command is a documented command that does not work.
    //
    // All of them carry `full_format`, and that is the predicate rather than an accident:
    // a doctest compiles under whichever selection `cargo test --doc` ran with and cannot
    // say which features it needs, so the live blocks in the README are the ones every
    // documented selection can compile. The building group is not, and its block is
    // `ignore` with `tests/building.rs` carrying those assertions instead.
    for selection in [
        "full_format,full_building,full_type_coercion",
        "no_alloc,full_format,full_building,full_type_coercion",
        "no_alloc,full_format",
        "no_std,full_format",
        "full_format",
    ] {
        let (ok, report) = doc_suite(selection);
        assert!(ok, "the doc suite passes at {selection}:\n{report}");
    }
}

#[test]
fn the_doc_suite_actually_runs_something() {
    // The control. Every assertion above would pass just as well against a crate whose
    // doctests had all been marked `ignore`, which is the cheapest way to make this green
    // and the least honest.
    let (ok, report) = doc_suite("no_alloc,full_format,full_building,full_type_coercion");
    assert!(ok, "the doc suite passes:\n{report}");

    let ran: usize = report
        .lines()
        .find_map(|line| line.strip_prefix("test result: ok. "))
        .and_then(|rest| rest.split(' ').next())
        .and_then(|count| count.parse().ok())
        .expect("the doc suite reported a result line");

    assert!(
        ran >= 3,
        "the doc suite ran {ran} doctests, where it has at least 3"
    );
}
