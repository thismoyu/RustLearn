// Using Structs to Structure Related Data
//
//

struct User {
    active : bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active : true,
        username : String::from("somename123"),
        email : String::from("someemail@123.com"),
        sign_in_count : 1,
    };
}
