// todo!() macro
// alias = different name

use std::iter::{Skip, Take};
use std::vec::IntoIter;

type SkipAndTake = Take<Skip<IntoIter<char>>>;
struct  MyOtherString(String);

fn skip_five_take_five(input: Vec<char>) -> SkipAndTake {
    input
        .into_iter()
        .skip(5)
        .take(5)
}

fn main() {

}

