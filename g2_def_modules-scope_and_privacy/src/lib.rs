// DEFINING MODULES TO CONTROL SCOPE AND PRIVACY

// here we talk about modules and other parts of the module
// system, namely paths that allow you to name items
// the use keyword that brings a path into scope
// the pub keyword to make items public
// the as keyword
// external packages
// the glob operator

// modules let us organize code within a crate into groups for
// readability and easy reuse
// modules also control the privacy of items, which is wheather
// an item can be used by outside code (public) or is an
// internal implementation detail and not available for outside
// use (private)

// let's create a new library crate using the following command
// cargo new --lib "crate-name"
// and let's define dome modules and function signatures

mod _front_of_house {
    mod _hosting {
        fn _add_to_waitlist() {}
        fn _seat_at_table() {}
    }
    mod serving {
        fn _take_order() {}
        fn _serve_order() {}
        fn _take_payment() {}
    }
}

// we define a module by starting with the mod keyword, then
// specify the name of the module and place curly brackets around
// the body of the module
// inside modules we can have other modules, as in this case
// modules can also hold definitions of other items such as structs,
// enums, constants, traits or (as in this case) functions
// by using modules, we can group related definitions together
// and name why they are related

// src/main.rs and src/lib.rs are called crate roots
// the reason for their name is that the contents of either of
// these 2 files form a module named crate at the root of the
// crate's module structure, known as the module tree

//  crate
//   └── front_of_house
//       ├── hosting
//       │   ├── add_to_waitlist
//       │   └── seat_at_table
//       └── serving
//           ├── take_order
//           ├── serve_order
//           └── take_payment

// some modules are siblings to each other
// a parent module is a module that contains another module
// a child module is a module that is contained by another module
// the entire module tree is rooted under the implicit module
// named crate
