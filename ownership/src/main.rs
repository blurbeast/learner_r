

fn main() {
    let mut s: String = String::from("hello world");
    println!("The value of s is: {}", s);
    println!("The length of s is: {}", s.len());
    println!("The capacity of s is: {}", s.capacity());

    s.push_str(" from Rust");
    println!("The value of s after push_str is: {}", s);
    println!("The length of s after push_str is: {}", s.len());
    println!("The capacity of s after push_str is: {}", s.capacity());

}
