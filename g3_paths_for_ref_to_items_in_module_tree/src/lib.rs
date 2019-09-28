//to show rust where to find an item in a module tree we use a path
//in the same way we use a path when navigating a filesystem
//for example if we want to call a function we need to know its path

//a path can take 2 forms
//absolute path: it starts from a crate root by using a crate name
//of a literal crate
//relative path: it starts from the current module and uses self,
//super or an identifier in the current module

//both absolute and relative paths are followed by 1 or more 
//identifiers separated by double colons ::

//let's show 2 ways to call the "add_to_waitlist" function from a new
//function "eat_at_restaurant" defined in the crate root
//the "eat_at_restaurant" function is part of our library crate's 
//public API so we mark it with the pub keyword

//exposing paths with the pub keyword

//modules are useful to organize your code and they also define 
//rust's privacy boundary: the line that encapsulates the 
//implementation details external code isn't allowed to know about,
//call or rely on.
//if you want to make an item like a function or struct private
//you put it in a module

//The way privacy works in Rust is that all items (functions, methods, 
//structs, enums, modules, and constants) are private by default.
//items in a parent module can’t use the private items inside child modules, 
//but items in child modules can use the items in their ancestor modules.
//the reason is that child modules wrap and hide their implementation details, 
//but the child modules can see the context in which they’re defined.

//we can refer to front_of_house (that is not pub) from 
//eat_to_restaurant because the 2 modules are siblings (they are)
//defined in the same module
mod front_to_house {
    pub mod hosting {
        pub fn add_to_waitlist()  {}
    }
}

pub fn eat_at_restaurant() {
    //absolute path
    //we use the word "crate" because the add_to_waitlist function
    //is defined in the same crate as eat_at_restaurant
    crate::front_to_house::hosting::add_to_waitlist();
    //relative path
    //the path starts with front_to_house that is the name of the 
    //module defined at the same level of the module tree as 
    //eat_at_restaurant
    front_to_house::hosting::add_to_waitlist();

    //relative vs absolute paths
    //this is a design choice but our preference is to specify absolute
    //paths because it's more likely to move code definitions and item 
    //calls independently of each other 
}

//starting relative paths with super 

//We can also construct relative paths that begin in the 
//parent module by using super at the start of the path. 
//this is like starting a filesystem path with the .. syntax.
fn serve_order() {}

mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    pub fn cook_order() {}
}

pub fn incorrect_order() {
    back_of_house::cook_order();
    back_of_house::fix_incorrect_order();
}

//making structs and enums public

//We can also use pub to designate structs and enums as public, 
//but there are a few extra details. If we use pub before a struct 
//definition, we make the struct public, but the struct’s fields 
//will still be private. We can make each field public or not on a 
//case-by-case basis.

mod back_of_house2 {
    pub struct Breakfast {
        pub toast: String,
        _fruit: String
    }
    //note that because back_of_house2::Breakfast has a private
    //field, the struct needs to provide a public associated
    //function that constructs an instance of Breakfast
    //if Breakfast didn't have such a function, we couldn't create 
    //an instance of Breakfast in eat_at_restaurant2 because we
    //couldn't see the value of the private fruit field in 
    //eat_at_restaurant2
    impl Breakfast {
        pub fn summer(_toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from("cheese toast"),
                _fruit: String::from("peaches")
            }
        }
    }
}

pub fn eat_at_restaurant2(){
    let mut meal = back_of_house2::Breakfast::summer("rye");
    //let's change our mind about what bread we'd like
    meal.toast = String::from("wheat");
    println!("i'd like {} toast please", meal.toast);

    //we are not allowed to see or modify the fruit that comes
    //with the meal (the next line won't compile if we uncomment it)
    //meal.fruit = String::from("blueberries");
}

//in contrast, if we make an enum public, all of its variants are
//then public

mod back_of_house3 {
    pub enum Appetizer {
        Soup,
        Salad
    }
}

pub fn eat_at_restaurant3() {
    let _order1 = back_of_house3::Appetizer::Soup;
    let _order2 = back_of_house3::Appetizer::Salad;
}

//Enums aren’t very useful unless their variants are public; 
//it would be annoying to have to annotate all enum variants 
//with pub in every case, so the default for enum variants is 
//to be public. Structs are often useful without their fields 
//being public, so struct fields follow the general rule of 
//everything being private by default unless annotated with pub.