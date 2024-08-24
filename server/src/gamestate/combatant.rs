use serde::Serialize;

use super::Player;


#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Combatant {
    pub id: String,
    name: String,
    combatant_type: CombatantType,
    init_roll: u32,
    init_mod: i32,
}

#[derive(Serialize, Clone)]
pub enum CombatantType {
    PlayerCombatant,
    MonsterCombatant
}

impl Combatant {
    pub fn from_player(player: &Player) -> Self {
        Combatant {
            id: player.guid.clone(),
            name: player.name.clone(),
            combatant_type: CombatantType::PlayerCombatant,
            init_roll: 10,
            init_mod: player.initiative_bonus
        }
    }

    pub fn initiative(&self) -> i32 {
        self.init_mod + (self.init_roll as i32)
    }
}
