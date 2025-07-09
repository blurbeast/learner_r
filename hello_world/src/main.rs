// fn main() {
//     // println!("Hello, world!");

//     // const variable cannot be shadowed
//     const X : i32 = 5;
//     println!("The value of X is: {}", X);

//     // const X: i32 = 3;
//     // println!("The value of X is: {}", X);

//     //

//     let f: bool = true;
//     println!("The value of f is: {}", f);

//     // character
//     let c: char = 'a';
//     println!("The value of c is: {}", c);

// }


// fn main() {

//     // array
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//     println!("The value of arr is: {:?}", arr);

//     let arr2: [i32; 5] = [1; 5]; // all elements are initialized to 1
//     println!("The value of arr2 is: {:?}", arr2);

// }


// ownership
fn main() {
    // there are three rules which govern rust ownership
    // 1. every value must have an owner
    // 2. there can only be one owner at a time
    // 3. when an owner is out of scope it can then be dropped 

    let s = String::from("here is a string");

    // a value can be borrowed multiple times
    borrow(&s);
    borrow(&s);
    borrow(&s);

    // when the value is moved it can neither be borrowed nor moved again
    moved(s);


    let x = String::from("hello world");
    let y = x.clone();
    let z  = x;


    // panic occurs
    // moved(s);

}

fn borrow(s: &String) {
    println!("{}", s)
}

fn moved(s: String) {
    println!("{} , {}", s.len(), s.capacity())
}