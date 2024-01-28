#[derive(Debug)]
#[allow(unused)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    age: u8,
}

#[derive(Debug)]
#[allow(unused)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
// struct as tuple
struct Color(i32, i32, i32);

// struct without fields
#[derive(Debug)]
struct EmptyRequest;

#[allow(unused)]
pub fn example1() {
    let color: Color = Color(255, 0, 0);
    let request: EmptyRequest = EmptyRequest;

    // debug print
    dbg!("{:?}", request);
    dbg!("{:?}", color);

    let user1: User = factory_user(
        String::from("email@gmail.com"),
        String::from("jfosajdofajssandev"),
    );

    let user2: User = User {
        email: String::from("email2@gmail.com"),
        username: String::from("jfosajdofajssandev"),
        ..user1
    };

    println!("user 1 {:?}", user1);

    println!("user 2 {:?}", user2);
}

#[allow(unused)]
pub fn example2() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect 1 {:?}", rect1);
    println!("area 1 {:?}", area(&rect1));
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn factory_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 0,
        age: 18,
    }
}
