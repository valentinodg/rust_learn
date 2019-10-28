// COMMON COLLECTIONS

// rust's std lib includes a number of very useful
// data structures called collections
// most other data types represent 1 specific value
// but collections can contain multiple values
// unlike the built-in array and tuple types, the data
// these collections point to is stored on the heap,
// which means the amount of data dows not need to be
// known at compile time and can grow or shrink as the
// program runs

fn main() {
    // STRING LISTS OF VALUES WITH VECTORS

    // Vec<T> is a collection type known as vector
    // vectors allow you to store more than 1 value in a
    // single data structure that puts all the values next
    // to each other in memory
    // they are useful when you have a list of items

    //CREATING A NEW VECTOR

    // let's create a new empty vector calling the
    // Vec::new function
    let v0: Vec<i32> = Vec::new();
    println!("\nv0: {:?}", v0);
    // we added a type annotation -> <i32>
    // because we aren't inserting any values into this
    // vector, rust doesn't know what kind of elements we
    // intend  to store
    // vectors are implemented using generics
    // the Vec<T> type provided by the std lib can hold
    // any type
    // when a vector holds a specific type, the type
    // is specified within {}

    // rust can often infer the type of calue you want
    // to store once you insert values
    // the Vec<i32> annotation is very rare in practice
    // it's more common to create a Vec<T> that has
    // initial values and tust provides the vec! macro
    // for convenience
    // the macro will create a new vector that holds
    // the values you give it
    let v1 = vec![1, 2, 3];
    println!("v1: {:?}", v1);
    // rust can infer now that the type of v2 is Vec<i32>

    // UPDATING A VECTOR

    // let's use the push method
    let mut v2 = Vec::new();

    v2.push(1);
    v2.push(2);
    v2.push(3);
    v2.push(4);
    v2.push(5);

    println!("v2: {:?}", v2);
    // as any variable if we want to be able to change
    // its value we need to make it mutable using the mut
    // keyword
    // rust can infer the type so you don't need the
    // type annotation Vec<i32>

    // DROPPING A VECTOR DROPS ITS ELEMENTS

    // like any other struct a vector is freed when it
    // goes out of scope
    {
        let _v3 = vec![1, 2, 3, 4];
    } // v3 goes out of scope and is freed here
      // when the vector is dropped all of its contents are
      // also dropped

    // READING ELEMENTS OF VECTORS

    // there are 2 ways to reference a value stored in a
    // vector:
    // the indexing syntax and
    // the get method
    let v3 = vec![1, 2, 3, 4, 5, 6];

    let third: &i32 = &v3[2];
    println!("\nv3: {:?}\nv3[2]: {}", v3, third);

    match v3.get(2) {
        Some(third) => println!("thirs elem: {}", third),
        None => println!("this isn't the third elem"),
    }
    // vector are indexed by number starting at 0

    // indexing syntax method:
    // we use & and [] to get the
    // 3rd element, this operation gives us a reference

    // get method:
    // we pass the index as an argument of the get function,
    // this operation gives us an Option<&T>

    // rust has 2 ways to reference an element so you can
    // choose how the program behaves when you try to use
    // an index value that the vector doesn't have an
    // element for
    //  let v4 = vec![1,2,3,4,5];

    // this method will cause the program to panic because
    // it references a nonexistent element (this method is
    // best used when you want your program to crash)
    //  let _not_exist = &v4[100];

    // when the get method is passed an index that is outside
    // the vector, it returns None without panicking (you
    // would use this method if accessing an element beyond
    // the range of the vector happens occasionally under
    // normal circumstances)
    //  let _not_exist = v4.get(100);

    // this code won't compile
    //  let mut v = vec![1,2,3,4];
    //  let first = &v[0];
    //  v.push(5);
    //  println!("first elem: {}", first);
    // because the ownership/borrowing rules are still valid
    // we can't have mutable and immutable references in
    // the same scope (v, first respectively)

    // this error is due to the way vectors works: adding
    // a new element onto the end of the vector might require
    // allocating new memory and compying the old elements to
    // the new space, if there isn't enough room to pull all
    // the alements next to each other where the vector
    // currently is
    // in that case the reference to the first element would
    // be pointing to deallocated memory and the borrowing
    // rules prevent programs from ending up in that situation

    // ITERATING OVER THE VALUES IN A VECTOR

    // let's use a for loop to get immutable references to
    // each element in a vectory of i32 and print them
    let v4 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    print!("\nv4: [ ");
    for i in &v4 {
        print!("{} ", i);
    }
    println!("]");

    // let's iterate over mutable references to each element
    // in mutable vector in order to make changes to all the
    // elements
    let mut v5 = vec![0, 50, 100, 150, 200, 250];
    println!("\nv5: {:?}", v5);
    for i in &mut v5 {
        *i += 50;
    }
    println!("v5: {:?} (new)", v5);
    // we can refer to the value of the mutable element using
    // the dereference operator *
    // += is the increment operator

    // USING ENUMS TO STORE MULTIPLE TYPES

    // vector can only store values that are the same type
    // (this is really inconvenient)
    // when we need to store elements of a different type in a
    // vector we can define and use an enum
    // we can store a list of items of different types
    // from the point of view of the vector there will be only
    // enum types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("\nrow: {:?} (vector of enums)", row);
    // rust needs to know what types will be in the vector at
    // compile time so it knows exactly how much memory
    // on the heap will be needed to store each element
    // we can be explicit about what types are allowed in
    // this vector
    // rust prevents errors resulting from the use of vectors
    // that can hold any(different) types
    // using an enum plus a match expression means that rust
    // will ensure at compile time that every possbile case
    // is handled

    // if you're writing a program, if you don't know the
    // exhaustive set of types the program will get at
    // runtime to store in a vector, the enum technique won't
    // work

    // see API: there are some other methods like pop
}
