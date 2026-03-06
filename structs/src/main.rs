mod example;

struct User {
    active: bool,
    username: String, // taking ownership of the username in Struct
    email: String,    // taking ownership of the email in Struct
    sign_in_count: u64,
}

// Tuple structs
struct Color(u8, u8, u8);
struct Point(u8, u8, u8);

fn main() {
    let user1 = User {
        // instance of a struct User
        active: true,
        username: String::from("insanely_Tamojit"),
        email: String::from("tamojitdas181007@gmail.com"),
        sign_in_count: 0,
    };

    let user2 = User {
        // instance of a struct User
        active: true,
        username: String::from("Tamoziit"),
        email: String::from("tamojitdas@rediffmail.com"),
        sign_in_count: 0,
    };

    println!("{} --> {}", user1.username, user1.email);
    println!("{} --> {}", user2.username, user2.email);

    let mut user1 = User {
        active: true,
        username: String::from("insanely_Tamojit"),
        email: String::from("tamojitdas181007@gmail.com"),
        sign_in_count: 0,
    };
    user1.username = String::from("Nobility");
    user1.username.push_str(" Highness");

    println!("{} --> {}", user1.username, user1.email);

    // user building
    let user = build_user(String::from("Test"), String::from("test@gmail.com"));
    println!(
        "{}\n{}\n{}\n{}",
        user.username, user.email, user.active, user.sign_in_count
    );

    let user_2 = User {
        email: String::from("helllyeah@gmail.com"),
        ..user // al the remaining fields same as user --> transfering ownership --> user out of scope
    };

    println!(
        "{}\n{}\n{}\n{}",
        user_2.username, user_2.email, user_2.active, user_2.sign_in_count
    );

    let red = Color(255, 0, 0);
    set_bg_color(red);

    let pt = Point(3, 8, 9);
    set_point(pt);

    // Rectangle area
    let rect = example::Rectangle {
        width: 32,
        height: 50,
    };
    let area = example::calculate_area(&rect); // passing instance

    println!(
        "Area of rect: w = {}; h = {} --> {area}",
        rect.width, rect.height
    );

    example::struct_debug_display(&rect);
}

fn build_user(username: String, email: String) -> User {
    User {
        username: username,
        email: email,
        active: true,
        sign_in_count: 0,
    }
}

fn set_bg_color(color: Color) {
    println!(
        "Setting BG-Color: R={}, G={}, B={}",
        color.0, color.1, color.2
    );
}

fn set_point(point: Point) {
    println!("Setting Point: x={}, y={}, z={}", point.0, point.1, point.2);
}
