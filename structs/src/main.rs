#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }


    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}


fn build_user(email: String, username: String) -> User {
    return User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    };
}


fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("user1 is {:#?}", user1);
    let user2 = build_user(String::from("foo@bar"), String::from("foo"));


    // user1 is gone after this
    let user3 = User {
        email: String::from("foo@example.com"),
        ..user1
    };

    println!("user2 is {:#?}", user2);
    println!("user3 is {:#?}", user3);


    let tup = (1.0, 2.0, 3.0);
    println!("tup first entry {}", tup.0);


    struct Color(i32, i32, i32);

    let black = Color(0, 0, 0);
    println!("black first entry {}", black.0);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let square = Rectangle::square(3);
    println!("square {:#?}", square);
}
