#![cfg(feature = "derive")]

use redactkit::RedactDebug;

#[derive(RedactDebug)]
struct Config {
    username: String,

    #[redact]
    password: String,
}

#[test]
fn debug_hides_redacted_field_and_shows_normal_field() {
    let config = Config {
        username: "anna".to_string(),
        password: "s3cur3".to_string(),
    };

    let debug = format!("{config:?}");

    assert!(debug.contains("username"));
    assert!(debug.contains("anna"));

    assert!(debug.contains("password"));
    assert!(debug.contains("******"));

    assert!(!debug.contains("s3cur3"));
}

#[derive(RedactDebug)]
struct MultipleSecrets {
    username: String,

    #[redact]
    password: String,

    #[redact]
    api_token: String,
}

#[test]
fn debug_hides_multiple_redacted_fields() {
    let config = MultipleSecrets {
        username: "dmitry".to_string(),
        password: "t3rc3s".to_string(),
        api_token: "qwerty123456".to_string(),
    };

    let debug = format!("{config:?}");

    assert!(debug.contains("dmitry"));

    assert!(debug.contains("******"));

    assert!(!debug.contains("t3rc3s"));
    assert!(!debug.contains("qwerty123456"));
}

#[derive(RedactDebug)]
#[allow(dead_code)]
struct NoSecrets {
    host: String,
    port: u16,
}

#[test]
fn debug_shows_all_fields_when_no_redact() {
    let config = NoSecrets {
        host: "localhost".to_string(),
        port: 5432,
    };

    let debug = format!("{config:?}");

    assert!(debug.contains("host"));
    assert!(debug.contains("localhost"));

    assert!(debug.contains("port"));
    assert!(debug.contains("5432"));

    assert!(!debug.contains("******"));
}
