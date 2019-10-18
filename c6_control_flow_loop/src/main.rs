fn main() {
    //loops with loop statement
    let mut x: i32 = 0;
    loop {
        println!("x: {}", x);
        x = x + 1;

        if x > 5 {
            break;
        }
    }

    println!();

    //loop as the right part of a statement
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            //returned value, after break expression
            break counter * 2;
        }
    };
    println!("result: {}", result);

    println!();

    //loop with while statement
    let mut number = 3;

    while number != 0 {
        println!("n: {}", number);
        number -= 1;
    }
    println!("n: {}\n... listoff", number);

    println!();

    //while loop to interate over the elements of a collection
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("a[{}]: {}", index, a[index]);
        index += 1
    }

    println!();

    //for loop to interate over the elements of a collection
    let b = [10, 20, 30, 40, 50];

    for element in b.iter() {
        println!("{}", element);
    }

    println!();

    //loop with for statement using Range and rev
    //the Range type is provided by the std lib and serves to
    //generate numberical sequences
    //(1..4) -> 1 included, 4 excluded
    //the rev method serves to reverse a range
    for countdown in (1..4).rev() {
        println!("{}", countdown);
    }
    println!("...listoff");
}
