//! Derive macro example.
//!
//! Run with:
//! ```bash
//! cargo run --example derive_debug
//! ```
//!
//! If default features are disabled, run with:
//! ```bash
//! cargo run --example derive_debug --features derive
//! ```

#[cfg(feature = "derive")]
fn main() {
    use redactkit::RedactDebug;

    #[derive(RedactDebug)]
    #[allow(dead_code)]
    struct Config {
        username: String,
        #[redact]
        password: String,
        #[redact]
        api_token: String,
    }

    let config = Config {
        username: "anna".to_string(),
        password: "s3cr3t".to_string(),
        api_token: "qwerty123456".to_string(),
    };

    println!("{config:?}");
}

#[cfg(not(feature = "derive"))]
fn main() {
    println!("This example requires the `derive` feature. Run with:");
    println!("  cargo run --example derive_debug --features derive");
}
