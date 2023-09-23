// dbg!()
// inspect()

fn main() {
    let new_vec = [8, 9, 10];

    let double_vec = new_vec
        .iter()
        .inspect(|&first_item| {
            dbg!(first_item);
        })
        .map(|x| x * 2)
        .inspect(|&first_item| {
            dbg!(first_item);
        })
        .filter(|&num| num > 17)
        .collect::<Vec<_>>();

    dbg!(double_vec);
}
