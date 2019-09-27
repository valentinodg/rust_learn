fn main() {
    //a crate is a binary or a library
    //the crate root is a source file that the rust compiler 
    //starts from and makes up the root module of your crate
    //section
    //a package is one or more crates that provide a set of 
    //functionality
    //a package contains a Cargo.toml file that describe how to 
    //build those crates

    //a package must contain zero or one library crates and no more
    //it can contain as many binary crates you'd like, but it must
    //contain at least one crate (either library or binary)

    //commands
    //
    // $ cargo new my-project
    //      Created binary (application) 'my-project' package
    //
    // $ ls my project
    //      Cargo.toml
    //      src
    //
    // $ ls my project/src
    //      main.rs
    
    //with "cargo new" command, cargo create a Cargo.toml file,
    //giving us a package
    //in the Cargo.toml file there is no mention of src/main.rs
    //because cargo follow a convention that src/main.rs is the
    //crate root of a binary crate with the same name as the 
    //package
    //likewise, cargo knows that if the package directory contains
    //src/lib.rs, the package contains a library crate with the same 
    //name as the package, and src/lib.rs is its crate root
    //cargo passes the crate root files to rustc to build the 
    //library or binary

    //if we have a package that only contains src/main.rs, it means
    //that it only contains a binary crate named with the same name
    //of the package
    //if we have a package that contains src/main.rs and src/lib.rs,
    //it means that it has 2 crates: a library and a binary, both
    //with the same name as the package
    //a package can have multiple binary crates by placing files in 
    //the src/bin directory: each file will be a separate binary
    //crate

    //a crate will group related functionality together in a scope
    //so the functionality is easy to share between multiple projects
    //(for example we can bring the rand crate into our project's
    //scope to access the random number generation functionality 
    //that the rand crate provides)ù

    //rust prevents potential conflicts during this operation
    //a crate's functionality is namespaced in its own scope
    //(for example if we add rand as a dependency (that has a 
    //trait named Rng) and we define a struct also named Rng,
    //we can access both the functionalities by using rand::Rng and 
    //Rng respectively)
}
