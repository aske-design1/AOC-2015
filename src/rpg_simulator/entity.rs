use super::items::*;

#[derive(Clone)]
pub struct Entity {
    pub life_points: u32,
    pub dmg: u32, 
    pub armor: u32,
    pub mana: u32
}


/**** Day 21 *****/
impl Entity {
    pub fn new(life_points: u32, dmg:u32, armor:u32) -> Self {
        Self { 
            life_points,
            dmg,
            armor,
            mana:0
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

        Self { life_points: hit_points, dmg, armor, mana:0 }
    }
}


pub trait Stats {
    fn get_hit_points(&self) -> u32;
    fn get_dmg(&self) -> u32;
    fn get_armor(&self) -> u32;
    fn take_damage(&mut self, dmg: u32);
}

impl Stats for Entity { 
    fn get_hit_points(&self) -> u32 { self.life_points }
    fn get_dmg(&self) -> u32 { self.dmg }
    fn get_armor(&self) -> u32 { self.armor }
    fn take_damage(&mut self, dmg: u32) { self.life_points = 0.max(self.life_points as i32 - dmg as i32) as u32; }
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

impl Battle for Entity {}


/*****  Day 22 ******/
impl Entity {
    pub fn new_with_mana(life_points:u32, mana:u32) -> Self {
        Self {
            life_points,
            armor: 0,
            dmg:0,
            mana
        }
    }

    pub fn is_alive(&self) -> bool {
        self.life_points != 0
    } 

    pub fn attack(&mut self, other: &mut Self) {
        let attack_dmg = match self.dmg.saturating_sub(other.armor) {
            0 => 1,
            dmg => dmg 
        };
        other.life_points = other.life_points.saturating_sub(attack_dmg)
    }
    

    pub fn cast(&mut self, spell_type: &SpellType, boss: &mut Self) -> Option<(Option<Spell>, u32)> {
        let cost = match spell_type {
            SpellType::MagicMissile => {
                boss.life_points = boss.life_points.saturating_sub(4);
                53
            },
            SpellType::Drain => {
                self.life_points += 2;
                boss.life_points = boss.life_points.saturating_sub(2);
                73
            },
            SpellType::Shield => {
                self.armor += 7;
                113
            },
            SpellType::Poison => 173,
            SpellType::Recharge => 229,
        };

        if cost > self.mana { return None }

        self.mana -= cost;
        Some((Spell::new(spell_type), cost))
    }

}




#[cfg(test)]
mod tests {
    use super::*;
    #[test] 
    fn test_battle_logic() {
        let you = Entity::new(8, 5, 5);
        let opp = Entity::new(12, 7, 2);

        assert!(you.battle(&opp))
    }

}