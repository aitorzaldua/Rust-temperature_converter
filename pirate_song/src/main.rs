//Enunciado:
//Recitar una porción de la canción: 99 botellas de cerveza en la pared.
//Estrofa inicial: 99 bottles of beer on the wall, 99 bottles of beer.
//                 Take one down and pass it around, 98 bottles of beer on the wall.
//Las siguientes es restar uno a los valores de las botellas.
//Las últimas serán: 1 bottle of beer on the wall, 1 bottle of beer.
//                   Take it down and pass it around, no more bottles of beer on the wall.

//                   No more bottles of beer on the wall, no more bottles of beer.
//                   Go to the store and buy some more, 99 bottles of beer on the wall.

//El usuario introduce el rango que desea:



use std::io;

//Definimos las constantes
//En este caso las estrofas fijas.

const ZERO_BOTTLES: &str = "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n";

const ONE_BOTTLE: &str = "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n";


fn main() {

    //Solicitar el rango al usuario:

    println!("Vamos a cantar la canción del pirata. \nPero es muy larga, \
    ¿Qué rango de botellas quieres que cantemos?\nIntroduce el valor superior (Entre 99 y 1, inlcuidos): ");

    let mut higher_number = String::new();

    io::stdin()
        .read_line(&mut higher_number)
        .expect("Error al leer la entrada");


    println!("Introduce el valor inferior (Entre 98 y 0, incluidos):");

    let mut lower_number = String::new();

    io::stdin()
        .read_line(&mut lower_number)
        .expect("Error al leer la entrada");


    //Pasar los valores de String a Integer:

    let higher_number: u32 = higher_number.trim().parse().expect("Esperaba un número");

    let lower_number:u32 = lower_number.trim().parse().expect("Esperaba un número");


    //Llamamos a la función de una estrofa:

   let singing_the_song = sing(higher_number, lower_number);

   println!("{}", singing_the_song);


}

//Creamos la función para el valor superior (un verso):

pub fn verse(n: u32) -> String {

    match n {
        0 => ZERO_BOTTLES.to_string(),
        1 => ONE_BOTTLE.to_string(),
        _ => format!(
            "{} bottles of beer on the wall, \
            {} bottles of beer.\nTake one down and pass it around, \
            {} bottles of beer on the wall.\n",
            n, n, n-1),
    }

}

//Creamos la función de repetición de estrofas:

pub fn sing(start: u32, end: u32) -> String {

    let mut song: String = "".to_owned();

    for n in (end..start+1).rev() {
        song.push_str(&verse(n));
        song.push_str("\n");
    }

    song.pop();

    return song;

}

