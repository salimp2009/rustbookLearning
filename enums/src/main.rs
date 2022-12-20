#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
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
}
