fn main() {
    //string slices
    //a string slice is a reference to a part of a string
    let s = String::from("hello world");

    //&string[start..end]
    //start index is included
    //end index is excluded (declare 1 more than 
    //the last position in the slice)
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("hello: \"{}\"\nworld: \"{}\"", hello, world);

    //operator .. -> rust's range syntax
    //you can drop initial value if you want to start at the 
    //first index (zero)
    let hello2 = &s[..5];
    
    //or if you want you can drop the last (also using len)
    let len = s.len();

    let world2 = &s[6..len];
    let world3 = &s[6..];

    //you can also drop both values to take a slice of the
    //entire string
    let all = &s[..];
    //let all = &s[0..len]

    println!("\nhello2: \"{}\"\nworld2: \"{}\"\nworld3: \"{}\"\nall: \"{}\"", 
    hello2, world2, world3, all);

    //test function that return the first world of a sentence
    //using string slices
    let teststring = String::from("hello world by me!");
    let first = first_word(&teststring);
    println!("\ntest: \"{}\"\nfirst: \"{}\"", teststring, first);
    
    let teststring2 = String::from("helloworld byme!");
    let first2 = first_word(&teststring2);
    println!("\ntest: \"{}\"\nfirst: \"{}\"", teststring2, first2);

    let teststring3 = String::from("helloworldbyme!");
    let first3 = first_word(&teststring3);
    println!("\ntest: \"{}\"\nfirst: \"{}\"", teststring3, first3);

    //string literals are slices
    //s is a &str, a slice that point to that specific point
    //of the binary
    //string literals immutables, &str immutable reference
    let _s = "hello world";

    //string slices as parameters
    //let's create a new modified version of the first_world
    //function
    //new signature -> 
    //fn mod_first_world(s: &str) -> &str {

    let my_s = String::from("new string");
    let word = mod_first_word(&my_s[..]);
    println!("\nword: \"{}\" -> (string slice as parameter)", word);

    let my_sl = "new string";
    let word = mod_first_word(&my_sl[..]);
    println!("word: \"{}\" -> (string literal slice as parameter)", word);

    let word = mod_first_word(my_sl);
    println!("word: \"{}\" -> (string literal as parameter)", word);

    //other types of slices
    //array slices
    let a = [1,2,3,4,5,6,7,8,9,10];
    let slice = &a[1..9];
    
    print!("\nog array: [ ");
    for item in a.iter() {
        print!("{} ", item);
    }
    println!("]");

    print!("slice: [ ");
    for item in slice.iter() {
        print!("{} ", item);
    }
    println!("]");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn mod_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}