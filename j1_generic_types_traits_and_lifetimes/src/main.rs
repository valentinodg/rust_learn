fn main() {
    //every programming language has tools for effectively handling
    //the duplication of concepts, in rust one such tool is generics
    //generics are abstract stand-ins for concrete types or other
    //properties, we can express the behaviour of generics or how they
    //relate to other generics without knowing what will be in their 
    //place when compiling and running the code
    //functions can take parameters of some generic type instead of a
    //concrete type

    //next we will make a generic function from 2 functions that 
    //differs only in the types of their parameter
    //then we'll learn how to use traits to define behavior in a 
    //generic way
    //we can combine traits and generic types to constrain a generic
    //type to only those types that have a particular behavior, as
    //opposed to just any type
    //finally we'll discuss lifetimes, variety of generics that give
    //the compiler information about how to reference relate to 
    //each other
    //lifetimes allow us to borrow values in many situations while
    //still enabling the compiler to check that the references are 
    //valid

    //removing duplication by extracting a function

    //let's first try to recognize duplication on code and then let's
    //apply this technique to extract a generic fuction from 
    //duplicated code

    let number_list = vec![34,50,25,100,65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("\nthe largest number is {}", largest);
    //if we want to find the largest number  in 2 different lists
    //of number, we can duplicate the code using the same logic at
    //2 different places in the program
    //although this solution works, duplicating code is tedious and
    //error prone
    //the better choice for us is to create an abstraction by 
    //defining a function that operates on any list of integers 
    //given to it in a parameter

    let number_list2 = vec![102,35,7,34,5,4,4,67543];
    let largest2 = largest_fn(&number_list2);
    println!("\nthe largest number is {}", largest2);

    let number_list3 = vec![34,6,3,2,45,787654,234];
    let largest3 = largest_fn(&number_list3);
    println!("the largest number is {}", largest3);

    //we will use the same technique with generics to reduce code
    //duplication
    //in the same way that the function body can operate on an 
    //anstract list instead of specific values, generics allow code
    //to operate on abstract types
}

//let's define the largest_fn function
//the largest_fn function has a parameter called list, which 
//represents any concrete slice of i32 values that we might pass
//into the function
fn largest_fn(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
