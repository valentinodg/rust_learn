fn main() {
    //if expressions and branches
    //the condition of a if statement must be a bool
    //rust will not automatically convert non-bool types
    //in bool types (unlike ruby or javascript for example)

    let number = 3;

    if number < 5 {
        //true arm
        println!("condition -> true {}", number);
    } else
    /*optional*/
    {
        //false arm
        println!("condition -> false {}", number);
    }

    //single if
    if number != 0 {
        println!("number is not zero -> true {}", number);
    }

    let another_number = 17;

    //if, else if, else
    //many else if expression can clutter your code, so in this
    //case you might use the match construct
    if another_number % 4 == 0 {
        println!(
            "another_number is divisible by 4 -> true {}",
            another_number
        );
    } else if another_number % 3 == 0 {
        println!(
            "another_number is divisible by 3 -> true {}",
            another_number
        );
    } else if another_number % 2 == 0 {
        println!(
            "another_number is divisible by 2 -> true {}",
            another_number
        );
    } else {
        println!("another_number is not divisible by 4,3,2");
    }

    //if expression as right part of a statement
    let condition = true;
    let variable = if condition { 5 } else { 6 };
    println!("variable value: {}", variable);
}
