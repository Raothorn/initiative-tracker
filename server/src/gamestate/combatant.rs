use serde::Serialize;
use rand::Rng;


#[derive(Serialize, Clone)]
pub struct Combatant {
    pub id: u32,
    name: String,
    user: String,
    #[serde(rename = "initRoll")]
    init_roll: u32,
    #[serde(rename = "initMod")]
    init_mod: i32,
}

impl Combatant {
    pub fn new(id: u32, name: &str) -> Self {
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(0..20) + 1;

        Combatant {
            id,
            name: name.to_string(),
            user: "".to_string(),
            init_roll: roll,
            init_mod: 0,
        }    
    }

    pub fn initiative(&self) -> i32 {
        self.init_mod + (self.init_roll as i32)
    }
}
