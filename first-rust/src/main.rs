fn main() {}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (idx, &value) in bytes.iter().enumerate() {
        if value == b' ' {
            return &s[0..idx];
        }
    }
    &s[..]
}
