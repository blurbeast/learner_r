// fn main() {
//     println!("Hello, world!");
// }

use std::fmt::Formatter;

fn main() {

    //understanding arrays and vectors
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let vec = vec![1, 2, 3, 4, 5];

    // printing them to the console 
    println!("Array: {:?}", arr);
    println!("Vector: {:?}", vec);


    // there are many various ways to create an array and vectors

    // the below prints 1 only to the number of lengths specified
    let mut arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    println!("Array: {:?}", arr);

    // to access an array, we use the index base
    let index_0_value: i32 = arr[0];
    println!("Index 0: {:?}", index_0_value);

    // changing the value of an array at an index, we use the index accessor
    arr[0] = arr[0] + 1;
    arr[1] = arr[1] + 12;

    // now the values in the array have changed
    println!("array is : {:?}", arr);

    // let arr = [];
    // we also have built in methods of arrays

    // this returns the value at the specified index
    let found_index_value: Option<&i32> = arr.get(0);

    match found_index_value {
        Some(va) => println!("Index 0: {:?}", va),
        None => println!("None"),
    }

    // the below return a boolean value that checks if it is empty or not
    let check_if_empty: bool = arr.is_empty();
    println!("is empty {:?}", check_if_empty);

    let result = arr.as_mut_slice();
    println!("slice {:?}", result);
    let is_value_there: bool = result.contains(&1);

    println!("is_value_there {:?}", is_value_there);

    // this below reverses the entire elements in the collection
    result.reverse();

    println!("Reverse {:?}", result);

    // let ch: Chunks<i32> = arr.chunks(3).enumerate().collect();
    // println!("Chunked: {:?}", ch);

    let first_element: Option<&i32> =result.first();
    println!("first element is {:?}", first_element);


    // the fill gives each element a default of the specified value, also overriding the previous value in the collection
    result.fill(50);
    println!("after fill : {:?}", result);

    
}
