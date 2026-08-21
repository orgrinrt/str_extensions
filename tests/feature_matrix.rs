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
        let (ok, err) = consumer_compiles(
            &format!("prelude_{selection}"),
            selection,
            "    let _ = 1;",
        );
        assert!(ok, "a consumer can import the prelude at {selection}:\n{err}");
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
