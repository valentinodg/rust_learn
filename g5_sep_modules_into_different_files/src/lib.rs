//when modules get large you might want to move their
//definitions to a separate file to make the code easier
//to navigate

//let's move the front_of_house module to its own file
//src/front_of_house.rs by changing the crate root file
//(this procedure also works with binary crates whose
//crate root file is src/main.rs, in this case instead
//the crate root is src/lib.rs)

//using a ; adter "mod front_of_house" rather than using
//a block tells rust to load the contents of the module
//from another file with the same name as the module
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

//let's now extract the hosting2 module to its own file
//(let's create a front_of_house2 file)
mod front_of_house2;

//lib.rust file
//front_of_house2 file
//front_of_house2 directory
//  hosting.rs file

//the module tree remains the same and the function
//calls in eat_at_restaurant2 will work even though
//the definitions live in different files
//this technique lets you move modules to new files as
//they grow in size
pub use crate::front_of_house2::hosting2;

pub fn eat_at_restaurant2() {
    hosting2::add_to_waitlist2();
    hosting2::add_to_waitlist2();
    hosting2::add_to_waitlist2();
}

//note that the pub use crate::front_of_house2::hosting2
//statement in src/lib.rs also hasn't changed, nor does
//use have any impact on what files are compiled as part
//of the crate
//the mod keyword declares modules and rust looks in a
//file with the same name as the module for the code
//that goes into that module
