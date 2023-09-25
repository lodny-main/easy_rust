#![allow(dead_code)]
#![warn(
    clippy::pedantic,
    // clippy::nursery,
    // clippy::cargo,
)]

const OKAY_CHARACTERS: &str = "1324567890+- ";

fn math(input: &str) -> i32 {
    if !input.chars().all(|character| OKAY_CHARACTERS.contains(character)) {
        panic!("Please only input numbers, +-, or spaces");
    }

    // let input = input.replace(" ", "");
    let input = input
        .replace(" ", "")
        .trim_end_matches(|x| "+-".contains(x))
        .chars()
        // .filter(|x| *x != ' ')
        .collect::<String>();
    println!("{input:?}");
    9
}

// replace : remove blank
// remove after last number


// Test-Driven development
#[cfg(test)]
mod tests {
    use super::*;   // can use outside of mod tests

    #[test]
    fn one_plus_one_is_two() {
        assert_eq!(math("1 + 1 +-++-   "), 2);
    }

    #[test]
    fn one_minus_minus_one_is_two() {
        assert_eq!(math("1 - -1"), 2);
    }

    #[test]
    #[should_panic]
    fn panics_when_characters_not_right() {
        math("7 + please add seven");
    }

}

fn main() {

}
