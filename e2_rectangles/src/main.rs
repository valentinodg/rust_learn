#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    //let's write a program that calculates the area of a rectangle
    //the program will take the width and height of a rectangle
    //specified in pixels and calculate the area of the rectangle

    //with variables
    let width1 = 30;
    let height1 = 50;

    println!("\nrect area (with variables) {}", area(width1, height1));
    
    //refactor with tuples
    let rect1  = (30, 50);

    println!("\nrect area (with tuples) {}", area2(rect1));

    //refactor with structs (go on top for struct def)
    let rect2 = Rectangle{
        width: 30, height: 50
    };

    println!("\nrect area (with structs) {}", area3(&rect2));

    //adding useful functionality with derived traits
    //#[derive(Debug)]
    //{:?} -> debug print
    //{:#?} -> debud pretty print
    let rectx = Rectangle{
        width: 50, height: 50
    };
    println!("\nrectx {:#?}", rectx);
}

//with variables
fn area(width: u32, height: u32) -> u32 {
    width * height
}

//refactor with tuples
fn area2(dimensions: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

//refactor with stucts
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
