// fn main() {
//     println!("Hello, world!");
// }


// fn main() {
//     // let x = 7;
//     // let y = x;
//     // let z = x;

//     // println!("{x}")


//     // let x = "string literal";
//     // let y = x;
//     // let z = x;

//     // println!("{x}")

//     // let x = "string literal".to_string();
//     // let y = &x;
//     // let z = &x;

//     // println!("{}", x)

//     // let x = vec![6, 7, 8];
//     // let y = &x;
//     // let z = &x;

//     // println!("{:#?}", x);

//     // let j = x;

//     // dealing with mutability

//     let mut x = "string literal".to_string();

//     // there can only be one mutable reference at a time
//     // let mut y: &mut String = &mut x;

//     // cannot borrow a mutable of x
//     // let mut z: &mut String = &mut y;
//     // x + &"rust".to_string();

//     x.push_str("xxxx");

//     println!("{:p}", &x);


//     let v = vec![1, 2, 3, 4, 5];
//     let x = vec![1, 2, 3, 4, 5];


//     println!("checking the address where it is located");
//     println!("v address ::: {:p}", &v);
//     println!("x address ::: {:p}", &x);


// }


fn main() {

}

struct Todo {
    x: u8,
}

impl Todo {

    fn new(self: Self) {
        
    }
    
}



#[cfg(test)]
mod test {
    use crate::Todo;


    // #[test]

    // helper function
    fn create_todo() -> Todo {
        Todo{
            x: 8,
        }
    }

    #[test]
    fn test_it() {
        let todo: Todo = create_todo();
        let x: u8 = todo.x;

        assert_eq!(x, 8);
    }


}