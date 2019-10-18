fn main() {
    //unrecoverable errors with panic!

    //for unrecoverable errors rust has the panic! macro
    //when the panic! macro executes your program will print a failure message, unwind and clean up
    //the stack and then quit
    //this most commonly occurs when a bug of some kind has been detected and it's not clear to the
    //programmer how to handle the error

    //unwinding the stack or aborting in response to a panic

    //by default, then a penic occurs the program starts unwinding which means tust walks back up
    //the stack and cleans up the data from each function it encouters, but this walking back and
    //clean up is a lot of work
    //the alternative is to abort which ends the program without cleaning up (memory that the
    //program was using will then need to be cleaned up by the os)

    //if in your project you need to make the resulting binary as small as possible you can switch
    //from unwinding to aborting upon a panic by adding the following lines of code in your
    //Cargo.toml file
    //
    //Cargo.toml
    //
    //[profile.release]
    //panic = 'abort'

    //let's try calling panic! in a simple program

    //panic!("crash and burn");

    //the call to panic! causes the error message container in the last 2 lines
    //the 1 line shows out panic message and the place in our source code where the panic occurred
    //(the panic! code might be in code that our code calls, in this case look also at the filename
    //indicated before the line and the column in which the panic! occurs)

    //we can use the backtrace of the functions the panic! call came from to figure out the part of
    //our code that is causing  the problem

    //using a panic! backtrace

    //let's look at another example to see what it's like when a panic! call comes from a library
    //because of a bug in our code instead of from our code calling the macro directly

    let v = vec![1, 2, 3];

    v[99];
    //this particular situation may cause a buffer overread and can lead to security
    //vulnerabilities in languages like C
    //the C language, will attempt to give you exactly what you asked for in this situation,
    //even though it isn't what you want: you'll get whathever is at the location in
    //memory that would correspond to that element in the vector, even though the memory doesn't
    //belong to the vector
    //to protect your program from this sort of vulnerabilities, if you try to read an element at
    //an index that doesn't exist, rust will stop execution and refuse to continue

    //this error points at a file we didn't write -> libcore/slice/mod.rs
    //that's the implementation of slice in the rust source code
    //the code that gets run when we use [] on our vector v is in libcore/slice/mod.rs and that is
    //where the panic! is actually happening

    //the next note line tells us that we can set the RUST_BACKTRACE environment variable to get a
    //backtrace of exactly what happened to cause the error
    //a back trace is a list of all the functions that have been called to get to this point
    //backtraces in rust work as they do in other languages: the key to reading the backtrace is to
    //start from the top and read until you see files you wrote

    //let's try getting a backtrace by setting the RUST_BACKTRACE environment variable to any
    //value exept 0
    //--> RUST_BACKTRACE=1 cargo run

    //the exact output you see might be different depending on your os and rust version
    //in order to get backtraces with this information, debug symbols must be enabled (they are
    //enabled by default when using cargo build or cargo run without the --release flag

    //in the output, line 14 of the backtrace points to the line in our porkect that's causing the
    //problem: line 46 of src/main.rs (the number of the line may be different)
    //the way to fix the panic is to not request an element at index 99 from a vector that only
    //contains 3 items
}
