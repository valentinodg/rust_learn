//one the other hand, when bringing in structs, enums and
//other items with use, it's idiomatic to specify the full path
//(for example let's show the idiomatic way to bring the std lib's
//HashMap struct into the scope of a binary crate)
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    for item in map.iter() {
        println!("{:?}", item);
    }
}

//using nested paths to clean up large use lists
//if we're using multiple items defined in the same package or
//same module listing each item on its own line can take up a
//lot of vertical space in our files

//(for example)
//use std::io;
//use std::cmp::Ordering;

//instead we can use nested paths to bring the same items into
//scope in one line
//we do this by specifying the common part of the path followed
//by :: and then {} around a list of the parts of the paths that
//differ

// use std::{io, cmp::Ordering};

//we can use a nested path at any level in a path, which is useful
//ehn combining 2 use statements that share a subpath

//(for example)
//use std::io;
//use std::io::Write;

//the common part of these 2 paths is std::io
//to merge these 2 paths into one use statement we can use self
//in the nested path

// use std::io::{self, Write};

//the glob operator
//if we want to bring all public items defined in a path into scope
//we can specify that path followed by the glob operator *

// use std::collections::*;

//be careful when using the glob operator!
