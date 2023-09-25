use std::borrow::Cow;

#[derive(Debug)]
struct User<'a> {
    name: Cow<'a, str>,
}

impl<'a> User<'a> {
    fn is_borrowed(&self) {
        match &self.name {
            Cow::Borrowed(name) => println!("It's borrowed: {name}"),
            Cow::Owned(name) => println!("It's owned: {name}")
        }
    }
}


fn main() {
    let mut user_1 = User {
        name: "User 1".into(),
    };

    let user_2 = User {
        name: "User 2".to_string().into(),
    };

    user_1.is_borrowed();
    user_1.name.to_mut().push_str("~~~");
    user_1.is_borrowed();
}

