#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

//methods are similar to functions but they are different
//methods are defined within the context of a struct 
//(or enum, or trait object)
//the first method parameter is always self, which represents the 
//instance of the struct the method is being called on

//let's use impl to start an implementation block
//let's insert the area function within the impl block
//we bind the impl block to the Rectangle struct
//we change all the parameters (in the signature and in the body of 
//of the method) to self
impl Rectangle {
    //in the signature we can use &self instead of rectangle
    //rust knows automatically tha the type of self is Rectangle
    //due to this method's being inside the impl Rectangle context

    //we use the &(reference) operator because the methods can take
    //ownership of self, borrow self immutably or borrow self mutably
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //method with multiple parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //associated functions
    //they are functions (NOT methods) that are defined within impl
    //block but they don't take self as parameter
    //they are associated with the struct but they don't have an 
    //instance of the struct to work with (ex. String::from())
    fn square(size:u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

//we can use multiple impl blocks for the same struct
impl Rectangle {
    fn constructor(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height
        }
    }
}

fn main() {

    //methods are commonly used because of the method syntax
    //the use of methods within our code also improves the structure
    //of our project
    let rect = Rectangle {
        width: 50,
        height: 40
    };

    //we use method syntax to call the area method on our Rectangle
    //instance
    //we use the dot operator to access the area method defined before
    //within the impl block -> rect.area()
    println!("\nrect area: {}", rect.area());

    //in c and c++ there are 2 different operators for calling methods
    //. operator is used to call a method on the object directly
    //-> operator is used to call a method on a pointer to the object
    //and need to dereference the pointer first
    
    //rust uses a feature called "automatic referencing and dereferencing"
    //when we call a method with object.something() rust automatically
    //adds in &, &mut, or * so object matches the signature of the method

    //methods with more parameters
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("\nrect1 -> rect2? {}", rect1.can_hold(&rect2));
    println!("rect1 -> rect3? {}", rect1.can_hold(&rect3));

    //associated functions
    //to call associated functions we use the :: syntax
    //we use :: syntax also for namespaces created by modules
    //the function is namespaced by the struct
    let sq = Rectangle::square(30);
    println!("\nsq {:#?}", sq);

    //using constructor
    let rectc = Rectangle::constructor(100, 90);
    println!("\nrectc {:#?}", rectc);
    println!("rectc area: {}", rectc.area());
}
