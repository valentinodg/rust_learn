//enum declaration
enum IpAddrKind {
    //enum variants declaration
    V4,
    V6
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

//enum with different variables
enum Message {
    _Quit, //no data associated
    _Move {x:i32, y: i32}, //anonymous struct
    Write(String), //string
    _ChangeColor(i32, i32, i32) //3 i32 values
}


fn main() {
    //let's enumerate the possibilities for IP type (v4 and v6)
    //this is a case in which enums are a better choice 
    //(instead of structs)
    //an IP can be either v4 or v6 but not both at the same time
    //so the enum data structure is appropriate
    
    //instances creation
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    //----------initialization using struct

    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String
    // }

    //----------instances declaration

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1");
    // }
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1");
    // }

    //we can do the same by using only enums
    //with enums each variant can have different types and 
    //amounts of associated data
    //(see "IpAddr" enum above)

    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    //store IP addresses and encode which kind they are is so common
    //the std lib has a definition we can use

    struct _Ip4Addr {
        //code
    }

    struct _Ip6Addr {
        //code
    }

    enum _IpAddr2 {
        V4(_Ip4Addr),
        V6(_Ip6Addr)
    }

    //(see enum "Message" above)
    //this definition is similar to defining different kinds of struct
    //definitions
    struct _QuitMessage;
    struct _MoveMessage {x:i32, y: i32}
    struct _WriteMessage(String);
    struct _ChangeColorMessage(i32, i32, i32);

    //route(IpAddrKind::V4);
    //route(IpAddrKind::V6);

    let m = Message::Write(String::from("hello"));
    m.call();

    //the Option enum and its advantages over null values
    //the null type does not exists in rust
    //rust have an enum that can encode the concept of a value
    //beeing present or absent
    //this enum is Option<T> and it is defined by the std lib
    enum _Option<T> {
        Some(T),
        None
    }
    //Option<T> is so useful that it's even included in the prelude
    //you can use Some and None directly without the Option:: prefix
    //Option is a std enum and Some(T) and None are std variants

    //<T> syntax express a generic type parameter
    
    //Some express the presence of the value (the value is held
    //within the Some)
    //None express that we don't have a valid value
    let _some_number = Some(5);
    let _some_string = Some("string");

    //as we can see, the assignment below fails and cause an error
    //because Option<i32> and i32 are different types
    //this is the main advantage of Option<T> that rust uses instead
    //of using null types
    // let absent_number: Option<i32> = 8;

    //the match expression is a control flow used in combination
    //with Option<T> values
}

//fn route(ip_kind: IpAddrKind) {}

//like struct impl, we can define methods for enums
impl Message {
    fn call(&self) {
        println!("call method inside Message impl enum");
    }
}