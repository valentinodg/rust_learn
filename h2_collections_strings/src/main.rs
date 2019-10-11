fn main() {
    //strings are implemented as a collection of bytes plus some methods
    //to provide useful functionality when those bytes are interpreted as
    //text

    //rust has one string type in the core language which is the string slice
    //str that is usually seen in its borrowed form &str
    //we talked about string slices which are references to some UTF-8 encoded
    //string data store elsewhere
    
    //the String type which is provided by rust's std lib rather than coded into
    //the core language, is a growable, mutable, owned, UTF-8 encoded string type
    //when rustaceans refer to "strings" in rust, they usually mean the String
    //and the string slice &str types

    //rust's std lib also includes a number of other string types such as 0sString,
    //0sStr, CString, CStr and also library crates can provide even more options
    //for storing string data
    //the names that ends with String refers to the owned variants and the names
    //that ends with Str refers to the borrowed variants
    
    //create a new string
    //many of the same operations available with Vec<T> are available with String 
    //as well
    //let's use the new function to create a string

    let _s = String::new();

    //this line creates a new empty string called s
    //let's use the method to_string() to initialize the string variables
    //(to_string() method is available on any type that implements the Display
    //trait)

    let data = "initial contents";

    let s = data.to_string();
    let s1 = "initial contents".to_string();
    //the code creates a string containing "initial contents"
    //we can also use the function String::from to create String from a string
    //literal
    
    let s2 = String::from("initial contents");
    //because strings are UFT-8 encoded we can include any properly encoded data 
    //in them

    println!("\ns: \"{}\"\ndata: \"{}\"\ns1: \"{}\"\ns2: \"{}\"", s, data, s1, s2);

    //updating a string

    //a String can grow in size and its contents can change, just like the
    //contents of a Vec<T>, if you push more data into it
    //in additions you can use the + operator or the format! macro to concatenate
    //String values

    //appending to a string with push_str and push

    //you can grow a String by using the push_str() method to append a string slice

    let mut s3 = String::from("foo");
    s3.push_str("bar");
    
    println!("\ns3: \"{}\"", s3);
    //the push_str() method takes a string slice because we don't necessarily want
    //to take ownership of the parameter

    let mut s4 = String::from("foo");
    let s5 = "bar";
    s4.push_str(s5);

    println!("s4: \"{}\"s5: \"{}\"", s4, s5);

    //the push() method takes a single character as a parameter and adds it to the
    //String

    let mut s6 = String::from("lo");
    s6.push('l');

    println!("s6: \"{}\"", s6);

    //concatenation with the + operator or the format! macro

    //if we want to combine 2 existing strings, one way is to use the + operator

    let _string0 = String::from("hello ");
    let _string1 = String::from("world!");

    let string2 = _string0 + &_string1;

    println!("\nstring2: \"{}\"", string2);

    //string2 will contain "hello world!" as a result of this code
    //the reason string0 is no longer valid after the addition and the reason we used 
    //a reference to string1 has to do with the signature of the method that gets
    //called when we use the + operator
    //
    //the + operator uses the add method, whose signature looks something like this:
    //fn add(self, s: &str) -> String {}
    //(this isn't the exact signature that's in the std lib, because in std add is 
    //defined using generics)
    //
    //the second parameter has an &, meaning that we're adding a reference of the
    //second string to the first string
    //we can only add a &str to a String
    //but we used a &String value as second parameter and not &str, why rustc compiles?
    //the reason we're able to use &string1 in the call to add is that the compiler
    //can coerce the &String argument into a &str
    //when we call the add method rust uses a deref coercion which here turns &sring1
    //into &string1[..]
    //
    //the implementation is more efficient than copying

    //multiple concatenation
    
    //using the + operator
    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = String::from("toe");

    let fs = s7 + " " +  &s8 + " " + &s9;

    println!("\nfs: \"{}\"", fs);

    //using the format! macro
    let s72 = String::from("tic");
    let s82 = String::from("tac");
    let s92 = String::from("toe");

    let fs2 = format!("{} {} {}", s72, s82, s92);

    println!("fs2: \"{}\"", fs2);
    //the format! macro works in the same way as println!
    //it returns a String with contents
    //the version of code using format! is much easier to read and doesn't take 
    //ownership of any of its parameters
    
    //indexing into Strings

    //in many other programming languages, accessing individual characters in a string
    //by referencing them by index is a valid and common operation
    //however, if you try to access aprts of a String using indexing syntax in
    //tust, you'll get an error
    //rust strings don't support indexing because of way that rust stores strings
    //in memory
    
    //internal representation

    //a String is a wrapper over a Vec<u8>
    let len = String::from("hello").len();
    println!("\nlen: {}", len);
    //in this case len will be 5 which means the vector storing the string "hello"
    //is 5 bytes long
    //in this case every letter takes 1 byte when encoded in UTF-8

    let len2 = String::from("Здравствуйте").len();
    println!("len2: {}", len2);
    //in this case each unicode scalar value in that string takes 2 bytes of storage
    //therefore an index into the string's bytes will not always correlate to a 
    //valid unicode scalar value

    //to avoid returning unexpected values and causing bugs taht might not be discovered
    //immediately, rust doesn't compile the following code
    //
    //let hello = "hello";
    //let letterh = &hello[0];

    //bytes and scalar values and grapheme clusters

    //rust looks at strings in 3 ways: as bytes, scalar values and grapheme clusters
    //
    //let's traduce the Hindi word “नमस्ते” representation in bytes, scalar values 
    //and grapheme clusters
    //
    //(in bytes -> vector of u8 values)
    //[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
    //224, 165, 135]
    //
    //(in unicode scalar values -> rust's char type)
    //['न', 'म', 'स', '्', 'त', 'े']
    //
    //(in grapheme clusters -> human readable letters)
    //["न", "म", "स्", "ते"]

    //a final reason rust doesn't allow us to index into a String to get a characters 
    //is that indexing operations are expected to always take constant time O(1)
    //but it isn't possible to guarantee that performance with a String because rust 
    //would have to walk through the contents from the beginning to the index to
    //determine how many valid characters there were

    //slicing strings

    //indexing into a string is ofter a bad idea because it's not clear that the return
    //type of the string-indexing operations should be: a byte value, a character,
    //a grapheme cluster, or a string slice
    //if you want to use a string slice you need to be more precise in your indexing
    //and use [] with a range to create a string slice containing particular bytes
    
    let sstring = "Здравствуйте";
    let stringslice = &sstring[0..4];
    println!("\nstringslice: \"{}\"\n", stringslice);
    //if we use &sstring[0..1] rust would panic at runtime because of the 2 bytes per
    //letter representation that we explain before

    //methods for iterating over strings

    //if you need to perform operations on individual unicode scalar values (access 
    //elements in a string) you can use the chars method (best way)
    
    for c in "Здрав".chars() {
        print!("{} ", c);
    }
    
    println!("");

    //the bytes method returns each raw byte of the string
    for c in "Здрав".bytes() {
        print!("{} ", c);
    }

    println!("");

    //remember that unicode scalar values may be made up of more than 1 byte
    //getting grapheme clusters from strings is complex so this functionality is not
    //provided by the std lib (look at crates available on crates.io for this 
    //functionality)
}
