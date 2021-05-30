fn main() {

let mut string_total = String::from("Hello Rust!");

let first_word_length = first_word(&string_total);

println! ("la primera palabra es: {}", first_word_length);

string_total.clear();

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {

        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

