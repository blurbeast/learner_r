// fn main() {
//     println!("Hello, world!");
// }

use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn main() {

    let mut score: HashMap<String, u32> = HashMap::new();
    score.insert("a".to_string(), 1);
    score.insert("b".to_string(), 2);

    println!("{:?}", score);

    let result: Option<&u32> = score.get("a");
    println!("{:?}", result.unwrap());

    let a_entry: Entry<String, u32> = score.entry("b".to_string());

    let a_value: &mut u32 = a_entry.or_insert(0);
    *a_value += 7;

    println!("{:?}", score);


    let v: Vec<u32> = Vec::new();

    // v.retain()
}