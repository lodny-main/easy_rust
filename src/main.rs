use std::collections::HashMap;

struct City {
    name: String,
    population: HashMap<u32, u32>   // year + population
}

fn main() {
    let mut tallinn = City {
        name: "Tallinn".to_string(),
        population: HashMap::new(),
    };

    tallinn.population.insert(1372, 3250);
    tallinn.population.insert(1851, 2_4000);
    tallinn.population.insert(2020, 43_7619);

    for (year, population) in tallinn.population {
        println!("In the year {} the population was {}", year, population);
    }
}
