#![allow(dead_code, unused_variables)]

pub mod front_of_house {
    // to make it available at root
    // related defn.
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {
            crate::front_of_house::hosting::seat_at_table(); // absolute path
            super::hosting::seat_at_table(); // relative path
        }

        fn take_payment() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }
}

fn eat_at_restuarant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    let breakfast = back_of_house::Breakfast::summer("Wheat"); // seasonal_fruit isn't public -> hence it cannot be constructed without its implementation summer()
}
