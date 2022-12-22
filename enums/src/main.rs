use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
#[allow(dead_code)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
#[allow(dead_code)]
enum IpAddrNew {
    V4(String),
    V6(String),
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum IpAdrNew2 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[derive(Debug)]
#[allow(dead_code)]
enum IpAdrNew3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
#[allow(dead_code)]
enum Message {
    Quit,                       // unit struct
    Move { x: i32, y: i32 },    // struct
    Write(String),              // tuple struct
    ChangeColor(i32, i32, i32), // like a tuple struct; struct Color(...)
}

#[allow(dead_code)]
impl Message {
    fn call(&self) { /* to be implemented */
    }
}

fn route(ip_kind: IpAddrKind) {
    println!("ip kind : {:?}", ip_kind);
}

fn main() {
    let vfour_ip = IpAddrKind::V4;
    let vsix_ip = IpAddrKind::V6;
    route(vfour_ip);
    route(vsix_ip);
    route(IpAddrKind::V4);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    println!("home router add : {:?}", home);

    let _home = IpAddrNew::V4(String::from("127.0.0.1"));
    let _local = IpAddrNew::V6(String::from("::1"));

    let localhost = Ipv4Addr::new(127, 0, 0, 1);
    assert_eq!("127.0.0.1".parse(), Ok(localhost));
    assert!(localhost.is_loopback());
    assert!("012.004.002.000".parse::<Ipv4Addr>().is_err()); // all octets are in octal
    assert!("0000000.0.0.0".parse::<Ipv4Addr>().is_err()); // first octet is a zero in octal
    assert!("0xcb.0x0.0x71.0x00".parse::<Ipv4Addr>().is_err()); // all octets are in hex
                                                                //
    let new_home = IpAdrNew2::V4(Ipv4Addr::new(127, 0, 0, 1));
    println!("new_home : {:?}", new_home);

    let home_new2 = IpAdrNew3::V4(127, 0, 0, 1);
    println!("new_home2: {:?}", home_new2);

    let message1 = Message::Write(String::from("first commit"));
    message1.call();

    let _some_number = Some(25);
    let some_char = Some('r');

    let result = some_char.map_or('E', |x| x);
    println!("result some_char: {result}");

    let x = Some("foo");
    assert_eq!(x.map_or(42, |v| v.len()), 3);

    let x: Option<&str> = None;
    assert_eq!(x.map_or(42, |v| v.len()), 42);
    let absent_number: Option<i32> = None;

    if let Some(x) = absent_number {
        println!("Some(x): {x}");
    } else {
        println!("no value(None)");
    }
}
