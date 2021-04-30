struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {


    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    let area = calcular_area(&rect1);

    println! ("el area es {}", area);
}

fn calcular_area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}
