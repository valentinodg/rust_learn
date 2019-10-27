//traits: defining shared behavior

//a trait tells the rust compiler about functionality a particular type has and can
//share with other types
//we can use traits to define shared behavior in an abstract way
//we can use trait bounds to specify that a generic can be any type that has certain
//behavior
//(traits are similar to a feature often called interface in other languages, although
//with some differences)

//defining a trait

//a type's behavior consists of the methods we can call on that type
//different types share the same behavior if we can call the same methods on all of
//those types
//trait definitions are a way to group method signatures together to define a set of
//behaviors necessary to accomplish some purpose

//let's make a media aggregator library that can display summaries of data that might
//be stored in a NewsArticle or Tweet instance
//NewsArticle struct holds a news story filed in a particular location
//Tweet can have at most 280 chars along with metadata that indicates whether it was a
//new tweet, a retweet or a reply to another tweet

//we need a summary from each type adn we need to request that summary by calling a
//summarize method on an instance

pub trait Summary {
    fn summarize(&self) -> String;
}

//a summary trait consists of the behavior provided by a summarize method

//here, we declare a trait using the trait keyword and then the trait's name, which is
//Summary in this case
//inside the {} we declare the method signatures that describe the behaviors of the
//types that implement this trait, which in this case is fn summarize(&self) -> String

//after the method signature, instead of providing an implementation within {}, we use a ;
//each type implementing this trait must provide its own custom behavior for the body of
//the method
//the compiler will enforce that any type that has the Summary trat will have the method
//summarize defined with this signature exactly

//a trait can have multiple methods in its body (the method signatures are listed 1 per
//line and each line ends in a ;)

//implementing a trait on a type

//let's implement the Summary trait on the types in our media aggregator
//first let's implement it on the NewsArticle struct using the headline, the author, and
//the location to create the return value of summarize

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

//for the Tweet struct, we define summarize as the username followed by the entire text
//of the tweet (assuming that the tweet content is already limited to 280 chars)

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

//implementing a trait on a type is similar to implementing regular methods
//the difference is that after impl, we put the trait name that we want to implement,
//then use the for keyword and then specify the name of the type we want to implement
//the trait for
//within the impl block we put the method signatures that the trait definition has defined
//instead of adding a ; after each signature, we use {} and fill in the method body with
//the specific behavior that we want the methods of the trait to have for the particular
//type

//(look main.rs)

//we have defined NewsArticle, Tweet and the Summary trait all in the same scope (lib.rs)
//let's say this lib.rs is for a crate we've called aggregator and someone else wants to
//use our crate's functionality to implement the Summary trait on a struct defined within
//their library's scope
//they would need to bring the trait into their scope first specifying
//
// use aggregator::Summary;
//
//which then would enable them to implement Summary for their type
//the Summary trait would also need to be a public trait for another crate to implement it
//(use pub trait for implement the trait as public)

//one restriction to note with trait implementations is that we can implement a trait on
//a type only if either the trait or the type is local to our crate (we can't implement
//external traits on external types)
//this restriction is part of a property of programs called coherence, and more
//specifically the orphan rule, so named because the parent type is not present
//(this code ensures that other people's code can't break your code and vice versa,
//without the rule 2 crates could implement the same trait for the same type, and rust
//wouldn't know which implementation to use)

//default implementations

//sometimes it's useful to have default behavior for some or all of the emthods in a trait
//instead of requiring implementations for all methods on every type

pub trait Summary2 {
    fn summarize2(&self) -> String {
        String::from("(read more...)")
    }
}

//(to use a default implementation to summarize instances of NewsArticle istead of
//defining a custom implementation, we specify an empty impl block
//impl Summary for NewsArticle {})
impl Summary2 for NewsArticle {}

//default implementations can call other methods in the same trait, even if those other
//methods don't have a default implementation
pub trait Summary3 {
    fn summarize_author(&self) -> String;

    fn summarize3(&self) -> String {
        format!("(read more from {}...)", self.summarize_author())
    }
}

//now we only need to define summarize_author when we implement the trait on a type

impl Summary3 for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

//note that is isn't possible to call the default implementation from an overriding
//implementation of that same method

//traits as parameters

//let's explore how to use traits to define functions that accept many different types
//let's define a notify function that calls the summarize method on its item parameter,
//which is of some type that implements the Summary trait (let's use the impl Trait
//syntax)

pub fn notify(item: impl Summary) {
    println!("breaking news! {}", item.summarize());
}

//instead of a concrete type for the item parameter, we specify the impl keyword and the
//trait name
//this parameter accepts any type that implements the specified trait
//in the body of notify we can call any methods on item that come from the Summary trait,
//such as summarize

//trait bound syntax

