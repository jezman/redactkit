//! Basic redactor example.
//!
//! Run with:
//! ```bash
//! cargo run --example basic
//! ```

use redactkit::Redactor;

fn main() {
    let redactor = Redactor::builder().field("password").build();

    println!(
        "should_redact_field(\"password\") = {}",
        redactor.should_redact_field("password")
    );
    println!(
        "should_redact_field(\"username\") = {}",
        redactor.should_redact_field("username")
    );

    println!(
        "redact_field(\"password\", \"s3cr3t\") = {:?}",
        redactor.redact_field("password", "s3cr3t")
    );
    println!(
        "redact_field(\"username\", \"anna\") = {:?}",
        redactor.redact_field("username", "anna")
    );
}
