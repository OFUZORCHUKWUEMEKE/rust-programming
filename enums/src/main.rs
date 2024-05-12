// IpAddrKind is now a custom data type that we can use elsewhere in our code.IpAddrKind is now a custom data type that we can use elsewhere in our code

fn main() {
    println!("whats popping guys");

    enum IpAddrKind {
        v4,
        v6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    struct QuitMessage;
    struct MoveMessage {
        x: i32,
        y: i32,
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();

    // let HOME = IpAddr {
    //     kind: IpAddrKind::v4,
    //     address: String,
    // };
    // let LOOPBACK = IpAddr {
    //     kind: IpAddrKind::v6,
    //     address: String::from("::1"),
    // };

    // enum IpAddress{
    //     v4(u8,u8,u8,u8),
    //     v6(String)
    // }

    // enum IpAddr {
    //     v4(String),
    //     v6(String),
    // }

    // let home = IpAddr::v4(String::from("127.0.0.1"));

    // let loopback = IpAddr::V6(String::from("::1"));

    // THE OPTION ENUM 

    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
}