//the impl Trait syntax works for simple cases but is actually syntax sugar for
//a longer form, which is called a trait bound (equivalent form but more verbose)

pub fn _notify2<T: Summary>(item: T) {
    println!("breaking news! {}", item.summarize());
}

//the trait bound syntax can express more complexity in cases like the following
//(for ex.: we want to force both parameters to have the same type)[2nd form is better]

// [1st form]
pub fn _notify3(_item1: impl Summary, _item2: impl Summary) {}

// VS

// [2nd form]
pub fn _notify4<T: Summary>(_item1: T, _item2: T) {}

//(for ex.: we want that the funcion notify2 to allow item1 and item2 to have
//different types)[1st form is better]

//specifying multiple trait bounds with the + syntax

//we can also specify more than 1 trait bound
//let's use the + syntax to make the notify function able to use display formatting on
//item as well as the summarize method (Summary and std Display traits)

use std::fmt::Display;

pub fn _notify5(_item: impl Summary + Display) {}

pub fn _notify6<T: Summary + Display>(_item: T) {}

//now the body of notify can call summarize and use {} to format item

//clearer trait bounds with where clauses

//using too many trait bounds has its downsides
//each generic has its own trait bounds, so functions with multiple generic type
//parameters can contain lots of trat bound information between the function's name and
//its paramenter list, making the function signature hard to read
//for this reason rust has alternate syntax for specifying trait bounds inside a where
//clause after the function signature

use std::fmt::Debug;

//(instead of writing this)
fn _some_function<T: Display + Clone, U: Clone + Debug>(_t: T, _u: U) -> i32 {
    1
}

//(we can use a where clause like this)
fn _some_function2<T, U>(_t: T, _u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}

//returning types that implement traits

//we can also use the impl Trait syntax in the return position to return a value of some
//type that implements a trait
fn _return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("user"),
        content: String::from("contentx contentx"),
        reply: false,
        retweet: false,
    }
}

//by using impl Summary for the return type, we specify that the _return_summarizable
//function returns some type that implements the Summary trait without naming the
//concrete type
//the ability to return a type that is only specified by the trait it implements is
//especially useful in the context of closures and iterators (that we'll see later)
//closures and iterators create types that only the compiler knows or types that are
//very long to specify
//the impl Trait syntax lets you concisely specify that a function returns some type
//that implements the Iterator trait without needing to write out a very long type
//NOTE: however you can only use impl Trait if you're returning a single type, and for
//this reason we need to find a workaround to do this without using it

//THIS CODE WON'T COMPILE
//
// fn _return_summarizable2(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("title"),
//             location: String::from("place"),
//             author: String::from("user"),
//             content: String::from("contentx contentx"),
//         }
//     } else {
//         Tweet {
//             username: String::from("user"),
//             content: String::from("contentx contentx"),
//             reply: false,
//             retweet:false,
//         }
//     }
// }
//
//returning either a NewsArticle or a Tweet isn't allowed due to restrictions around how
//the impl Trait syntax is implemented in the compiler

//fixing the largest function with trait bounds

//(go to "j2_generic_data_types/src/main.rs" to fix "item > largest" - ROW 205)

//using trat bounds to conditionally implement methods

//by using a trait bound with an impl block that uses generic type parameters, we can
//implement methods conditionally for types that implement the specified traits

struct _Pair<T> {
    x: T,
    y: T,
}

impl<T> _Pair<T> {
    fn _new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> _Pair<T> {
    fn _cmp_display(&self) {
        if self.x >= self.y {
            println!("the largest number is x = {}", self.x);
        } else {
            println!("the largest number is y = {}", self.y);
        }
    }
}

//we can also conditionally implement a trait for any type that implements another trait
//implementations of a trait on any type that satisfies the trait bounds are called
//blanket implementations and are extensively used in the rust std lib

//(for ex.:)the std lib implements the ToString trait on any type that implements the
//Display trait
//
// impl <T: Display> ToString for T { // --snip-- }
//
//we can call the to_string method defined by the ToString trait on any type that
//implements the Display trait
//(for ex.: let's convert an integer (type that implements the Display trait) into a
//string)
//
// let s = 3.to_string();
//
//traits and trai bounds let us write code that uses generic type parameters to reduce
//duplication but also specify to the compiler that we want the generic type to have
//a particular behavior
//the compiler can then use the trai bound information to check that all the concrete
//types used with our code provide the correct behavior
//in dinamically typed languages, we would get an error at runtime if we called a method
//on a type that the type didn't implement, but rust moves these errors to compile time
//so we're forced to fix the problems before our code is even able to run
//additionally we don't have to write code that checks for behavior at runtime because
//we've already checked at compile time (improving in performance without loosing the
//flexibility of generics)
