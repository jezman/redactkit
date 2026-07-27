//! Redactor rules example: fields, custom mask, and value redaction.
//!
//! Run with:
//! ```bash
//! cargo run --example redactor_rules
//! ```

use redactkit::Redactor;

fn main() {
    let redactor = Redactor::builder()
        .field("password")
        .fields(["token", "api_key"])
        .mask("[hidden]")
        .build();

    println!(
        "password -> {:?}",
        redactor.redact_field("password", "s3cr3t")
    );
    println!(
        "token    -> {:?}",
        redactor.redact_field("token", "qwerty123456")
    );
    println!(
        "api_key  -> {:?}",
        redactor.redact_field("api_key", "secret-key")
    );
    println!(
        "username -> {:?}",
        redactor.redact_field("username", "anna")
    );

    println!(
        "redact_value(\"anything\") -> {:?}",
        redactor.redact_value("anything")
    );
}
