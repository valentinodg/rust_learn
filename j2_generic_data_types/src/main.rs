fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    //we can use generics to create definitions for items like
    //function signatures or structs, which we can then use with
    //many different concrete data types
    //let’s first look at how to define functions, structs, enums,
    //and methods using generics

    //in function definitions

    //when defining a function that uses generics, we place the
    //generics in the signature of the function where we would
    //usually specify the data types of the parameters and
    //return value

    //first let's show an example of code duplication
    let number_list = vec![345, 7, 7, 23, 3, 467, 4, 3, 68];
    let result = largest_i32(&number_list);
    println!("\nthe largest number is {}", result);

    let number_list2 = vec!['c', 'r', 'g', 's', 'e', 'm', 'q'];
    let result2 = largest_char(&number_list2);
    println!("the largest char is {}", result2);
    //the 2 functions: largest_i32 and largest, have the same bodies but accept
    //different type of parameter (i32 and char respectively)
    //let's eliminate the duplication by introducing a generic type parameter in a
    //single function

    //we can use any identifier as a type parameter name in our new function
    //but we'll use T because, by convention, parameter names in rust are short,
    //ofter just a letter and rust's type-naming convention is CamelCase

    //when we use a parameter in the body of the function, we have to declare the
    //parameter name in the signature so the compiler knows what that name means
    //similarly when we use a type parameter name in a function signature, we have to
    //declare the type parameter name before we use it

    //let's write the new function (go below)
    //let's call the new function using an i32 slice and a char slice references

    let number_list3 = vec![34, 50, 24, 100, 65];
    let resul3 = largest_i32(&number_list3);
    println!("\nthe largest number is {}", resul3);

    let number_list4 = vec!['y', 'm', 'a', 'q'];
    let result4 = largest_char(&number_list4);
    println!("the largest char is {}", result4);

    //in struct definitions

    //we can define structs to use a generic type parameter in 1 or more fields using
    //the < > syntax (go below)

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    //we've used only 1 generic type to define Point<T>, this definition says that the
    //Point<T> struct is generic over some type T, and the fields x and y are both the
    //SAME type, whatever that type may be (type mismatch error)
    //
    //let wont_work = Point { x: 5, y: 4.0 }; ---> this code won't compile
    //
    println!("\nint point: {:?}\nflt point: {:?}", integer, float);

    let new_integer = NewPoint { x: 5, y: 10 };
    let new_float = NewPoint { x: 1.0, y: 4.0 };
    let new_mix = NewPoint { x: 5, y: 4.0 };
    println!(
        "\nnew int point: {:?}\nnew flt point {:?}\nnew mix point: {:?}",
        new_integer, new_float, new_mix
    );

    //in enum definitions

    //as we did with structs, we can define enums to hold generic data types in their
    //variants, let's recall the Option<T> enum that the std lib provides:
    //
    // Option<T> {
    //     Some(T),
    //     None,
    // }
    //
    //Option<T> is an enum that is generic over type T and has 2 variants: Some, which
    //holds 1 value of type T, and None that doesn't hold any value
    //by using the Option<T> enum, we can express the abstract concept of having an
    //optional value and because Option<T> is generic, we can use this abstraction no
    //matter what the type of the optional value is

    //enums can use multiple generic types as well, let's recall the definition of the
    //Result enum
    //
    // Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    //
    //the Result enum is generic over 2 types T and E, and has 2 variants: Ok, which
    //holds value of type T and Err which holds a value of type E
    //this definition makes it convenient to use the Result enum anywhere we have an
    //operation that might succeed (return a value of some type T) or fail (return an
    //error of some type E)

    //in method definition

    //we can implement methods on structs and enums and use generic types in their
    //definitions
    //let's implement a method named x on the Point<T> struct that will return a
    //reference to the data in the field x of type T (go below)

    let p = Point { x: 5, y: 10 };
    println!("\npoint p -> x: {}", p.x());

    let fp = Point { x: 3.0, y: 5.3 };
    println!("\ndistance fp-origin is: {}", fp.distance_from_origin());

    //let's define a new generic methods that mixes the variable of 2 NewPoint instances
    let p1 = NewPoint { x: 5, y: 10.4 };
    let p2 = NewPoint { x: "hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("\np3: {:?}", p3);

    //performance of code using generics (monomorphization)

    //rust implements generics in such a way that your code doesn’t run any slower using
    //generic types than it would with concrete types
    //rust accomplishes this by performing monomorphization of the code that is using
    //generics at compile time
    //monomorphization is the process of turning generic code into specific code by
    //filling in the concrete types that are used when compiled
    //the compiler looks at all the places where generic code is called and generates
    //code for the concrete types the generic code is called with
    //
    // let integer = Some(5);
    // let float = Some(5.0);
    //
    //the compiler reads the values that have been used in Option<T> instances and
    //identifies two kinds of Option<T>: one is i32 and the other is f64
    //it expands the generic definition of Option<T> into Option_i32 and Option_f64,
    //thereby replacing the generic definition with the specific ones
    //the monomorphized version of the code looks like the following
    //the generic Option<T> is replaced with the specific definitions created by
    //the compiler
    //
    // enum Option_i32 {
    //     Some(i32),
    //     None,
    // }
    //
    // enum Option_f64 {
    //     Some(f64),
    //     None,
    // }
    //
    // fn main() {
    //     let integer = Option_i32::Some(5);
    //     let float = Option_f64::Some(5.0);
    // }
    //
    //because rust compiles generic code into code that specifies the type in each
    //instance, we pay no runtime cost for using generics
    //when the code runs, it performs just as it would if we had duplicated each
    //definition by hand
    //the process of monomorphization makes rust’s generics extremely efficient at
    //runtime
}

//the generic type parameter must to be placed inside < >, between the name of the
//function and the parameter list
//the function largest is generic of some type T: this function has one parameter
//named list, which is a slice of values of type T and returns a value of the same
//type T

fn lagest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//THIS CODE WON'T COMPILE (compiler -> std::cmp::PartialOrd, trait)
//the error states that the body of largest won't work for all possible types that T
//could be, because we want to compare values of type T in the body, we can only use
//types those values can be ordered
//to enable comparisons, the std lib has the std::cmp::PartialOrd trait that you can
//implement on types

//the syntax for using generics in struct definitions is similar to that used in
//function definitions: first we declare the name of the type parameter inside < >
//just after the name of the struct and then we can use the generic type in the struct
//definition
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

//struct with multiple generic type parameter
#[derive(Debug)]
struct NewPoint<T, U> {
    x: T,
    y: U,
}

//let's implement our generic method on the Point<T> struct
//note that we have to declare T just after impl so we can use it to specify that we
//are implementing methods on the type Point<T>
//by declaring T as a generic type after impl, rust can identify that the type in the
//< > in Point is a generic type rather than a concrete type
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
//let's implement another method only valid for Point<f32> instances, so we don't need
//to declare any type after impl (because f32 is a concrete type)
//this method is available for Point<f32> instances and for Point<T> instances where T
//is of type f32
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
//let's define a method that mixes the values of 2 NewPoint instances
//the purpose of this example is to demonstrate a situation in which some generic
//parameter are declared with impl and some are declared with the method definition
//here the generic parameters T and I are declared after impl, because they go with
//the struct definition
//the generic parameters V and W are declared after fn mixup, because they're only
//relevant to the method
impl<T, U> NewPoint<T, U> {
    fn mixup<V, W>(self, other_point: NewPoint<V, W>) -> NewPoint<T, W> {
        NewPoint {
            x: self.x,
            y: other_point.y,
        }
    }
}
