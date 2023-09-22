struct Item {
    num: i32
}

impl Item {
    fn compare(&self, other: i32) {
        println!("Are they equal? {}", self.num == other);
    }
}

fn main() {
    let num = 10;
    let ref_num = &num;

    println!("Are they the same? {}", num == *ref_num);

    // because dot operator ->> auto deref
    let item = Item {
        num: 10
    };
    let ref_item = &item;
    let other_ref_item = &ref_item;
    item.compare(10);
    ref_item.compare(10);
    other_ref_item.compare(10);
}
