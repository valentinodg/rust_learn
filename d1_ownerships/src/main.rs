fn main() {
    //scope 
    //valid from the declaration point
    //to the end of the current scope

    // stack vs heap

    //error -> string literal is immutable
    //because is stored on the stack
    let /*mut*/ r = "hello";
    println!("\nr: \"{}\" (string literal -> immutable (allocated on the stack)", r);

    //string data type introduction (discussed later)
    //strings are stored on the heap (not on the stack)
    
    //create a string from a string literal
    //using String type and from() function
    //String is a mutable type because it is stored on the heap
    //dynamic allocation of memory
    let mut s = String::from("hello");
    //the :: operator allow us to namespace the from() function
    //under the String type
    s.push_str(" world");
    println!("s: \"{}\" (String type -> mutable (allocated on the heap)", s);

    //in rust the memory is automatically returned
    //to the os once the variable that owns it goes out of scope

    //allocate and free methods aren't necessary
    //rust call automatically the method drop() to return
    //the memory previously allocated to the os
    //(like RAII patterns in C++)

    //variable interaction -> move
    //(stack variables)
    let x = 5;
    let y = x;
    println!("\nx: {}, y: {}", x, y);
    //(heap variables)
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2: {}", s2);
    //"move" (as shallow copy and not as deep copy)
    //rust avoid double free error
    //we can't use s1 after the move (it becomes not valid)
    //so s2 is the only valid variable after the move
    //when s2 goes out of the scope rust calls the drop
    //method and frees the memory (only 1 time, double free avoided)

    //rust will never automatically create deep copies
    //for runtime performance reanson (only valid for heap types)
    //for stack types rust performs a deep copy

    //variable interaction -> clone
    //shadowing
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("\ns1: {}, s2: {}", s1, s2);

    //after we see the Copy trait that allow us to use an old 
    //variable after an assignment (only valid for stack types)
    //we also see the Drop trait 

    //variable scope, ownership and functions
    println!("");

    let s = String::from("hello");
    takes_ownership(s);
    
    let x = 5;
    makes_copy(x);

    //return values and scope
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    //assign s2 to s3 moves s2 (drop)
    //s2 is no more available
    let s3 = takes_and_gives_back(s2);

    println!("\ns1: {}, s3: {}", s1, s3);

    //return values back to use again
    //s1 arg of s2 so no more available
    let s1 = String::from("hello");
    let (s2,len) = calculate_lenght(s1);
    //we can use s2 again because we have created
    //a function that returns it back 
    println!("\nthe lenght of '{}' is {}", s2, len);

    //we will see a rust feature that is used to manage
    //this kind of situations -> references
}


fn takes_ownership(some_string: String){
    println!("takes_ownership(\"{}\")", some_string);
}

fn makes_copy(some_integer: i32){
    println!("makes_copy({})", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_lenght(s: String) -> (String, usize){
    let lenght = s.len();
    (s,lenght)
}