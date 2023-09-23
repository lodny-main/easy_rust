// ok() : Result to Option
// ok_or() : Option to Result
// ok_or_else() : Option to Result with closure

fn main() {
    let user_input = vec!["8.9", "Nine point nine five", "8.0", "7.6", "eleventh-twelve"];

    let actual_numbers = user_input
        .into_iter()
        .filter_map(|input| input.parse().ok())
        .collect::<Vec<f32>>();

    println!("{:?}", actual_numbers);
}
