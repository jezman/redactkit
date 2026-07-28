//! Example: redacting sensitive fields during serialization.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example serde_serialize --features serde
//! ```

#[cfg(feature = "serde")]
fn main() {
    use redactkit::{RedactDebug, RedactSerialize};

    #[derive(RedactDebug, RedactSerialize)]
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
        api_token: "s3cr3t-t0k3n".to_string(),
    };

    println!("Debug output:");
    println!("{config:?}");
    println!();

    let json = serde_json::to_string_pretty(&config).expect("serialization should succeed");

    println!("Serialized output:");
    println!("{json}");
}

#[cfg(not(feature = "serde"))]
fn main() {
    println!("This example requires the `serde` feature.");
    println!();
    println!("Run:");
    println!("  cargo run --example serde_serialize --features serde");
}
