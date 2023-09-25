// Cow = Clone on Write

// + Add
// - Sub
// += AddAssign

use std::ops::Add;


#[derive(Debug)]
struct Country {
    name: String,
    population: u32,
    gdp: u32,
}

impl Add for Country {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            name: format!("{} and {}", self.name, other.name),
            population: self.population + other.population,
            gdp: self.gdp + other.gdp,
        }
    }
}

impl Country {
    fn new(name: &str, population: u32, gdp: u32) -> Self {
        Self {
            name: String::from(name),
            population,
            gdp,
        }
    }
}

fn main() {
    let nauru = Country::new("Nauru", 1_0670, 1_6000_0000);
    let vanuatu = Country::new("Vanuatu", 30_7815, 8_2000_0000);
    let micronesia = Country::new("Micronesia", 10_4468, 3_6700_0000);

    println!("Nauru + Vanuatu + Micronesia = {:?}", nauru + vanuatu + micronesia);
}
