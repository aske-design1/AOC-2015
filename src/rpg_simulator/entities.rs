use super::object::{
    Item,
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Spell,
    Recharge
};

#[derive(Clone)]
pub struct SwordPlayer {
    hit_points: u32,
    dmg: u32, 
    armor: u32,
}

pub struct Wizard<'a> {
    hit_points: u32, 
    mana: u32,
    spells: Vec<Spell<'a>>
}

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

impl<'a> Wizard<'a> {
    fn new(hit_points: u32, mana: u32, spells: Vec<Box<&dyn Spell>>) -> Self {
        Self {
            hit_points, 
            mana,
            spells
        }
    }

    fn battle(&self, other: &impl Stats) -> bool {
        let mut opp_health = other.get_hit_points(); 
        


        true
    }

    pub fn create_basic_spells() -> Vec<Box<dyn Spell<'static, Boss>>> {
        vec![
            Box::new(MagicMissile::new()),
            Box::new(Drain::new()),
            Box::new(Shield::new()),
            Box::new(Poison::new()),
            Box::new(Recharge::new()),
        ]
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