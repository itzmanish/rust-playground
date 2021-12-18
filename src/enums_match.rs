// enums and Match
#[derive(Debug)]
enum IPAddr {
    V4(String),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(IPAddr),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(ipaddr) => match ipaddr {
            IPAddr::V4(_) => 4,
            IPAddr::V6(_) => 6,
        },
    }
}

fn main() {
    let ip = IPAddr::V4(String::from("127.0.0.1"));
    let ipv6 = IPAddr::V6(String::from("::0"));

    println!("Ip v4 address is {:?}", ip);
    println!("Ip v6 address is {:?}", ipv6);

    let value = value_in_cents(Coin::Quarter(ipv6));
    println!("lets get the type of ip {}", value)
}
