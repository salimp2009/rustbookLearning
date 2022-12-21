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

#[derive(Debug)]
#[allow(dead_code)]
enum IpAdrNew2 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
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
    assert_eq!(localhost.is_loopback(), true);
    assert!("012.004.002.000".parse::<Ipv4Addr>().is_err()); // all octets are in octal
    assert!("0000000.0.0.0".parse::<Ipv4Addr>().is_err()); // first octet is a zero in octal
    assert!("0xcb.0x0.0x71.0x00".parse::<Ipv4Addr>().is_err()); // all octets are in hex
                                                                //
    let new_home = IpAdrNew2::V4(Ipv4Addr::new(127, 0, 0, 1));
    println!("new_home : {:?}", new_home);
}
