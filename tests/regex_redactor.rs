#![cfg(feature = "regex")]

use redactkit::Redactor;

#[test]
fn field_pattern_matches_case_insensitively() {
    let redactor = Redactor::builder()
        .field_pattern("(?i)token|secret")
        .unwrap()
        .build();

    assert!(redactor.should_redact_field("api_token"));
    assert!(redactor.should_redact_field("CLIENT_SECRET"));
    assert!(!redactor.should_redact_field("username"));
}

#[test]
fn value_pattern_replaces_digits() {
    let redactor = Redactor::builder()
        .value_pattern(r"\d{4}", "****")
        .unwrap()
        .build();

    assert_eq!(
        redactor.redact_field("note", "card 1234 ok"),
        "card **** ok"
    );
}

#[test]
fn invalid_pattern_returns_error() {
    let result = Redactor::builder().field_pattern("(unclosed");

    assert!(result.is_err());
}

#[test]
fn exact_field_and_regex_pattern_work_together() {
    let redactor = Redactor::builder()
        .field("password")
        .field_pattern("(?i)token")
        .unwrap()
        .build();

    assert!(redactor.should_redact_field("password"));
    assert!(redactor.should_redact_field("api_token"));
    assert!(!redactor.should_redact_field("usename"));
}

#[test]
fn value_pattern_does_not_change_unmatched_value() {
    let redactor = Redactor::builder()
        .value_pattern(r"\d{4}", "****")
        .unwrap()
        .build();

    assert_eq!(
        redactor.redact_field("note", "no digits here"),
        "no digits here"
    );
}
