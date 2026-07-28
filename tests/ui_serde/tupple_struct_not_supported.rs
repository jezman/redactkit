use redactkit::RedactSerialize;

#[derive(RedactSerialize)]
struct Config(String, #[redact] String);

fn main() {}
