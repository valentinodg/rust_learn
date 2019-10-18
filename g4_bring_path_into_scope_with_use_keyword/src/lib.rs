//call functions using paths (with pub and :: operators as we did
//before) is very inconvenient
//the paths are too long and repetitive
//there's a way to simplify this process, we can bring a path into
//scope once and then call the items in that path as if they're
//local items with the use keyword

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

//adding use and a path in a scope is similar to creating a
//symbolic link in the filesystem
//by adding "use crate::front_of_house::hosting" in the crate root,
//hosting is now a valid name in that scope just as though the
//hosting module had been defined in the crate root
use crate::front_of_house::hosting;
//paths brought into scope with use also check privacy, like any
//other paths

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

//specifying a relative path with use is slighty different
//instead of starting from a name in the current scope, we must
//start the path given to use with the keyword self
mod front_of_house2 {
    pub mod hosting2 {
        pub fn add_to_waitlist2() {}
    }
}

//IMPORTANT!!!
//note that using self in this way might not be necessary in the
//future, it's an inconsistency in the language that rust
//developers are working to eliminate
use self::front_of_house2::hosting2;

pub fn eat_at_restaurant2() {
    hosting2::add_to_waitlist2();
    hosting2::add_to_waitlist2();
    hosting2::add_to_waitlist2();
}

//creating idiomatic use paths
mod front_of_house3 {
    pub mod hosting3 {
        pub fn add_to_waitlist3() {}
    }
}

//the following is the idiomatic way to bring a function into
//scope with use
//bringing the function's parent module into scope with use so
//we have to specify the parent module when calling the function
//makes it clear that the function isn't locally defined while
//still minimizing repetition of the full path
use self::front_of_house3::hosting3::add_to_waitlist3;

pub fn eat_at_restaurant3() {
    add_to_waitlist3();
    add_to_waitlist3();
    add_to_waitlist3();
}

//(let's look at src/main.rs)

//the exception to this idiom is if we're bringing 2 items
//with the same name into scope with use statements because
//rust doesn't allow that
//the following lines of code shows how to bring 2 Result types
//into scope that have the same name but different parent
//modules and how to refer to them

// use std::fmt;
// use std::io;
//
// fn function1() -> fmt::Result {}
// fn function2() -> io::Result<()> {}

//using the parent modules distinguished the 2 Result types
//if instead we specified "use std::fmt::Result" and
//"use std::io::Result", we'd have 2 Result types in the same
//scope and rust wouldn't know one we meant when we used Result

//providing new names with the as keyword
//a solution to this problem can be the use of the as keyword
//if we want to bring 2 types of the same name into the same
//scope with use, after the path, we can specify as and a new
//local name (or alias) for the type

// use std::fmt::Result;
// use std::io::Result as IoResult;
//
// fn function1() -> Result {}
// fn function2() -> IoResult<()> {}

//re-exporting names with pub use

mod front_of_house4 {
    pub mod hosting4 {
        pub fn add_to_waitlist4() {}
    }
}

//when we bring a name into scope with the use keyword, the name
//abailable in the new scope is private
//to enable the code that calls out code to refer to that name as
//if it had been defined in that code's scope we can combine pub
//and use
//this technique is called re-exporting because we're bringing an
//item into scope but also making that item available for others
//to bring into theri scope
pub use crate::front_of_house4::hosting4;
//external code can now call the "add_to_waitlist4" function using
//"hosting4::add_to_waitlist4" in its scope

pub fn eat_at_restaurant4() {
    hosting4::add_to_waitlist4();
    hosting4::add_to_waitlist4();
    hosting4::add_to_waitlist4();
}

//using external packages

//(for example rand) add rand as a dependency in Cargo.toml
//cargo will download the rand package and any dependencies from
//crates.io (now the rand package is available to our project)

//to bring rand definitions into the scope of our package add
//a use line starting with the name of the package (rand) and
//use the :: operator to list the items we want to bring into
//scope

//the std lib is also a crate that is external to our package
//the std lib is shipped with the rust language so we don't
//need to change Cargo.toml to include it
//but we need to refer to it with use to bring items from there
//into our package's scope

//(let's look at src/main.rs)
