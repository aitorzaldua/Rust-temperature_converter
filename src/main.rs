//This program converts the temperature between Fahrenheit and Celsius
//Made with Rust by Aitor Zaldua on 2021 March 19th

//(Faherenheit -32) x 5/9 = Celsius


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

    let user_input: u32 = user_input.trim().parse().expect("Please type a number!");

    if user_input == 1 {

        println! ("Please, insert the temperature in Fahrenheit");

        let mut user_temperature = String::new();

        io::stdin()
            .read_line(&mut user_temperature)
            .expect("Failed to read line");

        let user_temperature: u32 = user_temperature.trim().parse().expect("Please type a number!");

        let converter = (user_temperature -32) * 5/9;

        println! ("They are {} Celsius", converter);


    }

    else if user_input == 2 {

        println! ("Please, insert the temperature in Celsius");

        let mut user_temperature = String::new();

        io::stdin()
            .read_line(&mut user_temperature)
            .expect("Failed to read line");

        let user_temperature: u32 = user_temperature.trim().parse().expect("Please type a number!");

        let converter = (user_temperature * 9/5) + 32;

        println! ("They are {} Fahrenheit", converter);

    }

    else  {
        println! ("Please, type 1 or 2");
    }

}