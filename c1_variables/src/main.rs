fn main() {
    //mutability versus immutability
    //variables -> let operator
    let mut x = 5;
    println!("\nx: {}", x);
    x = 6;
    println!("x: {}", x);

    //constants -> const
    const MAX_POINTS: u32 = 100_000;
    println!("\nconst: {}", MAX_POINTS);

    //shadowing (different from mut)
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("\ny: {}", y);

    //shadowing for different types
    //(warning on prefix)
    let spaces = "    "; //4 spaces
    let spaces = spaces.len();
    println!("\nspaces: {}", spaces);
    //(warning -> fix)
    //let _spaces = spaces.len();
}
