use crate::rpg_simulator::object::SpellType;

use super::object::{
    Item, Spell,
};

#[derive(Clone)]
pub struct SwordPlayer {
    hit_points: u32,
    dmg: u32, 
    armor: u32,
}

#[derive(Clone)]
pub struct Wizard {
    hit_points: u32, 
    mana: u32,
    spells: Vec<Spell>
}

#[derive(Clone)]
pub struct Boss {
    hit_points: u32,
    dmg: u32, 
    armor: u32,
}

impl SwordPlayer {
    pub fn new(hit_points: u32, dmg:u32, armor:u32) -> Self {
        Self { 
            hit_points,
            dmg,
            armor,
        }
    }
    pub fn add_stats(&mut self, item: &Item) {
        self.dmg += item.get_dmg();
        self.armor += item.get_armor();
    }
}

impl Boss {
    pub fn new(input: &str) -> Self {
        let splitter = if input.contains("\r\n") { "\r\n" } else { "\n" };
        let split_input: Vec<&str> = input.split(splitter).collect();
        let (mut hit_points, mut dmg, mut armor) = (0,0,0); 

        for el in split_input.iter() {
            let (field, value) = el.split_once(": ").unwrap();
            let value = value.parse().unwrap();
            match field {
                "Hit Points" => hit_points = value,
                "Damage" => dmg = value,
                "Armor" => armor = value,
                _ => panic!("Field not recognized")
            }
        }

        Self { hit_points, dmg, armor }
    }
}

impl Wizard {
    pub fn new(hit_points: u32, mana: u32) -> Self {
        Self {
            hit_points, 
            mana,
            spells: Spell::create_basic_spells()
        }
    }

    pub fn win_with_least_mana(&mut self, other: &Boss) -> u32 {
        let mut least_mana = u32::MAX; 

        for spell in self.spells.iter() {
            let mut you = self.clone();
            let mut opp = other.clone();
            spell.activate();

            if let Some(mana) = Self::fight_recursively(you, opp) {
                least_mana = least_mana.min(mana);
            }
        }
        least_mana
    }

    fn check_status_effects(&self) -> bool {
        for spell in self.spells.iter() {
            if spell.is_active() { return true } 
        }
        false
    }

    fn decrease_statuses(&mut self) {
        for spell in self.spells.iter_mut() {
            spell.decrease_status();
        }
    }

    fn fight_recursively(mut wiz: Wizard, mut opp: Boss) -> Option<u32> {
    if wiz.hit_points == 0 {
        return None
    } else if opp.hit_points == 0 {
        return Some(wiz.mana)
    }
    let least: Option<u32> = None;

    let spells = wiz.spells.clone();

    for spell in spells.into_iter() {
        spell.cast(&mut wiz, &mut opp); 
    }
    wiz.decrease_statuses();

    if wiz.check_status_effects() {
        if let Some(val) = Self::fight_recursively(wiz.clone(), opp.clone()) {
            least = val.min(least.map(f))
        }
    }
    
    for spell_idx in 0..wiz.spells.len() {
        let mut new_wiz = wiz.clone(); 
        new_wiz.spells[spell_idx].activate();

        Self::fight_recursively(new_wiz, opp.clone());
    }
    

    

    None
}


}


pub trait Stats {
    fn get_hit_points(&self) -> u32;
    fn get_dmg(&self) -> u32;
    fn get_armor(&self) -> u32;
}

impl Stats for SwordPlayer {
    fn get_hit_points(&self) -> u32 { self.hit_points }
    fn get_dmg(&self) -> u32 { self.dmg }
    fn get_armor(&self) -> u32 { self.armor }
}

impl Stats for Boss {
    fn get_hit_points(&self) -> u32 { self.hit_points }
    fn get_dmg(&self) -> u32 { self.dmg }
    fn get_armor(&self) -> u32 { self.armor }
}

pub trait Battle: Stats {
    fn battle(&self, other: &impl Stats) -> bool {
        //true if you win 
        let your_dmg = 1.max(self.get_dmg() as i32 - other.get_armor() as i32) as u32;
        let opp_dmg = 1.max(other.get_dmg() as i32 - self.get_armor() as i32) as u32;

        let your_cycles = self.get_hit_points() / opp_dmg;
        let opp_cycle = other.get_hit_points() / your_dmg;

        your_cycles >= opp_cycle
    }
}

impl Battle for SwordPlayer {}
impl Battle for Boss {}


#[cfg(test)]
mod tests {
    use super::*;
    #[test] 
    fn test_battle_logic() {
        let you = SwordPlayer::new(8, 5, 5);
        let opp = SwordPlayer::new(12, 7, 2);

        assert!(you.battle(&opp))
    }
}