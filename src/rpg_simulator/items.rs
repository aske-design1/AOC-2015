use super::entity::*;

pub struct Item<'a> {
    name: &'a str,
    cost: u32,
    dmg: u32, 
    armor: u32,
}

impl<'a> Item<'a> {
    pub fn from(name: &'a str, cost: u32, dmg: u32, armor: u32) -> Self {
        Self { name, cost, dmg, armor }
    }
    pub fn get_inventory() -> Vec<Vec<Self>> {
        vec![
            vec![
                Item::from("Dagger", 8, 4, 0), 
                Item::from("Shortsword", 10, 5, 0), 
                Item::from("Warhammer", 25, 6, 0),
                Item::from("Longsword", 40, 7, 0),
                Item::from("Greataxe", 74, 8, 0)
            ],
            vec![
                Item::from("Nothing", 0, 0, 0),
                Item::from("Leather", 13, 0, 1),
                Item::from("Chainmail", 31, 0, 2),
                Item::from("Splintmail", 53, 0, 3),
                Item::from("Bandedmail", 75, 0, 4),
                Item::from("Platemail", 102, 0, 5),
            ],
            vec![
                Item::from("Nothing", 0, 0, 0),
                Item::from("Damage +1", 25, 1, 0),
                Item::from("Damage +2", 50, 2, 0),
                Item::from("Damage +3", 100, 3, 0),
                Item::from("Defense +1", 20, 0, 1),
                Item::from("Defense +2", 40, 0, 2),
                Item::from("Defense +3", 80, 0, 3),
            ]
        ]
    }

    pub fn get_name(&self) -> &'a str { self.name }
    pub fn get_cost(&self) -> u32 { self.cost }
    pub fn get_dmg(&self) -> u32 { self.dmg }
    pub fn get_armor(&self) -> u32 { self.armor }

}


#[derive(Clone, PartialEq, Debug)]
pub enum SpellType {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge
}

#[derive(Clone)]
pub struct Spell(pub SpellType, pub u32); 

impl Spell {
    pub fn new(spell_type: &SpellType) -> Option<Self> {
        let spell_type = spell_type.clone();
        match spell_type {
            SpellType::Shield | SpellType::Poison => Some(Self(spell_type, 6)),
            SpellType::Recharge => Some(Self(spell_type, 5)),
            _ => None
        }
    }

    pub fn passives(&mut self, you: &mut Entity, boss: &mut Entity) {
        //if !self.is_active() { return }
        self.1 -= 1;

        match self.0 {
            SpellType::Shield => {
                if self.1 == 0 { you.armor -= 7; }
            },
            SpellType::Poison => boss.life_points = boss.life_points.saturating_sub(3),
            SpellType::Recharge => you.mana += 101,
            _ => unreachable!()
        }
    }
    pub fn is_active(&self) -> bool { self.1 > 0 }
}
