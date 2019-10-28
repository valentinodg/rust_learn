fn main() {
    // STORING KEYS WITH ASSOCIATED VALUES IN HASH MAPS

    // the type HashMap<K,V> stores a mapping of keys of type K to values of type V.
    // it does this via a hashing function that determines how it places there keys and values
    // into memory
    // hash maps  are useful when you want to look up data not by using an index, as you can do with
    // vectors, but by using a key that can be of any type

    // CREATING A NEW HASH MAP

    // you can create an empty hash map with new and add elements with insert

    use std::collections::HashMap;

    let mut scoresx = HashMap::new();

    scoresx.insert(String::from("blue"), 10);
    scoresx.insert(String::from("yellow"), 50);

    println!("\n{:?}", scoresx);
    // note that we need to first use the HashMap from the collections portion of the std lib
    // because of our 3 common collections, this one is the least ofter used so it's not included
    // int the features brought into scope automatically in the prelude
    // hash maps also have less support from the std lib (for example there's no built-in macro
    // to construct them)

    // just like vectors, hash maps store their data on the heap
    // like vectors, hash maps are homogeneous: all of the keys must have the same type and all of
    // the values must have the same type

    // another way of constructing a hash map is by using the collect method on a vector of tuples
    // (where each tuple consists of a key and its value)
    // the collect method gathers data into a number of collection types, including hash maps

    // if we have a vector of keys and a vector of values, we can use the zup method to create a
    // vector of tuples and then we can use the collect method to create our hash map

    let teams = vec![String::from("blue"), String::from("yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("\n{:?}", scores);
    // the type annotation HashMap<_,_> is needed here because it's possible to collect into many
    // different data structures and rust doesn't know wichi you want unless you specify
    // for the paramenters for the key and values types we use _ and rust can infer the types that
    // the hash map contains based on the types of the data in the vectors

    // HASH MAPS AND OWNERSHIP

    // for types that implements the copy trait, like i32, the values are copied into the hash map
    // for owned values like String, the values will be moved and the hash map be the owner of
    // those values

    let field_name = String::from("color");
    let field_value = String::from("blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("\n{:?}", map);
    // we aren't able to use the variables field_name and field_value after they've been moved into
    // the hash map with the call to insert
    // if we insert references to values into the hash map, tha values won't be moved into the hash
    // map
    // the values that the references point to must be valued for at least as long as the hash map
    // is valid ("validating references with lifetimes")

    // ACCESSING VALUES IN HASH MAP

    // we can get a value out of the hash map by providing its key to the get method

    let team_name = String::from("blue");
    let score2 = scores.get(&team_name);

    println!("\n{:?}\n", score2);
    // here, score2 will have the value that's associated with the blue key and the result will be
    // Some(&10)
    // the result is wrapped in Some because get returns an Option(&V) or None (if there's no value
    // for that key in the hash map)

    // we can iterate over each key/value pair in a hash map in a similar manner as we fo with
    // vectors, using a for loop
    let mut scores2 = HashMap::new();

    scores2.insert(String::from("blue"), 10);
    scores2.insert(String::from("yellow"), 50);

    for (key, value) in &scores2 {
        println!("{}: {}", key, value);
    }

    // UPDATING A HASH MAP

    // although the number of keys and values is growable, each key can only have 1 value
    // associated with it at a time

    // OVERWRITING A VALUE

    // if we insert a key and a value into a hash map and then insert that same key with a
    // differrent value, the values associated with that key will be replaced

    let mut scr = HashMap::new();

    scr.insert(String::from("blue"), 10);
    scr.insert(String::from("blue"), 25);

    println!("\n{:?}", scr);

    // INSERTING A VALUE IF THE KEY HAS NO VALUE

    // it's common to check whether a particular key has a value and, if it doesn't, insert a value
    // for it
    // hash maps have a special API for this called entry that takes the key you want to check as a
    // parameter
    // the return value of the entry method is an enum called Entry that represents a value that
    // might or might not exist

    let mut scr2 = HashMap::new();
    scr2.insert(String::from("blue"), 10);

    scr2.entry(String::from("yellow")).or_insert(50);
    scr2.entry(String::from("blue")).or_insert(50);

    println!("\n{:?}", scr2);
    // the or_insert method on Entry is defined to return a mutable reference to the value for the
    // corresponding Entry key if that key exist, and if not, inserts the parameter as the new
    // values for this key and returns a mutable reference to the new value
    // this technique is much cleaner than writing the logic ourselves and, in addition, plays more
    // nicely with the borrow checker

    // UPDATING A VALUE BASED ON THE OLD VALUE

    // another common use case for hash maps is to look a key's value and then update it based on
    // the old value

    let text = "hello world wonderfull world";

    let mut map2 = HashMap::new();

    for word in text.split_whitespace() {
        let count = map2.entry(word).or_insert(0);
        *count += 1;
    }

    println!("\n{:?}", map2);
    // the or_insert method actually returns a mutable reference (&mut V) to the value for this key

    // HASHING FUNCTIONS 
    
    // by default HashMap uses a cryptographically strong hashing function that can provide
    // resistance to DoS attacks
    // this is not the fastest hashing algorithm available, but the trade-off for better security
    // that comes  with the drop in performance is worth it
    // you can specify a difference hasher (look at libs in craters.io
    // a hasher is a type that implements the BuildHasher trait
}
