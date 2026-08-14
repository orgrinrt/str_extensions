//! The extension traits are the crate's entire public surface, and they are reachable only if
//! they are `pub`: they were `pub(super)`, so the `prelude` re-export resolved to nothing usable
//! and no external crate could call any of them. This file is an external consumer, so it
//! failing to compile is the regression.
//!
//! The assertions check reachability rather than formatting behaviour; the conversions themselves
//! are covered in `cases.rs`.

use str_extensions::prelude::*;

#[test]
fn case_conversions_are_callable_from_another_crate() {
    assert_eq!("foo bar".to_snake_case(), "foo_bar");
}

#[test]
fn string_building_is_callable_from_another_crate() {
    assert_eq!("foo".concat(&["bar"]), "foobar");
}
