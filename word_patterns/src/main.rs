fn main() {
    let word = String::from("hello");

    let result = word_pattern(word);

    println!("la respuesta es {}", result);

}


fn word_pattern(word: String) -> String {

    let string_to_lower = word.to_lowercase();
    let mut vector_word = vec![];
    let mut counter = 0;


    println!("El Vector es: {:?}", vector_word);

    let result = String::from("return");

    result

}

