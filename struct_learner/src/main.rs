// fn main() {
//     println!("Hello, world!");
// }


struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}


// fn main() {

//     let user: user = User {
//         username: String::from("blurbeast"),
//         active: true,
//         sign_in_count: 0,
//         email: String::from("email@blurbeast.co")
//     };
// }


// using the dot notation to access the fields of a struct
// and also changing the value of a field
fn main() {

    let mut user: User = User {
        username: String::from("blurbeast"),
        active: true,
        sign_in_count: 0,
        email: String::from("email@blurbeast.co"),
    };

    let new_email: String = String::from("blurrybeast@blurbeast.co");

    user.email = new_email;
}

// creating a function that builds the struxt
// because user does not have to decide the active and sign_in_count fields, we use default values

fn build_user_struct(username: String, email: String) -> User {
    // using this approach to build the struct is a really good practice
    // User{
    //     active: true,
    //     sign_in_count: 0,
    //     email: email,
    //     username: username,
    // }

    // but what about when we have more fields?
    // how do we cope with and how do we go about it ?
    // this is when we parameters naming carry the sname as the struct fields

    User{
        username,
        email,
        active: true,
        sign_in_count: 0,
    }

    // @Note: this is called field init shorthand
}