//This program converts the temperature between Fahrenheit and Celsius
//Made with Rust by Aitor Zaldua on March 19th

//(Faherenheit -32) x 5/9 = Celsius

//Steps:
//Ask to the user to insert the Fahrenheit
//Create the variables F and C
//Store the result
//Print the result

use std::io;


fn main () {
    println! ("TEMPERATURE CONVERTER");
    println! ("Please, insert:");
    println! ("1 - To convert Fahrenheit to Celsius");
    println! ("2 - To convert Celsius to Fahrenheit");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    println!("Your selecction: {}", user_input);

    let user_input: u32 = user_input.trim().parse().expect("Please type a number!");

    if user_input == 1 {
        println! ("Lets convert F to C");
    }
    else if user_input == 2 {
        println! ("Lets convert C to F");
    }
    else  {
        println! ("Please, type 1 or 2");
    }

}