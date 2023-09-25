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

    fn with_age(mut self, age: u8) -> Self {
        self.age = age;
        self
    }

    fn with_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    fn with_weight(mut self, weight: u32) -> Self {
        self.weight = weight;
        self
    }

    fn build() -> Self {
        Self {
            name: "".to_string(),
            age: 0,
            height: 0,
            weight: 0,
            lifestate: LifeState::Alive,
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

    let npc_3 = Character::default()
        .with_age(20)
        .with_height(194)
        .with_weight(82);
    println!("{npc_3:?}");
}
