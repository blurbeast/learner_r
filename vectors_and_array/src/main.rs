// fn main() {
//     println!("Hello, world!");
// }

// fn main() {
//
//     //understanding arrays and vectors
//     let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//     let vec = vec![1, 2, 3, 4, 5];
//
//     // printing them to the console
//     println!("Array: {:?}", arr);
//     println!("Vector: {:?}", vec);
//
//
//     // there are many various ways to create an array and vectors
//
//     // the below prints 1 only to the number of lengths specified
//     let mut arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//
//     println!("Array: {:?}", arr);
//
//     // to access an array, we use the index base
//     let index_0_value: i32 = arr[0];
//     println!("Index 0: {:?}", index_0_value);
//
//     // changing the value of an array at an index, we use the index accessor
//     arr[0] = arr[0] + 1;
//     arr[1] = arr[1] + 12;
//
//     // now the values in the array have changed
//     println!("array is : {:?}", arr);
//
//     // let arr = [];
//     // we also have built in methods of arrays
//
//     // this returns the value at the specified index
//     let found_index_value: Option<&i32> = arr.get(0);
//
//     match found_index_value {
//         Some(va) => println!("Index 0: {:?}", va),
//         None => println!("None"),
//     }
//
//     // the below return a boolean value that checks if it is empty or not
//     let check_if_empty: bool = arr.is_empty();
//     println!("is empty {:?}", check_if_empty);
//
//     let result = arr.as_mut_slice();
//     println!("slice {:?}", result);
//     let is_value_there: bool = result.contains(&1);
//
//     println!("is_value_there {:?}", is_value_there);
//
//     // this below reverses the entire elements in the collection
//     result.reverse();
//
//     println!("Reverse {:?}", result);
//
//     // let ch: Chunks<i32> = arr.chunks(3).enumerate().collect();
//     // println!("Chunked: {:?}", ch);
//
//     let first_element: Option<&i32> =result.first();
//     println!("first element is {:?}", first_element);
//
//
//     // the fill gives each element a default of the specified value, also overriding the previous value in the collection
//     result.fill(50);
//     println!("after fill : {:?}", result);
//
// }


fn main() {
    let mut v: Vec<i32> = vec![1, 3, 1, 6, 1, 5];
    println!("{:?}", v);

    // the sort method sorts all the elements in the collection in an ordering manner
    v.sort();

    println!("after sorting : {:?}", v);
    println!("capacity is ::: {}", v.capacity());

    // the try reserve increases the capacity of the vector
    v.try_reserve(4).unwrap();

    println!("after reserving ::: {}", v.capacity());

    let result = v.iter().len();
    println!("{:?}", result);

    // the method returns exact elements in the number of specified times but doesn't change the real collection
    let repeat = v.repeat(5);
    println!("repeat :: {:?}", repeat);

    println!("after repeating ::: {:?}", v);

    // the dedup uses the partialEq to check if there is any duplicate value in the list
    v.dedup();

    println!("{:?}", v);

    // the index method takes two arguments which are the index which specify the particular index we want to alter and the value which is used to replace the previous value there .
    // the limitation to insert is that even though it was vectors are of growable sizes, if the index does not exist or not previously filled with a value, it panics
    v.insert(0, 80);

    println!("{:?}", v);

    // the way the append method works is that it combines two mutable vectors of the same data types together, which apparently increases the length and capacity of the vector
    let mut v_2 = vec![8, 9, 10];
    v.append(&mut v_2);

    println!("after appending :: {:?}", v);

    // v.iter().enumerate()

}
