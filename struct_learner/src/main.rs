// fn main() {
//     println!("Hello, world!");
// }


// struct User {
//     email: String,
//     username: String,
//     active: bool,
//     sign_in_count: u64,
// }


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
// fn main() {

//     let mut user: User = User {
//         username: String::from("blurbeast"),
//         active: true,
//         sign_in_count: 0,
//         email: String::from("email@blurbeast.co"),
//     };

//     let new_email: String = String::from("blurrybeast@blurbeast.co");

//     user.email = new_email;
// }

// creating a function that builds the struxt
// because user does not have to decide the active and sign_in_count fields, we use default values

// fn build_user_struct(username: String, email: String) -> User {
//     // using this approach to build the struct is a really good practice
//     // User{
//     //     active: true,
//     //     sign_in_count: 0,
//     //     email: email,
//     //     username: username,
//     // }

//     // but what about when we have more fields?
//     // how do we cope with and how do we go about it ?
//     // this is when we parameters naming carry the sname as the struct fields

//     User{
//         username,
//         email,
//         active: true,
//         sign_in_count: 0,
//     }

//     // @Note: this is called field init shorthand
// }


// fn main() {

//     // using the build_user_struct function to create an instance of User
//     let first_user:User = build_user_struct(String::from("blubreast"), String::from("blurbeast@blurbeast.co"));

//     println!("First User: {}, {}, {}, {}", first_user.username, first_user.email, first_user.active, first_user.sign_in_count);

//     // creating another user
//     //using the instance of the first user to create another user
//     let second_user: User = User{
//         email: String::from("burry@blurbeast.co"),
//         // we use the .. know as the spread operator to fill in other fields
//         ..first_user
//     }
// }


// fn main() {


//     // we also have tuple structs
//     struct Color(i32, i32, i32);
//     struct Point(i32, i32, i32);

//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);


//     // when destructuring a tuple struct, we cdo it this way
//     let Color(r, g, b) = black;
//     println!("Color: {}, {}, {}", r, g, b);
//     let Point(x, y, z) = origin;
//     println!("Point: {}, {}, {}", x, y, z);
// }


// understanding why we used owned data types in the struct
// struct Subject {
//     name: String,
//     subject_code: &str,
//     credit_hours: u8,
//     mentor: String,
// }

// fn main() {
//     // we can use the owned data type in the struct
//     // this is because the struct will own the data and it will be dropped when it goes out of scope
//     let subject = Subject {
//         name: String::from("Computer Science"),
//         subject_code: "CS101",
//         credit_hours: 3,
//         mentor: String::from("Dr. Smith"),
//     };

//     println!("Subject: {}, {}, {}, {}", subject.name, subject.subject_code, subject.credit_hours, subject.mentor);
// }

// fn main() {
//     // using an example like rectangle 
//     let width: u32 = 30;
//     let height: u32 = 50;

//     let area_of_rectangle:u32 = area(width, height);

//     println!("The area of the rectangle is: {}", area_of_rectangle);

// }


// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }


// struct Rectangle(u32, u32);

// fn main() {

//     // using the field init shorthand to create an instance of Rectangle
//     let rectangle: Rectangle = Rectangle(30, 50);

//     let area_of_rectangle: u32 = area(rectangle);

//     println!("The area of the rectangle is: {}", area_of_rectangle);

// }

// fn area(rectangle: Rectangle) -> u32 {
//     rectangle.0 * rectangle.1
// }


// using tuple to build common related terms
// fn main() {
//     let width: u32 = 30;
//     let height: u32 = 50;

//     let dimensions: (u32, u32) = (width, height);
//     let area_of_rectangle: u32 = area(dimensions);

//     println!("The area of the rectangle is: {}", area_of_rectangle);
// }

// fn area(dimensions: (u32, u32)) -> u32{
//     dimensions.0 * dimensions.1
// }


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// fn main() {

//     let rectangle: Rectangle = Rectangle {
//         width: 30, 
//         height: 60,
//     };

//     let area_of_rectangle: u32 = area(&rectangle);

//     println!("The area of the rectangle is: {}", area_of_rectangle);

//     // printing the rectangle to the console 
//     println!("rectangle is ::: {:?}", rectangle);

//     // using the dbg macro to print the rectangle
//     dbg!(rectangle);
// }


// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }


// we can always use method instead of functions and here we define similar methods peculiar to the struct
// methods are called in the context of a struct and their first parameter is always self

impl Rectangle {

    // this method below give the area of the rectangle 
    // the sweet thing about this is that we use self to represenet the struct 
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn build(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }
}


fn main() {

    // let rectangle: Rectangle = Rectangle {
    //     width: 30, 
    //     height: 60,
    // };

    // let area_of_rectangle: u32 = rectangle.area();

    // println!("The area of the rectangle is: {}", area_of_rectangle);

    // println!("rectangle is ::: {:?}", rectangle);

    let rectangle_1: Rectangle = Rectangle {
        width: 30, 
        height: 60,
    };

    let rectangle_2: Rectangle = Rectangle {
        width: 20, 
        height: 40,
    };

    let rectangle_3: Rectangle = Rectangle {
        width: 50, 
        height: 70,
    };

    let can_rec_1_hold_rec_2: bool = rectangle_1.can_hold(&rectangle_2);
    let can_rect_1_hold_rect_3: bool = rectangle_1.can_hold(&rectangle_3);


    println!("Can rectangle 1 hold rectangle 2? {}", can_rec_1_hold_rec_2);

    let built_rect: Rectangle = Rectangle::build(40, 80);
    println!("Built rectangle is: {:?}", built_rect);

}