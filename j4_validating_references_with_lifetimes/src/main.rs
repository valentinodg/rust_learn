fn main() {
    // VALIDATING REFERENCES WITH LIFETIMES
    
    // one detail we didn't discuss when we talked about references and lifetimes is
    // that every reference in rust has a lifetime, which is the scope for which that 
    // reference is valid
    // most of the times, lifetimes are implicit and inferred, just like most of the
    // times, types are inferred
    // we must annotate types when multiple types are possible; in a similar way, we
    // must annotate lifetimes when the lifetimes of references could be related in a
    // few different ways
    // rust requires us to annotate the relationships using generic lifetime parameters
    // to ensure the actual references used at runtime will definitely be valid
    
    // lifetimes is rust's most distinctive feature

    // PREVENTING DANGLING REFERENCES WITH LIFETIMES

    // the main aim of lifetimes is to prevent dangling references, which cause a program
    // to reference data other than the data it's intended to reference
    // (for example)
    //
    // {
    //     let r;
    //     {
    //         let x = 5;
    //         r = &x;
    //     }
    //     println!("r: {}", r);
    // }
    //
    // inside the inner scope, we attempt to set the value of r as a reference to x
    // this code won't compile because the value r is referring to has gone out of 
    // scope before we try to use it
    // (furthermore if we try to use a variable before giving it a value, we'll get a 
    // compile-time error, because rust does not allow null values)
    // the variable x doesn't "live long enough"
    // (if rust allowed this code to work, r would be referencing memory that was 
    // deallocated when x went out of scope and so anything we tried to do with r 
    // wouldn't work correctly)
    
    // THE BORROW CHECKER
    
    // the rust compiler has a borrow checker that compares scopes to determine whether
    // all borrows are valid
    // let's annotate the lifetime of r with 'a (block) and the lifetime of x with 'b 
    // (block); the inner 'b block is much smaller than the outer 'a lifetime clock
    //
    // {
    //     let r;                   ----------+-- 'a
    //     {                                  |
    //         let x = 5;           --+-- 'b  |
    //         r = &x;                |       |
    //     }                        --+       |
    //     println!("r: {}", r);              |
    // }                            ----------+
    //
    // at compile time, rust compares the size of the 2 lifetimes and sees that r has a
    // lifetime of 'a but than it refers to memoty with a lifetime of 'b; the program
    // is rejected because 'b is shorter than 'a (the subject of the reference doesn't
    // live as long as the reference)
    // FIX:
    //
    // {
    //     let x = 5;               ----------+-- 'b
    //                                        |
    //     let r = &x;              --+--     |
    //                                |       |
    //     println!("r: {}", r);      |       |
    //                              --+       |
    // }                            ----------+
    //
    // here, x has the lifetime 'b, which in this case is larger that 'a, this means r
    // can reference x because rust knows that the reference in r will always be valid
    // while x is valid

    // GENERIC LIFETIMES IN FUNCTIONS

    // let's write a function that returns the longer of 2 string slices
    
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("the longest string is {}", result)
    
    // note that we want the function to take string slices, which are references, 
    // because we don't want the longest function to take ownership of its parameters
    // we want to allow the function to accept slices of a String (string1) as well as 
    // string literals (string2)

    // LIFETIME ANNOTATION SYNTAX
    
    // lifetyme annotations don't change how long any of the references live
    // just as functions can accept any type when the signature specifies a generic
    // type parameter, functions can accept references with any lifetime by specifying
    // a generic lifetime parameter
    // lifetime annotations describe the relationship of the lifetimes of multiple
    // references to each other without affecting the lifetimes
    
    // lifetime annotations have a slightly unusual syntax: 
    // the names of lifetime parameters must start with an ' and are usually all 
    // lowercase and very short, like generic types
    // (for example)
    //
    // &i32                 //a reference (without a lifetime parameter)
    // &'a i32              //a reference with an explicit lifetime
    // &'a mut i32          //a mutable reference with an explicit lifetime
    //
    // 1 lifetime annotation by itself doesn't have much meaning, because the 
    // annotations are meant to tell rust how generic lifetime parameters of multiple
    // references relate to each other

    // AFTER FIX:
    
    // the function signature now tells rust that for some lifetime 'a, the function takes
    // 2 parameters, both of which are string slices that live at least as long as lifetime
    // 'a
    // the function signature also tells rust that the string slice returned from the function
    // will live at least as long as lifetime 'a
    // this means taht the lifetime of the reference returned by the longest function
    // is the same as the smaller of the lifetimes of the references passed in
    // (REMARK: when we specify the lifetime parameters in this function signature,
    // we're not changing the lifetimes of any values passed in or returned, we're specifying
    // that the borrow checker should reject any values that don't adhere to these 
    // constraints)
    // we must annotate lifetimes in function signature and not in function body (rust can
    // analyze the code within the function without any help)
    // when a function has references to or from code outside that function it becomes almost 
    // impossible for rust to figure out the lifetimes of the paramenters or return values on 
    // its own and so we must annotate the lifetimes manually
    
    // 
    
}

// this code won't compile; the return type needs a generic lifetime parameter on it
// because rust can't tell whether the reference being returned refers to x or y
// when we're defining this function, we don't know the concrete values that will be
// passed into this function, so we don't know whether the if case or the else case
// will execute
// we also don't know the concrete lifetimes of the references that will be passed in,
// so we can't look at the scopes as we did before to determine whether the reference 
// we return will always be valid
// the borrow checker can't determine this either, because it doesn't know how the 
// lifetimes of x and y relate to the lifetime of the return value

// FIX:
// to fix this error, we'll add a generic lifetime parameters y that define the 
// relationship between the references so the borrow checker can perform its analysis
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        &str1
    } else {
        &str2
    }
}

// LIFETIME ANNOTATIONS IN FUNCTION SIGNATURES
    
// we need to declare generic lifetime paramenters inside < > between the function name 
// and the parameter list
// the constraint we want to express in this signature is that all the references in the
// parameters and the return value must have the same lifetime
// we'll name the lifetime 'a and then add it to each reference
