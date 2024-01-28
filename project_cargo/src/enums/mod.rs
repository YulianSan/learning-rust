use std::net::Ipv4Addr;

pub fn example1() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4 = 4,
        V6 = 6,
    }

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let job = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    println!("{:?}", home);
    println!("{:?}", job);
    println!("number: {:?}", IpAddrKind::V4 as i32);
    println!("string: {:?}", IpAddrKind::V4);
}

pub fn example2() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddrKind::V4(127, 0, 0, 1);
    let job = IpAddrKind::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", job);
    println!("{:?}", String::from("127.0.0.1").parse::<Ipv4Addr>());
}
