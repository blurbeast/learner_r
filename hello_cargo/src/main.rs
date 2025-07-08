// fn main() {
//     // let x = 5;
//     // println!("The value of x is: {}", x);

//     // // will throw an error
//     // // x = 6;

//     // let mut x = 6;
//     // println!("The value of x is: {}", x);
//     // x = 7;
//     // println!("The value of x is: {}", x);


//     // shadowing
//     let x = 5;
//     println!("The value of x is: {}", x);
//     let x = x + 1;
//     println!("The value of x is: {}", x);
//     let x = x * 2;
//     println!("The value of x is: {}", x);
// }


fn main() {

    let tup: (i32, u32) = (-8, 10);
    println!("The value of tup is: {:?}", tup);

    // destructuring the tuple
    let (x, y) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    // accessing tuple elements
    println!("The first element of tup is: {}", tup.0);
    println!("The second element of tup is: {}", tup.1);
}