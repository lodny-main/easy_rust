struct Adventurer<'a> {
    name: &'a str,
    hit_points: u32,
}

// implicit == not said
// elided == not shown
// impl Adventurer {                // implicit elided lifetime not allowed here
// impl<'a> Adventurer<'a> {        // older version
impl Adventurer<'_> {
    fn take_damage(&mut self) {
        self.hit_points -= 20;
        println!("{} has {} hit points left!", self.name, self.hit_points);
    }
}

fn main() {

}
