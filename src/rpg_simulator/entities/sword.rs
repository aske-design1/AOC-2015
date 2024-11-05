use super::super::object::{
    Item
};

#[derive(Clone)]
pub struct SwordUser {
    hit_points: u32,
    dmg: u32, 
    armor: u32,
}


#[derive(Clone)]
pub struct Boss {
    hit_points: u32,
    dmg: u32, 
    armor: u32,
}

impl SwordUser {
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
    pub fn new_from_input(input: &str) -> Self {
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

pub trait Stats {
    fn get_hit_points(&self) -> u32;
    fn get_dmg(&self) -> u32;
    fn get_armor(&self) -> u32;
    fn take_damage(&mut self, dmg: u32);
}

impl Stats for SwordUser { 
    fn get_hit_points(&self) -> u32 { self.hit_points }
    fn get_dmg(&self) -> u32 { self.dmg }
    fn get_armor(&self) -> u32 { self.armor }
    fn take_damage(&mut self, dmg: u32) { self.hit_points = 0.max(self.hit_points as i32 - dmg as i32) as u32; }
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

impl Battle for SwordUser {}



#[cfg(test)]
mod tests {
    use super::*;
    #[test] 
    fn test_battle_logic() {
        let you = SwordUser::new(8, 5, 5);
        let opp = SwordUser::new(12, 7, 2);

        assert!(you.battle(&opp))
    }

}