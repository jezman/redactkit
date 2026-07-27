//! Example: redacting tracing fields with redactkit.
//!
//! Run with:
//! ```bash
//! cargo run --example tracing_fmt --features tracing
//! ```

#[cfg(feature = "tracing")]
fn main() {
    use redactkit::tracing::redact_fields;

    tracing_subscriber::fmt()
        .fmt_fields(redact_fields().field("session_id"))
        .init();

    tracing::info!(
        user = "anna",
        password = "s3cr3t",
        session_id = "qwerty123456"
    );
    tracing::warn!(user = "dmitry", api_key = "secret-key", host = "localhost");
}

#[cfg(not(feature = "tracing"))]
fn main() {
    println!("This example requires the `tracing` feature. Run with:");
    println!("  cargo run --example tracing_fmt --features tracing");
}
