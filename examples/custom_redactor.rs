//! Custom redactor example using the default redactor plus extra rules.
//!
//! Run with:
//! ```bash
//! cargo run --example custom_redactor
//! ```

use redactkit::{Redactor, default_redactor};

fn main() {
    // Start from the default redactor and extend it.
    let base = default_redactor();

    println!("Default rules:");
    println!(
        "  password -> {:?}",
        base.redact_field("password", "s3cr3t")
    );
    println!(
        "  api_key  -> {:?}",
        base.redact_field("api_key", "qwerty123456")
    );
    println!("  username -> {:?}", base.redact_field("username", "anna"));

    // Build a fully custom redactor.
    let custom = Redactor::builder()
        .fields(["session_id", "cookie", "x_auth_token"])
        .mask("<redacted>")
        .build();

    println!("\nCustom rules:");
    println!(
        "  session_id   -> {:?}",
        custom.redact_field("session_id", "qwerty123456")
    );
    println!(
        "  cookie       -> {:?}",
        custom.redact_field("cookie", "sid=xyz")
    );
    println!(
        "  x_auth_token -> {:?}",
        custom.redact_field("x_auth_token", "token")
    );
    println!(
        "  host         -> {:?}",
        custom.redact_field("host", "localhost")
    );
}
