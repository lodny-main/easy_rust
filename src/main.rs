// Default and the builder pattern

#[derive(Debug)]
enum LifeState {
    Alive,
    Dead,
    NeverAlive,
    Uncertain,
}

#[derive(Debug)]
struct Character {
    name: String,
    age: u8,
    height: u32,
    weight: u32,
    lifestate: LifeState,
}

impl Character {
    fn new(name: String, age: u8, height: u32, weight: u32, alive: bool) -> Self {
        Self {
            name,
            age,
            height,
            weight,
            lifestate: if alive { LifeState::Alive } else { LifeState::Dead },
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: "Billy".to_string(),
            age: 15,
            height: 170,
            weight: 70,
            lifestate: LifeState::Alive,
        }
    }
}

fn main() {
    let npc_1 = Character::new("Billy".to_string(), 15, 170, 70, true);
    println!("{npc_1:?}");

    let npc_2 = Character::default();
    println!("{npc_2:?}");
}
