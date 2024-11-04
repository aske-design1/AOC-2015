use super::entities::{Stats, Wizard};


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


#[derive(Clone)]
pub struct Spell {
    spell_type: SpellType,
    mana_cost: u32, 
    status_active: u32,
    status_max_time: u32,
}

#[derive(Clone)]
pub enum SpellType {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge
}

impl Spell {
    pub fn new(spell_type: SpellType, mana_cost: u32, active: u32, duration: u32) -> Self {
        Self { spell_type, mana_cost, status_active: active, status_max_time: duration }
    }
    pub fn is_active(&self) -> bool { self.status_active != 0 }
    
    pub fn cast(&self, wizard: &mut Wizard, enemy: &mut impl Stats) {
        match self.spell_type {
            SpellType::MagicMissile => self.cast_magic_missile(wizard, enemy),
            SpellType::Drain => self.cast_drain(wizard, enemy),
            SpellType::Shield => self.cast_shield(wizard, enemy),
            SpellType::Poison => self.cast_poison(wizard, enemy),
            SpellType::Recharge => self.cast_recharge(wizard, enemy),
        }
    }

    pub fn activate(&mut self) {}

    pub fn decrease_status(&mut self) {
        if self.is_active() { self.status_active -= 1 }
    }

    fn cast_magic_missile(&self, wizard: &mut Wizard, enemy: &mut impl Stats) {

    }

    fn cast_drain(&self, wizard: &mut Wizard, enemy: &mut impl Stats) {

    }

    fn cast_shield(&self, wizard: &mut Wizard, enemy: &mut impl Stats) { 
    }

    fn cast_poison(&self, wizard: &mut Wizard, enemy: &mut impl Stats) {

    }

    fn cast_recharge(&self, wizard: &mut Wizard, enemy: &mut impl Stats) {

    }

    pub fn create_basic_spells() -> Vec<Spell> {
        vec![
            Spell::new(SpellType::MagicMissile, 53, 0, 1),          // Does 4 damage instantly
            Spell::new(SpellType::Drain, 73, 0, 1),                 // Deals 2 damage, heals 2 HP
            Spell::new(SpellType::Shield, 113, 1, 6),              // Increases armor for 6 turns
            Spell::new(SpellType::Poison, 173, 1, 6),              // Deals 3 damage each turn for 6 turns
            Spell::new(SpellType::Recharge, 229, 1, 5),          // Gives 101 mana each turn for 5 turns
        ]
    }
}


/*
pub struct Drain<'a> {
    name: &'a str,
    mana_cost: u32, 
    active: u32,
    duration: u32 
}

pub struct Shield<'a> {
    name: &'a str,
    mana_cost: u32, 
    active: u32,
    duration: u32 
}


pub struct Poison<'a> {
    name: &'a str,
    mana_cost: u32, 
    active: u32,
    duration: u32 
}

pub struct Recharge<'a> {
    name: &'a str,
    mana_cost: u32, 
    active: u32,
    duration: u32 
}
*/