

// fn main() {
//     let mut s: String = String::from("hello world");
//     println!("The value of s is: {}", s);
//     println!("The length of s is: {}", s.len());
//     println!("The capacity of s is: {}", s.capacity());

//     s.push_str(" from Rust!!");
//     println!("The value of s after push_str is: {}", s);
//     println!("The length of s after push_str is: {}", s.len());
//     println!("The capacity of s after push_str is: {}", s.capacity());
// }

// fn main() {
//     let s: String = String::from("hello");
//     println!("The value of s is: {}", s);
//     take_ownership(s);
    
//     // a compile error occur because ownership has been moved
//     // println!("The value of s after ownership is taken: {}", s);
// }


// fn take_ownership(s: String) {
//     // ownership taken here 
//     println!("{s}")
// }


// fn main() {

//     // mutable reference 
//     let mut s: String = String::from("i am ");
//     change_something(&mut s);
//     println!("The value of s after change_something is: {}", s);
// }

// fn change_something(s: &mut String) {
//     s.push_str("here")
// }

fn main() {

    let mut s: String = String::from("hello world");
    println!("The value of s is: {}", s);

    // let s_1= &mut s; // mutable reference
    // s_1.push_str(" from Rust!!");
    // println!("The value of s after push_str is: {}", s_1);
    let s_2 = &s; // immutable reference
    let s_3 = &s; // another immutable reference
    let s_1 = &mut s; // mutable reference
    s_1.push_str(" from Rust!!");
    println!("The value of s after push_str is: {}", s_1);
    println!("The value of s_2 is: {}", s);
}