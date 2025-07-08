fn main() {
    // let x = 5;
    // println!("The value of x is: {}", x);

    // // will throw an error
    // // x = 6;

    // let mut x = 6;
    // println!("The value of x is: {}", x);
    // x = 7;
    // println!("The value of x is: {}", x);


    // shadowing
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is: {}", x);
    let x = x * 2;
    println!("The value of x is: {}", x);
}
