#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn increment_sign_in_count(&mut self) {
        self.sign_in_count += 1;
    }
    fn change_username(&mut self, new_username: &str) {
        self.username = String::from(new_username);
    }
    fn change_email(&mut self, new_email: &str) {
        self.email = String::from(new_email);
    }
}

fn change_username(user: &mut User, new_username: &str) {
    user.username = String::from(new_username);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structs() {
        let mut user_1 = User {
            username: String::from("Rust"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
            active: true,
        };

        change_username(&mut user_1, "new_username");

        let mut user_2: User = User {
            username: String::from("new_username"),
            email: String::from("someone@example.com"),
            sign_in_count: 0,
            active: false,
        };

        user_2.increment_sign_in_count();

        user_2.change_email("new_email");

        dbg!(user_1);
    }
}
