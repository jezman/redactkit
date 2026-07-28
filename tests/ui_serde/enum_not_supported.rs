use redactkit::RedactSerialize;

#[derive(RedactSerialize)]
enum Config {
    Username,
    Password,
}

fn main() {}
