// match_indices()      : indices = indexes
// peekable()
// peek()
// next()

fn main() {
    let rules = "Rule number 1: No fighting.
Rule number 2: Go to bed at 8 pm.
Rule number 3: Wake up at 6 am.";

    let rule_locations = rules
        .match_indices("Rule")
        .collect::<Vec<(_,_)>>();
    println!("Rule locations: {rule_locations:?}");
}
