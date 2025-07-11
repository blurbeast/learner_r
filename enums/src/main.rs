


// #[derive(Debug)]
// enum IpAddrKind {
//     v4,
//     v6
// }

// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn main() {
//     let four: IpAddrKind = IpAddrKind::v4;
//     let six: IpAddrKind = IpAddrKind::v6;

//     let home: IpAddr = IpAddr {
//         kind: four,
//         address: String::from("127.0.0.1"),
//     };

//     let loopback: IpAddr = IpAddr {
//         kind: six,
//         address: String::from("::1"),
//     };


//     // println!("Home IP: {:?}", home);
//     // println!("Loopback IP: {:?}", loopback);
//     dbg!(home);
//     // dbg!(loopback);
// }


// enum IpAddrKind {
//     V4(String),
//     V6(String),
// }

// fn main() {

//     let home: IpAddrKind = IpAddrKind::V4(String::from("127.0.0:1"));
//     let loopback: IpAddrKind = IpAddrKind::V6(String::from("::1"));
// }


fn main() {
    // dealing with option types 
    let x: Option<u8> = Some(5);
    let y: u8 = 4;

    // in the above we have two variables x and y and while x is an Option<u8> type, y is a u8 type.
    // hence we can say x and y are not of the same type 

    // while x is a vairiant of the option type of u8 , y is a u8 type.

    let sum: u8 = x.unwrap() + y;

    println!("The sum is: {}", sum);
}