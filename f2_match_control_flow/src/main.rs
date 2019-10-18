#[derive(Debug)]
enum UsState {
    Alabama,
    _Alaska,
}

enum Coin {
    Penny,
    _Nickel,
    _Dime,
    //another useful feature of match arms is that we can bind to
    //he parts of the values that match the pattern (this is how
    //we can extract values out of enum variants)
    Quarter(UsState),
}

fn main() {
    //match -> extremely powerful control flow operator
    //the match operator allow us to compare a value against a
    //series of patterns and then execute code based on which
    //pattern matches
    //patterns can be made up of literals, variables, wildcards, etc
    //the compiler can confirm that all possible cases are handles
    let coin1 = Coin::Penny;
    println!("coin1: {}", value_in_cents(coin1));

    println!("");

    let state = UsState::Alabama;
    let coin2 = Coin::Quarter(state);
    println!("coin2: {}", value_in_cents(coin2));

    //matching with Option<T>
    //we can use match to handle and compare Option<T> values by type
    let _five = Some(5);
    let _six = plus_one(_five); //fn returns 6
    let _none = plus_one(None); //fn returns None

    //the _ placeholder
    //we can use the _ pattern when we don't want to list all
    //possible values
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        //the _ pattern will match any value/case (from 0 to 255 in this
        //case) that aren't specified BEFORE it
        //the () is just the unit value, so nothing will happen in
        //the _ case
        //the match expression can be a bit wordy in a situation in
        //which we care about only one of the cases, for this situation
        //rust provides if let
        _ => (),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    //match expressions are similar to if expressions but not equal
    //if expressions needs to return a bool type, match expressions
    //can return any type
    match coin {
        //here we define the match arms
        //an arm has 2 parts: a pattern and some code
        //the => operator separes the 2 parts
        //the , separates the arms
        //the code associated with each arm is an expression and
        //the resulting value of the expression in the matching
        //arm is the value that gets returned for the entire match
        //expression

        //{} are used if there are multiple lines of code
        Coin::Penny => {
            println!("lucky penny!");
            1
        }
        Coin::_Nickel => 5,
        Coin::_Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {:?}", state);
            25
        }
    }
}

//matches are exhaustive and if (for example) avoid to put within the
//match cases the "None case", the compiler won't compile
//rust knows that we didn't cover every possible case and even knows
//which pattern we forgot
//we must exhaust every last possibility in order for the code to be
//valid (especially in the case of Option<T>)
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
