
// re-exporting (重新导出）
// 此导出之后，相当于减少了 front_of_house 层级 
pub use crate::front_of_house::hosting;

pub mod front_of_house {
    pub mod hosting {

        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {
            // ../hosting/add_to_waitlist
            super::hosting::add_to_waitlist();
        }
        fn serve_order() {}

        fn take_payment() {}
    }
}

pub mod back_of_house {
    use core::str;

    // 特殊：对于枚举类来说，如果枚举是public的，那么所有的变量都是public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // absolute path: /front_of_house/hosting/add_to_waitlist
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path: front_of_house/hosting/add_to_waitlist
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // error: seasonal_fruit is not pub
    // meal.seasonal_fruit = String::from("test");

    // back_of_house::Breakfast {
    //     toast :String::from("test"),
    //     seasonal_fruit : String::from("test2"),// error : seasonal_fruit is private, 所以只能通过函数创建实例
    // };

    use back_of_house::Appetizer;
    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;
}
