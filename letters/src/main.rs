fn main() {
    let input = String::from("a   **&  cZ");

    let result = letters(input);

    println!("La cadena resultante es {}", result);
}

fn letters(input: String) -> String {

    let string_to_lower = input.to_lowercase();
    let diccionary = ["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z"];
    let mut output = String::new();

    for i in diccionary.iter(){

        if string_to_lower.find (i) != None {
            output.push_str("1");
        }
        else {
            output.push_str("0");
        }

    }


    output

}
