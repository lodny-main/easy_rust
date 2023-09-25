use std::borrow::Cow;

struct User {
    name: String,
}

#[derive(Debug)]
struct User2<'a> {
    name: Cow<'a, str>,
}

impl User {
    fn new(input: &str) -> Self {
        Self {
            name: input.to_string(),
        }
    }
}

fn main() {
    let name_1 = "User 1";
    let name_2 = "User 2".to_string();

    let my_user_1 = User::new(name_1);
    let my_user_2 = User::new(&name_2);

    let user_1 = User2 {
        name: name_1.into(),
    };

    let user_2 = User2 {
        name: name_2.into(),
    };

    println!("User 1 is {user_1:?} and User 2 is {user_2:?}");
}

