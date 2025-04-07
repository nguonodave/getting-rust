// #[derive(Debug)]
// struct User {
//     name: String,
//     age: i32,
//     mail: String,
//     active: bool
// }

// fn main() {
//     let user1 = User {
//         name: String::from("dave"),
//         mail: String::from("dave@g.com"),
//         age: 120,
//         active: true
//     };

//     // at this point user1.name is invalid since it has been moved to user2.name
//     let user2 = User {
//         mail: String::from("dave2@g.com"),
//         ..user1
//     };

//     // the {} tells the print macro to use a formatting called display
//     // primitive types use it by default, but with types like struct we have to specify
//     // {:?} tells rust we want to use an output format called debug, this means
//     // that we will have to include the trait debug in the struct initialization
//     // the # is added to make the output pretty
//     println!("{:#?}", user2)
// }

// // example in structs
// struct Rectangle {
//     width: u32,
//     height: u32
// }

// fn main() {
//     let rect = Rectangle {
//         width: 10,
//         height: 20
//     };

//     let a = area(&rect);

//     println!("{a}")
// }

// fn area(r: &Rectangle) -> u32 {
//     r.height * r.width
// }

// from the above, since the function area is tied to the struct Rectangle,
// we can make it a method and tie it to an instance of Rectangle
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && other.width < self.width
    }
}

// square is an associated function, not a method, since it does not have self
// square is accessed through the namespacing syntax ::
// the Self being returned is an instance of a Rectangle
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 20,
    };

    let rect2 = Rectangle {
        width: 5,
        height: 10,
    };

    let rect3 = Rectangle::square(25);

    println!("{}", rect.area());
    println!("{}", rect.can_hold(&rect2));
    println!("{:#?}", rect3);
}
