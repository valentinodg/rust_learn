//function declared through the fn keyword
//snake code syntax -> my_function(), let my_variable, ...
//the function declaration can be found
//anywhere in the program

//statement are istructions that perform some action
//and don't return a value
//INCLUDE ENDING SEMICOLON
//example: let x = (let y = 5); -> error
//(in ruby or c you can write x = y = 6)
//(statements -> create/assign a variable/value,
//define a function, ...)

//expression evaluate to a resulting value
//NOT INCLUDE ENDING SEMICOLON
//rust is an expression-based language
//(expressions -> call a function, call a macro, {} block, ...)

//return value of a function -> return keyword
//all functions return implicitly the value of the
//final expression in the block of the body
//you must declare the type of the return value
//with -> operator
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
    //or (without semicolon)
    // x + 1
}

//##### MAIN FUNCTION #####
fn main() {
    function1();
    function2(5, 6);
    println!("\nfive(): {}", five());
    println!("\nplus_one: {}", plus_one(5));
}

fn function1() {
    println!("\nfunction1");
}

//in function signatures you must declare
//the type of each parameter
fn function2(x: i32, y: i32) {
    println!("\nfunction2 param x: {}", x);
    println!("function2 param y: {}", y);
}
