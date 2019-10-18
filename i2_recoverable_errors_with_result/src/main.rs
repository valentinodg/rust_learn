fn main() {
    //recoverable errors with result

    //most errors aren't serious enough to require the program to stop entirely
    //sometimes when a function fails it's for a reason that you can easily interpret and respond
    //to

    //the Result enum is defined as having 2 variants: Ok and Err
    //
    //enum Result<T, E> {
    //  Ok(T),
    //  Err(E),
    //}
    //
    //the T and E are generic type parameter
    //the T represents the type of the value that will be returned in a success case within the Ok
    //variant
    //the E represents the type of the error that will be returned in a failure case within the Err
    //variant
    //because Result has these generic type parameters we can use the Result type and the functions
    //that the std lib has defined on it in many different situations where the successful value
    //and error value we want to return may differ

    use std::fs::File;

    let _f = File::open("hello.txt");
    //File::open returns a Result -> check the std lib API doc or ask the compiler
    //if we gibe f a type annotation that we know is not the return type of the function and then
    //try to compile the code, the compiler will tell us that the types don't match
    //the error message will then tell us what the type of f is.

    //the compiler tells us the return type of the File::opne function is Result<T, E>
    //the generic parameter T has been filled in here with the type of the success value,
    //std::fs::File, which is a file handle
    //the type of E used in the error value is std::io::Error

    //in the case where File::open succeeds, the value in the variable f will be an instance of Ok
    //that contains a file handle
    //in the case where it fails, the value in f will be an instance of Err that contains more
    //information about the kind of error that happened

    // let _f = match _f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("problems opening the file: {:?}", error)
    //     }
    // };

    //like the Option enum, the Result enum and its variants have been brought into scope by the
    //prelude, so we don't need to specify Result:: before the Ok and Err variants in the match arms

    //here we tell rust that when the result is Ok, return the inner file value out of the Ok
    //variant, and we then assign that file handle value to the variable f (after the match we can
    //use the file handle for reading or writing
    //the other arm of the match handles the case where we get an Err value from File::option and
    //here we've chosen to call the panic! macro

    //matching on different errors

    //the previous core will panic! no matter why File::open failed
    //we want to take different actions for different failure reasons: if File::open failed because
    //the file doesn't exist we want to create the file and return the hangle to the new file; if
    //File::open failed for any other reason (for example: permission lack) we still want the code
    //to panic!

    use std::io::ErrorKind;

    let _f = match _f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating the file: {:?}", e),
            },
            other_error => panic!("problem opening the file: {:?}", other_error),
        },
    };
    //the type of the value that File::open returns inside the Err variant is io::Error, which is a
    //struct provided by the std lib
    //this struct has a method kind that we can call to get an io::ErrorKind value
    //the enum io::ErrorKind is provided by the std lib and has variants representing the different
    //kinds of errors that might result from an io operation

    //the Result<T, E> type has many methods that accept a closure and are implemented using match
    //expressions, let's rewrite our code and make it more concise

    let _f2 = File::open("hello2.txt");

    let _f2 = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello2.txt").unwrap_or_else(|error| {
                panic!("problem creating the file: {:?}", error);
            })
        } else {
            panic!("problem opening the file; {:?}", error);
        }
    });
    //this code doesn't contain any match expressions and is cleaner to read (look up the
    //unwrap_or_else method in the std lib doc)
    //many more of these methods can clean up huge nested match expressions when you're dealing
    //with errors

    //shortcut for panic on error: unwrap and expect

    //the Result<T, E> type has many helper methods defined on it to do various tasks
    //one of these methods, called unwrap, is a shortcut method that is implemented just like the
    //match expression we wrote above
    //if the Result value is the Ok variant, unwrap will return the value inside the Ok
    //if the Result value is the Err variant, unwrap will call the panic! macro for us

    //
    // let _f3 = File::open("hello3.txt").unwrap();
    //
    //this code will panic if the hello3.txt file doesn't exist

    //
    // let _f4 = File::open("hello3.txt").expect("failed to open hello3.txt");
    //
    //the expect method is similar to unwrap but lets us also choose the panic! error message

    //propagating errors

    //when you're writing a function whose umplementation calss something that might fail, instead
    //of handling the error within this function, you can return the error to the calling code so
    //that it can decide what to do
    //thisi is known as propagating the error and gives more control to the calling code, where
    //there might be more information or logic that dictates how the error should be handled than
    //what you have available in the context of your code

    use std::io;
    use std::io::Read;

    fn _read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello4.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
    //this function can be written in a much shorter way (see after)
    //the return type of the function is Result<T, E> where the generic parameter T has been filled
    //in with the concrete type String and where the generic type E has been filled with the
    //concrete type io::Error
    //we chose io::Error as the return type of this function bevause that happens to be the type of
    //the error value return from both of the operations we're calling in this function's body that
    //might fail (the File::open function and the read_to_string method)

    //with these function we are trying to read a username from a file
    //we start by calling the File::open function that return a Result<T, E> value
    //then we handle that Result value using a match expression
    //in the Err case we return early from this function and pass the error value from File::open
    //back to the calling code as this function's error value
    //in the Ok case we store the file handle in the variable f and continue

    //then we create a new String in variable s and call the read_to_string methos on the file
    //handle in f to read the contents of the file into s
    //the read_to_string method also returns a Result because it might fails and so we need to use another match expression
    //in the Err case we return the error value using propagation (like above)
    //in the Ok value we store and return (without return because the match expression is the last
    //expression of the function) the username

    //this pattern of propagating errors is so common in rust that rust provides the ? operator to
    //make this easier

    //a shortcut for propagating errors: the ? operator

    //let's implement again the read_username_from_file function in a more concise way using the ?
    //operator for error propagation

    fn _read_username_from_file2() -> Result<String, io::Error> {
        let mut f = File::open("hello5.txt")?;
        let mut s = String::new();

        f.read_to_string(&mut s)?;
        Ok(s)
    }
    //the ? placed after a Result value is defined to work in almost the same way as the match
    //expression that we used above
    //if the value of the Result is an Ok, the value inside the Ok will get returned from this
    //expression and the program will continue
    //if the value is an Err, the Err will be returned from the whole function as if we had used
    //the return keyword so the error value gets propagated to the calling code

    //there is a difference between what match expression and ? operator do:
    //error values that have the ? operator called on them fo through the from function, defined in
    //the From trait in the std lib, which is used to convert errors from one type into another
    //when the ? operator calls the from function, the error type received is converted into the
    //error type defined in the return type of the current function
    //as long as each error type implements the from function to define how to convert itseld to
    //the returned error type, the ? operator takes care of the conversion automatically

    //the ? operator eliminates a lot of boilerplate and makes this function's implementation
    //easier
    //we could even shorten this code further by chaining method calls immediately after
    //the ?
    fn _read_username_from_file3() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello6.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }
    //here, instead of creating a variable f, we've chained the call to read_to_string directly
    //onto the result of File::open (we still have the ? operators)

    //there's a way to make this function even shorter

    use std::fs;

    fn _read_username_from_file4() -> Result<String, io::Error> {
        fs::read_to_string("hello7.txt")
    }
    //reading a file into a string is a fairly common operation so rust provides the convenient
    //fs::read_to_string function that opens the file, creates a new String, reads the contets of
    //the file, puts the contents into  that String, and returns it

    //the ? operator can only be used in functions that return Result

    //the ? operator can only be used in functions that have a return type of Result, because it is
    //defined to work in the same way as the match expression we defined above
    //let's look at what happens if we use the ? operator in the main funcion, whichc you'll recall
    //has a return type of ()

    //
    // let fo = File::open("hello.txt")?;
    //
    //the compiler will respond to us with the following error message:
    //"we cannot use the ? operator in a function that returns ()"

    //this error points out that we're only allowed to use the ? operator in a function that
    //returns Result<T, E>
    //when you're writing code in a function that doesn't return Result<T, E> and you want to use ?
    //when you call other functions that return Result<T, E>, you have 2 choises to fix this
    //problem:
    //one technique is to change the return type of your function to be Result<T, E> if you have no
    //restrinctions preventing that
    //the other technique is to use a match or one of the Result<T, E> methods to handle the
    //Result<T, E> in whatever way is appropriate

    //the main function is special and there are restrictions on what its return type must be
    //1 valid return type for main is () and conveniently another valid return type is Result<T, E>
    //
    // fn main() -> Result<(), Box<dyn Error>> {
    //     let f = File::open("hello.txt");
    //
    //     Ok(())
    // }
    //
    //the Box<dyn Error> type is called a trait object
    //for now you can read Box<dyn Error> to mean "any kind of error"
    //using ? in a mani function with this return type is allowed
}
