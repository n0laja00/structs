// Introducing a global data structure
//Struct
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//Tuple structure: useful if you want to make a tuple different form other tuples and when naming fields would be reduntandant.
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

// unit-like struct
#[derive(Debug)]
struct AlwaysEqual; //Unit-like types can be userful when you need a trait bt don't have data to keep inside the type.

fn main() {
    let subject = AlwaysEqual;
    println!("{:?}", subject);

    let black: Color = Color(0, 0, 0);
    let origin: Point = Point(0, 0, 0);
    println!("{:?}", black);
    println!("{:?}", origin);

    // To make mutable structures, we must make the whole thing mutable. We can't assign specific mutable fields.
    let mut user1: User = User {
        active: true,
        username: String::from("haroldtheuser"),
        email: String::from("Harold@gmail.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("Haroldsnewemail.gmail.com");
    println!("{:?}", user1);

    // Struct update syntax
    let user2: User = User {
        active: user1.active,
        username: user1.username,
        email: String::from("User2@hehe.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("{:?}", user2);

    // Struct update syntax: achieve the same as before but with ..user1.
    let user3: User = User {
        email: String::from("User3@hehe.com"),
        ..user2
    };
    println!("{:?}", user3);

    // NOTE: Remember that after moving complex data (String) from user to user, it implements Move rather than copy. So, user1 and user2 have been rendered invalid.
    // Only boolean and u64 remain invalid in user 2 and 1

    let user4: User = build_user(
        String::from("Haroldbro@gmail.com"),
        String::from("bro harold"),
    );
    println!("{:?}", user4);

    calculate_pixels();
}

fn build_user(email: String, username: String) -> User {
    //Using field init shorthand syntax. Username and email do not need "x: y" but only "y"
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

//Let's calculate rectangles
fn calculate_pixels() {
    println!("Let's calculate rectangles without Structs!");
    let width1: u32 = 30;
    let height1: u32 = 70;

    println!(
        "The area of a rectagle is {} square pixels.",
        area(width1, height1)
    );

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
    refac_calculate_pixels();
}
// This program can easily calculate the area, but it's readability isn't the best. it isn't allthat obvious how width and height are related; they are just some width and height.
// Refactoring this code is possible by using a tuple struct or a basic struct. Let's add more meaning by using a basic struct.

fn refac_calculate_pixels() {
    println!("Let's calculate rectangles with Structs and with more functionality and debugs!!");

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let scale: u32 = 2;

    let rect1: Rectangle = Rectangle {
        width: dbg!(30 * scale),
        height: 70,
    };

    dbg!(&rect1); //DEbug!
    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of a rectagle is {} square pixels.",
        area(&rect1) // Send ref of rect1
    );

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    define_methods();
}

// Methods are similar to functions, but inlike functions, methods are defined within the Structure (functions nested inside objects). First parameter is always self.
// all functions inside the impl block are called associated functions.
fn define_methods() {
    println!("Let's calculate rectangles with Structs and define methods!!");

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // Method (impl = implementation)
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn has_width(&self) -> bool {
            self.width > 0
        }
    }

    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 70,
    };
    println!("rect1 is {:#?}", rect1);

    // Rust knows when we use (), that we mean methods. The lack of () means we mean the field.
    // Sometimes, we have methods  with the same name as the field to return the value of that field and nothing else. These are known as "Getters"
    println!(
        "Rect1 has width; this is {} and it's {}; and The area of a rectagle is {} square pixels.",
        rect1.has_width(),
        rect1.width,
        rect1.area() //Use method
    );

    advanced_methods();
}

fn advanced_methods() {
    println!("Let's calculate rectangles with Structs and define advanced methods!!");

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // Method (impl = implementation)
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        // Works just like normal funcitons. We take in a ref and return a boolean.
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
        // We can also make constructors inside impl blocks!
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 70,
    };

    let rect2: Rectangle = Rectangle {
        width: 20,
        height: 60,
    };

    let rect3: Rectangle = Rectangle {
        width: 40,
        height: 80,
    };

    println!("rect1 is {:#?}", rect1);
    println!("rect1 is {:#?}", rect2);
    println!("rect1 is {:#?}", rect3);

    //Pass refs because we don't want to give ownership.
    println!("can rect1 hold rect 2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect 3? {}", rect1.can_hold(&rect3));

    let sqr = Rectangle::square(3);

    println!("We made a square with a constructor! {:#?}", sqr);

    println!(
        "TODAY WE LEARNED: how to calculate area with methods. {}",
        rect1.area()
    );
    println!("TODAY WE LEARNED: to use dbg!() and  #[derive(Debug)]");
    println!(
        "TODAY WE LEARNED: that we can use methods like functions! Does rect 1 hold rect 2? {}",
        rect1.can_hold(&rect2)
    );
    println!(
        "TODAY WE LEARNED: that we can make constructors! {:#?}",
        sqr
    );
}
