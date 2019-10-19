pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("guess value musst be between 1 and 100, got {}", value);
        }

        Guess {
            value
        }
    }

    //getter method
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    //to panic or not to panic
    
    //how do you decide when you should call panic and when you should return Result?
    //when code panics there's no way to recover
    //you could call panic! for any error situation, whether there's a possible way to recover or
    //not, but then you're making the decision on behalf of the code calling your code that a
    //situation is unirecoverable
    //when you choose to return a Result value, you give the calling code options rather than making
    //the decision for it
    //the calling code could choose to attempt to recover in a way that's appropriate for its
    //situation, or it could decide that an Err value in this case is unrecoverable, so it can call
    //panic!
    //therefore returning Result is a good default choise when you're defining a function that
    //might fail
    
    //in rare situations, it's more appropriate to write code that panics instead of returning a
    //Result (examples, prototype code and tests)
    
    //examples, prototype code and tests
    
    //when you're writing an example to illustrate some concept, having robust error-hangling code
    //in the example as well can make the example less clear
    //in examples it's understood that a call to a method like unwrap that could panic is meant as
    //a placeholder for the way you'd want your application to handle errors
    
    //similarly, the unwrap and expect methods are very handy when prototyping, before you're ready
    //to decide how to handle errors
    
    //if a method call fails in a test, you'd want the whole test to fail, even if that method
    //isn't the functionality under test
    //because panic! is how a test is marked as a failure, calling unwrap or expect is exactly
    //what should happen
    
    //case in which you have more information that the compiler
    
    //it would also be appropriate to call unwrap when you have some other logic that unsures the
    //Result will have an Ok value, but the logic isn't something the compiler understands
    //if you can ensure by manually inspecting the code that you'll never have en Err variant, it's
    //perfectly acceprable to call unwrap
    
    use std::net::IpAddr;

    let _home: IpAddr = "127.0.0.1".parse().unwrap();
    //here we're creating an IpAddr instance by parsing a hardcoded string
    //however, having a hardcoded, valid string doesn't change the return type of the parse()
    //method: we still get a Result value, and the compiler will still make us hangle the Result as
    //if the Err variant is a possibility because the compiler isn't smart enough to see that this
    //string is always a valid IP address
    //if the IP address string came from a user rather than being hardcoded into the program and
    //therefore did have a possibility of failure, we's definitely want to handle the Result in a
    //more robust way instead
    
    //guidelines for error handling
    
    //it's advisable to have your code panic when it's possible that your code could end up in a
    //bad state
    //in this context a bad state is when some assumption, guarantee, contract, or invariant has
    //been broken, such as when invalid values, contradictory values or missing values are passed
    //to your code, plus 1 or more of the following:
    //
    // the bad state is not something that's expected to happen occasionally
    // your code after this point needs to rely on not being in this bad state
    // there's not a good way to encode this information in the types you use
    //
    
    //if someone calls your code and passes in values that don't make sense, the best choice might
    //be to call panic!
    //similarly, panic! is often appropriate if you're calling external code that is our of your
    //control and it returns an invalid state that you have no way of fixing
    
    //however when failure is expected it's more appropriate to return a Result than to make a
    //panic! call
    //in these cases returning a Result indicates that failure is an expected possibilty that the
    //calling code must decide how to handle
    
    //when your code performs operations on values, your code should verigy the calues are valid
    //first adn panic if the values aren't valid
    //this is mostly for safety reasons: attempting to operate on invalid data can expose your code
    //to vulnerabilities 
    //this is the main reason the std lib will call panic! if you attempt an out-of-bound memory
    //access
    //functions ofter have contracts: their behaviour is only guaranteed if the inputs meet
    //particular requirements
    //panicking when the contract is violated makes sense because a contract violation always
    //indicated a caller-side bug and it's not a king of error you want the calling code to have to
    //explicitly handle (there's no reasonable way for calling code to recover, the calling
    //programmers need to fix the code)
    //contracts for a function, especially when a violation will cause a panic, should be explained
    //in the API doc for the function
    
    //however, having lots of error checks in all of your functions would be verbose and annoying
    //fortunately, you can use rust's type systems (and thus the type checking the compiler does)
    //to do many of the checks for you
    
    //creating custom types for validation
    
    //(recall the guessing game) let's convert the u32 type of the guess into an i32 type
    //let's also handle the following cases:
    // 
    // the guess is a letter
    // the guess is a negative number
    // the guess is out of range (1 - 100)
    //
    
    //
    // loop {
    //     // --snip--
    //     let guess: i32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    //
    //     if guess < 1 || guess > 100 {
    //         println!("the secret number will be between 1 and 100");
    //         continue;
    //     }
    //
    //     match guess.cmp(&secret_number) {
    //         // --snip--
    //     }
    // }
    //
    
    //if we have many many function with this critical requirement (range), we can make a new type
    //and put the validations in a function to create an instance of the type rather than repeating
    //the validations everywhere (in thies way, it's safe for functions to use the new type in
    //their signature and confidently use the values they receive)
    
    //look above (before main)

   //the code in the body of the new function tests value to make sure it's between 1 and 100
   //if value doesn't pass this test we make a panic! call which will alert the programmer who is
   //writing the calling code that they have a bug they need to fix because creating a Guess with a
   //value outside this range would violate the contract the Guess::new is relying on
   
   //we implement a method named value that borrows self, doesn't have any other parameters, and
   //returns an i32
   //this kind of method is sometimes called a getter, because its purpose is to get some data from
   //its fields and return it
   //this public method is necessary because the value field of the Guess struct is private
   //it's important that the value field be private so code using the Guess struct is not allowed
   //to set value directly: code outside the module must use the Guess::new function to create an
   //instance of Guess, thereby ensuring there's no way for a Guess to have a value that hasn't
   //been checked by the conditions in the Guess::new function
}
