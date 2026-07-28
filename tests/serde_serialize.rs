#![cfg(feature = "serde")]

use redactkit::RedactSerialize;

#[derive(RedactSerialize)]
struct Config {
    username: String,
    #[redact]
    password: String,
}

#[derive(RedactSerialize)]
struct NoSecrets {
    host: String,
    port: u16,
}

#[test]
fn redacts_marked_fields() {
    let config = Config {
        username: "anna".to_string(),
        password: "s3cr3t".to_string(),
    };

    let json = serde_json::to_string(&config).expect("serialization should succeed");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid JSON");

    assert_eq!(value["username"], "anna");
    assert_eq!(value["password"], "******");
    assert!(!json.contains("hunter2"));
}

#[test]
fn serializes_unmarked_fields_as_is() {
    let config = NoSecrets {
        host: "localhost".to_string(),
        port: 5432,
    };

    let json = serde_json::to_string(&config).expect("serialization should succeed");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid JSON");

    assert_eq!(value["host"], "localhost");
    assert_eq!(value["port"].as_u64(), Some(5432));
}
