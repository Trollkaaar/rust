// fn main() {
// println!("Hello, world!");
// let home = IpAddr {
//     kind: IpAddrKind::V4(String::from("test")),
//     address: String::from("127.0.0.1"),
// };
// home.info();
// let default_ip = IpAddr::default_info();
// default_ip.info();
// }
// #[derive(Debug)]
// enum IpAddrKind {
//     V4(String),
//     V6(String),
// }
// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// impl IpAddr {
//     fn info(&self) {
//         println!("{:#?} {:#?}", self.kind, self.address)
//     }
// }

// impl IpAddr {
//     fn default_info() -> IpAddr {
//         IpAddr {
//             kind: IpAddrKind::V6(String::from("127.0.0.1")),
//             address: String::from("127.0.0.1"),
//         }
//     }
// }
#[derive(Debug)]
enum Coin {
    Penny(Rarity),
    Nickel(Rarity),
    Dime(Rarity),
    Quarter(Rarity),
}
#[derive(Debug)]
enum Rarity {
    Common,
    Uncommon,
    Rare,
    VeryRare,
    Epic,
    Legendary,
}

fn base_value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny(rarity) => {
            println!("A {:?} penny.", rarity);
            1
        }
        Coin::Nickel(rarity) => {
            println!("A {:?} penny.", rarity);
            5
        }
        Coin::Dime(rarity) => {
            println!("A {:?} penny.", rarity);
            10
        }
        Coin::Quarter(rarity) => {
            println!("A {:?} penny.", rarity);
            25
        }
    }
}

fn main() {
    base_value_in_cents(Coin::Penny(Rarity::Legendary));
    let five = Some(5);
    let none: Option<i32> = None;
    print!("{:?}", plus_one(five));
    print!("{:?}", plus_one(none));

    let some_value = Some(5);
    if let Some(5) = some_value {
        println!("five")
    }
}
fn plus_one(input: Option<i32>) -> Option<i32> {
    match input {
        Some(i) => Some(i + 1),
        _ => None,
    }
}
