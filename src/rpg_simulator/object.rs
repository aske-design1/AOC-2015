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


pub trait Spell<'a, T: Stats> {
    //fn new(name: &'a str, mana_cost: u32, active: u32, duration: u32) -> Self;

    fn set_active(&mut self);
    fn get_active(&self) -> u32; 
    fn is_active(&self) -> bool;
    fn get_mana(&self) -> u32;
    fn do_spell(&mut self, you: &mut Wizard, opp:  &mut T);
}

pub struct MagicMissile<'a> {
    name: &'a str,
    mana_cost: u32, 
    active: u32,
    duration: u32 
}
impl<'a> MagicMissile<'a> {
    pub fn new() -> Self {
        Self {
            name: "Magic Missile",
            mana_cost: 53,
            active: 0,
            duration: 1
        }
    }
}
impl<'a, T: Stats> Spell<'a, T> for MagicMissile<'a> {
    fn set_active(&mut self) {
        self.active = if self.is_active() {
            self.active - 1
        } else { self.duration }
    }
    fn get_active(&self) -> u32 {0}
    fn is_active(&self) -> bool {true}
    fn get_mana(&self) -> u32 {0}
    fn do_spell(&mut self, you: &mut Wizard, opp: &mut T) {}
}

pub struct Drain<'a> {
    name: &'a str,
    mana_cost: u32, 
    active: u32,
    duration: u32 
}

impl<'a> Drain<'a> {
    pub fn new() -> Self {
        Self {
            name: "Magic Missile",
            mana_cost: 53,
            active: 0,
            duration: 1
        }
    }
}

impl<'a, T: Stats> Spell<'a, T> for Drain<'a> {
    fn set_active(&mut self) {
        self.active = if self.active != 0 {
            self.active - 1
        } else { self.duration }
    }
    fn get_active(&self) -> u32 { self.active }
    fn is_active(&self) -> bool { self.active != 0 }
    fn get_mana(&self) -> u32 { self.mana_cost }
    fn do_spell(&mut self, you: &mut Wizard, opp: &mut T) {


    }
}

pub struct Shield<'a> {
    name: &'a str,
    mana_cost: u32, 
    active: u32,
    duration: u32 
}
impl<'a> Shield<'a> {
    pub fn new() -> Self {
        Self {
            name: "Magic Missile",
            mana_cost: 53,
            active: 0,
            duration: 1
        }
    }
}
impl<'a, T: Stats> Spell<'a, T> for Shield<'a> {
    fn set_active(&mut self) {}
    fn get_active(&self) -> u32 {0}
    fn is_active(&self) -> bool {true}
    fn get_mana(&self) -> u32 {0}
    fn do_spell(&mut self, you: &mut Wizard, opp: &mut impl Stats) {}
}

pub struct Poison<'a> {
    name: &'a str,
    mana_cost: u32, 
    active: u32,
    duration: u32 
}
impl<'a> Poison<'a> {
    pub fn new() -> Self {
        Self {
            name: "Magic Missile",
            mana_cost: 53,
            active: 0,
            duration: 1
        }
    }
}
impl<'a, T: Stats> Spell<'a, T> for Poison<'a> {
    fn set_active(&mut self, value: u32) {}
    fn get_active(&self) -> u32 {0}
    fn is_active(&self) -> bool {true}
    fn get_mana(&self) -> u32 {0}
    fn do_spell(&mut self, you: &mut Wizard, opp: &mut impl Stats) {}
}

pub struct Recharge<'a> {
    name: &'a str,
    mana_cost: u32, 
    active: u32,
    duration: u32 
}
impl<'a> Recharge<'a> {
    pub fn new() -> Self {
        Self {
            name: "Magic Missile",
            mana_cost: 53,
            active: 0,
            duration: 1
        }
    }
}
impl<'a, T: Stats> Spell<'a, T> for Recharge<'a> {
    fn set_active(&mut self) { 
        self.active = if self.is_active() {
            self.active -1
        } else {
            self.duration
        }
    }
    fn get_active(&self) -> u32 {0}
    fn is_active(&self) -> bool {true}
    fn get_mana(&self) -> u32 {0}
    fn do_spell(&mut self, you: &mut Wizard, opp: &mut T) {}
}