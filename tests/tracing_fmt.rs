#![cfg(feature = "tracing")]

use std::io;
use std::sync::{Arc, Mutex, MutexGuard};

use redactkit::tracing::RedactFields;
use tracing_subscriber::fmt::MakeWriter;

/// Shared buffer used as a tracing writer.
#[derive(Clone)]
struct SharedBuf(Arc<Mutex<Vec<u8>>>);

impl<'a> MakeWriter<'a> for SharedBuf {
    type Writer = BufWriter<'a>;

    fn make_writer(&'a self) -> Self::Writer {
        BufWriter(self.0.lock().unwrap())
    }
}

/// Writer that writes into a locked buffer.
struct BufWriter<'a>(MutexGuard<'a, Vec<u8>>);

impl io::Write for BufWriter<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

/// Runs `action` with a tracing subscriber that writes into a buffer,
/// then returns the captured output as a string.
fn capture<F>(fields: RedactFields, action: F) -> String
where
    F: FnOnce(),
{
    let buf = Arc::new(Mutex::new(Vec::new()));
    let writer = SharedBuf(buf.clone());

    let subscriber = tracing_subscriber::fmt()
        .fmt_fields(fields)
        .with_writer(writer)
        .with_ansi(false)
        .without_time()
        .with_target(false)
        .finish();

    tracing::subscriber::with_default(subscriber, action);

    let data = buf.lock().unwrap().clone();
    String::from_utf8(data).unwrap()
}

#[test]
fn redacts_default_password_field() {
    let output = capture(redactkit::tracing::redact_fields(), || {
        tracing::info!(user = "anna", password = "s3cr3t");
    });

    assert!(output.contains("user=\"anna\""));
    assert!(output.contains("password=\"******\""));
    assert!(!output.contains("s3cr3t"));
}

#[test]
fn redacts_custom_field() {
    let fields = redactkit::tracing::redact_fields().field("session_id");

    let output = capture(fields, || {
        tracing::info!(user = "dmitry", session_id = "qwerty123456");
    });

    assert!(output.contains("user=\"dmitry\""));
    assert!(output.contains("session_id=\"******\""));
    assert!(!output.contains("qwerty123456"));
}

#[test]
fn custom_mask_is_used() {
    let fields = redactkit::tracing::redact_fields()
        .field("password")
        .mask("[hidden]");

    let output = capture(fields, || {
        tracing::info!(password = "secret");
    });

    assert!(output.contains("password=\"[hidden]\""));
    assert!(!output.contains("secret"));
}

#[test]
fn non_sensitive_fields_are_not_redacted() {
    let output = capture(redactkit::tracing::redact_fields(), || {
        tracing::info!(host = "localhost", port = 5432);
    });

    assert!(output.contains("host=\"localhost\""));
    assert!(output.contains("port=5432"));
    assert!(!output.contains("******"));
}

#[cfg(feature = "regex")]
#[test]
fn field_pattern_redacts_matching_fields() {
    let fields = redactkit::tracing::redact_fields()
        .field_pattern("(?i)token")
        .unwrap();

    let output = capture(fields, || {
        tracing::info!(user = "anna", api_token = "foobar");
    });

    assert!(output.contains("user=\"anna\""));
    assert!(output.contains("api_token=\"******\""));
    assert!(!output.contains("foobar"));
}

#[cfg(feature = "regex")]
#[test]
fn value_pattern_replaces_matched_value_parts() {
    let fields = redactkit::tracing::redact_fields()
        .value_pattern(r"\d{6}", "****")
        .unwrap();

    let output = capture(fields, || {
        tracing::info!(note = "card 123456 ok");
    });

    assert!(output.contains("note=\"card **** ok\""));
    assert!(!output.contains("123456"));
}
