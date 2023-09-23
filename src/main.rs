// Debug again

use std::fmt::{Debug, Formatter};
use crate::client::InternetClient;

// External code
mod client {
    pub struct InternetClient {
        pub(crate) client_id: u32,
    }
}
// use client::InternetClient;

// #[derive(Debug)]     // can't using because external code ->> impl Debug
struct Customer<'a> {
    money: u32,
    name: &'a str,
    client: &'a InternetClient,
}

impl<'a> Debug for Customer<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Customer")
            .field("money", &self.money)
            .field("name", &self.name)
            .field("client", &"Client")
            .finish()
    }
}

fn main() {
    let client = InternetClient {
        client_id: 0
    };

    let customer = Customer {
        money: 6724,
        name: "Coco",
        client: &client,
    };

    println!("{customer:?}");
}
