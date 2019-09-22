//stucts -> similar to tuples but more flexibles
//each value in the struct have a name
//the values in the struct can be of different types
//the order of the data to specify or access the values
//of an instance is not important
//struct definition
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    
    //struct instance creation
    let user1 = User {
        email: String::from("abc@abc.com"),
        username: String::from("abc"),
        active: true,
        sign_in_count: 1
    };
    //struct mutable instance creation
    let mut user2 = User {
        email: String::from("ddd@ddd.com"),
        username: String::from("ddd"),
        active: true,
        sign_in_count: 1
    };

    println!("\nuser1 (immutable):\n\tusername: \"{}\"\n\temail: \"{}\"\n\tactive: {}\n\tsign_in_count: {}", user1.username, user1.email, user1.active, user1.sign_in_count);

    println!("\nuser2 (mutable):\n\tusername: \"{}\"\n\temail: \"{}\"\n\tactive: {}\n\tsign_in_count: {}", user2.username, user2.email, user2.active, user2.sign_in_count);
    
    //change user2.email value (user2 -> mutable struct)
    user2.email = String::from("zzz@zzz.com");
    println!("\nuser2.email [changed](mutable): \"{}\"", user2.email);

    //all the struct must be mutable
    //rust doesn't allow us to mark only certain fields as mutable

    //dinamyc struct builder
    let userd: User;
    let usernamed = String::from("qqq");
    let emaild = String::from("qq@qq.com");
    userd = build_struct(usernamed, emaild);

    println!("\ndynamic_user (immutable):\n\tusername: \"{}\"\n\temail: \"{}\"\n\tactive: {}\n\tsign_in_count: {}", userd.username, userd.email, userd.active, userd.sign_in_count);

    //create an instance from other instances 
    //struct update syntax
    let user3 = User {
        username: String::from("ff"),
        email: user2.email,
        ..user1
        //or
        //active: user2.active,
        //sign_in_count: user2.sign_in_count
    };

    println!("\nuser3 (immutable):\n\tusername: \"{}\"\n\temail: \"{}\"\n\tactive: {}\n\tsign_in_count: {}", user3.username, user3.email, user3.active, user3.sign_in_count);

    //tuple structs definition
    struct Color(i32,i32,i32);
    struct Point(i32,i32,i32);

    let _black = Color(0,0,0);
    let _origin = Point(0,0,0);
    //tuple structs are similar to tuples
    //. operator to access (index) an individual value
    //you can destructure the tuple structs like tuples
    //the types of the tuples are different

    //unit-like structs (without any fields)
    //it behave similarly to (), the unit type
    //unit-like structs are useful in situation in which you
    //need to implement a trait on some type but you don't
    //have any data that you want to store in the type itself

    //ownership of struct data
    //in the user struct definition we used the String type rather
    //than the &str string slice type
    //because we want instances of the struct to own all of its 
    //data and for that data to be valid for as long as the entire
    //struct is valid
    //is possible to store a reference of data in structs
    //using lifetimes (we will see it later)

}

fn build_struct(email: String, username: String) -> User {
    User {
        //field init shorthand 
        //(only when variables and fields have the same name)

        //email: email,
        email,
        //username: username,
        username,
        active: true,
        sign_in_count: 1
    }
}