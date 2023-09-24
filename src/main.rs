use std::rc::Rc;

#[derive(Debug)]
struct City {
    name: String,
    population: i32,
    history: Rc<String>
}

#[derive(Debug)]
struct CityData {
    names: Vec<String>,
    histories: Vec<Rc<String>>
}

fn main() {
    let calgary = City {
        name: String::from("Calgary"),
        population: 133_6000,
        history: Rc::new(String::from("Calgary was founded in blah blah blah"))
    };

    let canada_cities = CityData {
        names: vec![calgary.name],
        histories: vec![Rc::clone(&calgary.history)]
    };

    println!("{canada_cities:?}");
    println!("Calgary's history is: {}", calgary.history);
    println!("Data has {} owners", Rc::strong_count(&calgary.history));
}
