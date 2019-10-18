#[derive(Debug)]
enum Coin {
    Quarter(State),
    _Nickel,
}
#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
}

fn main() {
    //the if let syntax lets you combine if and let into
    //a less verbose way to handle values that match one
    //pattern while ignoring the rest
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    //we can write the same check in a shorter way using
    //if let
    //the syntax if let takes a pattern and an expression
    //separated by an equal sign
    //using if let means less typing, less indentation and
    //less boilerplate code (but you lose the exhaustive
    //checking that match enforces)
    //if let is a sort of syntax sugar for match that runs
    //code when the value matches one pattern and then
    //ignores all other values
    if let Some(3) = some_u8_value {
        println!("three");
    }
    //we can include an else with an if let (the else block
    //is the same as the block of that would fo with the _
    //case in the match expression that is equivalent to the
    //if let and else)

    let xstate = State::Alabama;
    let coin1 = Coin::Quarter(xstate);

    //match expression
    match coin1 {
        Coin::Quarter(state) => println!("state quarter from {:?}", state),
        _ => println!(""),
    }

    let ystate = State::Alaska;
    let coin2 = Coin::Quarter(ystate);

    //if let expression
    if let Coin::Quarter(state) = coin2 {
        println!("state quarter from {:?}", state);
    } else {
        println!("");
    }
}
