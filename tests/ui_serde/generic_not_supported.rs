use redactkit::RedactSerialize;

#[derive(RedactSerialize)]
struct Config<T> {
    value: T,
}

fn main() {}
