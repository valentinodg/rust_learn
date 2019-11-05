// this struct has one field, part, that holds a string slice, which is a reference
// as with generic data types, we declare the name of the generic lifetime parameter
// inside < > after the name of the struct so we can use the lifetime paramenter in the
// body of the struct definition
// this annotation means an instance of ImportantExcerpt can't outlive the reference it
// holds in its part field 

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

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
    println!("the longest string is {}", result);
    
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
    
    // the function signature now tells rust that for some lifetime 'a, the function 
    // takes 2 parameters, both of which are string slices that live at least as long as 
    // lifetime 'a
    // the function signature also tells rust that the string slice returned from the 
    // function will live at least as long as lifetime 'a
    // this means taht the lifetime of the reference returned by the longest function
    // is the same as the smaller of the lifetimes of the references passed in
    // (REMARK: when we specify the lifetime parameters in this function signature,
    // we're not changing the lifetimes of any values passed in or returned, we're 
    // specifying that the borrow checker should reject any values that don't adhere to  
    // these constraints)
    // we must annotate lifetimes in function signature and not in function body (rust 
    // can analyze the code within the function without any help)
    // when a function has references to or from code outside that function it becomes  
    // almost impossible for rust to figure out the lifetimes of the paramenters or   
    // return values on its own and so we must annotate the lifetimes manually
    
    // when we pass concrete references to longest, the concrete lifetime that is 
    // substituted for 'a is the part of the scope of x that overlaps with the scope of
    // y (in other words the generic lifetime 'a will get the concrete lifetime that 
    // is equal to the smaller of the lifetimes of x and y)
    // because we've annotated the returned reference with the same lifetime parameter
    // 'a, the returned reference will also be valid for the lenght of the smaller of
    // the lifetimes of x and y

    let string3 = String::from("stringA");
    {
        let string4 = String::from("stringBlongest");
        let result2 = longest(string3.as_str(), string4.as_str());
        println!("the longest string is {}", result2);
    }

    // the borrow checker approves this code and the program compile
    // (in this example)
    // string3 is valid until the end of the outer scope
    // string4 is valid until the end of the inner scope
    // result2 references simething that is valid until the end of the inner scope
    //    
    // let string3 = String::from("stringA");
    // let result2;
    // {
    //     let string4 = String::from("stringBlongest");
    //     result2 = longest(string3.as_str(), string4.as_str());
    // }
    // println!("the longest string is {}", result2);
    //
    // in this way the code won't compile
    // the error shows that for result2 to be valid for the println! statement, string4
    // and therefore result2 will contain a reference to string3, because string3 has
    // not gone out of scope yet, a reference to string3 will still be valid for the
    // println! statement
    // however, the compiler can't see that the reference is valid in this case
    // we've told rust that the lifetime of the reference returned by the longest 
    // function is the same as the smaller of the lifetimes of the references passed in
    // the borrow checker disallows this code as possibly having an invalid reference

    // THINKING IN TERMS OF LIFETIMES

    // let's modify the longest function to always return the first parameter that it 
    // receives (and not the longest string)
    // (first function, go below)

    // when returning a reference from a function, the lifetime parameter for the return
    // type needs to match the lifetime parameter for one of the parameters
    // if the reference returned does not refer to one of the parameters, it must refer 
    // to a value created within this function, which would be a dangling reference 
    // because the value will go out of scope at the end of the function
    // (for example)
    //
    // fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    //     let result = String::from("really long string");
    //     result.as_str()
    // }
    //
    // even though we've specified a lifetime parameter 'a for the return type, this
    // implementation will fail to compile because the return value lifetime is not 
    // related to the lifetime of the parameters at all
    // the problem is that result goes out of scope and gets cleaned up at the end of
    // the longest function
    // we're also trying to return a reference to result from the function 
    // there is no way we can specify lifetime parameters that would change the dangling
    // and rust won't let us create a dangling reference
    // (in this case the best fix would be to return an owned data type rather than a 
    // reference so the calling function is then responsible for cleaning up the value)
    // lifetime syntax is about connecting the lifetimes of various parameters and 
    // return values of functions
    // once they're connected, rust has enough information to allow memory-safe operations
    // and disallow operations that would create dangling pointers or otherwise violate 
    // memory safety
    
    // LIFETIME ANNOTATIONS IN STRUCT DEFINITIONS

    // so far, we've only defined structs to hold owned types
    // is possible for structs to hold references, but in that case we would need to add
    // a lifetime annotation on every reference in the struct's definition
    // (struct ImportantExcerpy, go above)

    let novel = String::from("novel novel novel. novelnovel novel novelnovel");
    let first_sentence = novel.split('.')
        .next()
        .expect("'.' not found");
    let i = ImportantExcerpt { part: first_sentence };
    println!("i: {:?}", i);

    // here we creates an instance of ImportantExcerpt struct that holds a reference to
    // the first sentence fo the String owned by the variable novel
    // the data in novel exists before the ImportantExcerpt instance is created
    // in addition, novel doesn't go out of scope until after the ImportantExcerpt goes
    // out of scope, so the reference in the ImportantExcerpt instance is valid

    // LIFETIME ELISION

    // we have learned that every reference has a lifetime and that you need to specify
    // lifetime parameters for functions or structs that use references
    // (look below -> first_word() function)

    // the reason the first_word function compiles without lifetime annotations is 
    // historical: in early versions of rust (pre-1.0) this code wouldn't have compiled
    // because every reference needed an explicit lifetime
    // at that time, the function signature would have been written like this
    // 
    // fn first_word<'a>(s: &'a str) -> &'a str {}
    // 
    // after writing a lot of rust code, the rust team found that rust programmers were
    // entering the same lifetime annotations over and over in particular situations
    // these situations were predictable and followed a few deterministic patterns so 
    // the developers programmed these patterns into the compiler's code so the borrow
    // checker could infer the lifetimes in these situations and wouldn't need explicit
    // annotations
    
    // the patterns programmed into rust's analysis of references are called lifetime
    // elision rules
    // they're a set of particular cases that the compiler will consider, and if your
    // code fits these cases, you don't need to write the lifetimes explicitly
    // the elision rules don't provide full inference
    // if rust deterministically applies the rules but there is still ambiguity as to 
    // what lifetimes the references have, the compiler won't guess what the lifetime
    // of the remaining should be (instead of guessing, the compiler will give you an
    // error that you can resolve by adding the lifetime annotations)
    
    // lifetimes on function or method parameters are called input lifetimes
    // lifetimes on return values are called output lifetimes

    // the compiler uses 3 rules to figure out what lifetimes references have when there
    // aren't explicit annotations
    // the 1 rule applies to input lifetimes
    // the 2 and 3 rules aplly to output lifetimes
    // (these rules apply to fn definitions as well as impl blocks)

    // the 1 rule is that each parameter that is a reference gets its own lifetime 
    // parameter
    // (for example)
    // 
    // a function with 1 parameter gets 1 lifetime parameter
    // fn foo<'a>(x: &'a i32) {}
    // 
    // a function with 2 parameter gets 2 lifetime parameter
    // fn foo<'a, 'b>(x: &'a i32, y: &'b i32) {}
    // 
    // ...

    // the 2 rule is if there is exactly 1 input lifetime parameter, that lifetime is
    // assigned to all output lifetime parameters
    // (for example)
    // 
    // fn foo<'a>(x: &'a i32) -> &'a i32 {}

    // the 3 rule is if there are multiple input lifetime parameters, but 1 of them is 
    // &self or &mut self because this is a method, the lifetime of seld is assigned to
    // all output lifetime parameters (this makes methods much nicer to read and write
    // because fewer symbols are necessary)

    // (application of the rules to the first_word function)
    // (no lifetime parameters)
    // fn first_word(s: &str) -> &str {}
    // 
    // (the compiler applies the 1 rule, which specifies that each parameter gets its
    // own lifetime, let's call it 'a as usual)
    // fn first_word<'a>(s: &'a str) -> &str {}
    // 
    // (the compiler applies the 2 rule because there is exactly 1 input lifetime; the
    // 2 rule specifies that the lifetime of the 1 input parameter gets assigned to the
    // output lifetime)
    // fn first_word<'a>(s: &'a str) -> &'a str {}
    // 
    // (now the analysis is complete and all the references in this function signature
    // have lifetimes)

    // (application of the rules to the longest function)
    // (no lifetime parameters)
    // fn longest(x: &str, y: &str) -> &str {}
    // 
    // (the compiler applies the 1 rule, now we have 2 parameters and so 2 lifetimes)
    // fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
    // 
    // (the 2 rule doesn't apply because there is more than 1 input lifetime)
    // (the 3 rule doesn't apply because longest is a function rather than a method, so
    // none of the parameters are self)
    // (the compiler worked through the lifetime elision rules but still couldn't figure
    // out alle the lifetimes of the references in the signature (in this case the 
    // return type's lifetime))

    // LIFETIME ANNOTATIONS IN METHOD DEFINITIONS

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

// MOD LONGEST FUNCTION

// we've specified a lifetime parameter 'a for the parameter x and the return type, but
// not for the parameter y, because the lifetime of y does not have any relationship with
// the lifetime of x or the return value

fn _longest2<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

// FIRST_WORD FUNCTION

// the first_word function will compile without lifetime annotations, even though the
// parameter and return type are references

fn _first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}