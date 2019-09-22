fn main() {
    let s1 = String::from("hello");

    //using the reference to the variable s1 as a parameter 
    //of the function makes the variable s1 
    //still available after the call
    let s2 = calculate_lenght(&s1);

    println!("\ns1: \"{}\"\nlen(s1): {}", s1, s2);

    //& -> reference operator
    //* -> dereference operator

    //references are immutable

    let mut s = String::from("hello");
    print!("\ns: \"{}\"", s);

    change(&mut s);

    println!("\nsmod: \"{}\"", s);

    //data races -> particular type of race condition
    //rust prevents data races at compile time
    //by not allowing us to have multiple mutable references
    //to a piece of data in the same scope
    
    //this rule also exists for a combination of mutable
    //and immutable references

    //multiple immutable references are allowed

    //example -> illegal
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;

    //example -> legal (2 scopes)
    //not simultaneous multiple mutable references
    let mut s3 = String::from("hello");
    {
        //second scope
        let r1 = &mut s3;
        r1.push_str(" world");
        println!("\nr1: \"{}\"", r1);
    }
    //first scope
    let r2 = &mut s3;
    r2.push_str(" by me!");
    println!("r2: \"{}\"", r2);

    //if the immutable references are no longer used 
    //you can declare a mutable reference in the same scope
    let mut s4 = String::from("hello");

    let r3 = &s4;
    let r4 = &s4;
    println!("\nr3(immut): \"{}\"\nr4(immut): \"{}\"", r3, r4);

    let r5 = &mut s4;
    r5.push_str(" world!");
    println!("r5(mut): \"{}\"", r5);

    //rust prevent dangle references at compile time
}

//have references as function parameters is called borrowing
//when we apply borrowing the own of the variable we
//passed as parameter of the function doesn't change
//therefore we are not allowed to modify 
//the value of the variable -> error
fn calculate_lenght(s: &String) -> usize {
    s.len()
}

fn change(sstring: &mut String) {
    sstring.push_str(" world!");
}
