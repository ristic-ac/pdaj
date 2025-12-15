mod user_model {
    pub struct User {
        username: String,
        age: u8,
        is_active: bool,
    }

    impl User {
        pub fn new(username: String, age: u8) -> Self {
            User {
                username,
                age,
                is_active: true,
            }
        }

        pub fn username(&self) -> &str {
            &self.username
        }

        pub fn age(&self) -> u8 {
            self.age
        }

        pub fn display_info(&self) {
            println!(
                "Username: {}, Age: {}, Active: {}",
                self.username, self.age, self.is_active
            );
        }
    }

}
fn main() {
    let user = user_model::User::new(String::from("Alice"), 30);
    // let user = user_model::User {
    //     username: String::from("Bob"),
    //     age: 25,
    //     is_active: false,
    // };
    println!("User: {}, Age: {}", user.username(), user.age());
    println!("User info:");
    user.display_info();
    // println!("User: {}, Age: {}", user.username);
}