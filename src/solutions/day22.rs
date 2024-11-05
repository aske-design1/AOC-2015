use std::collections::VecDeque;

use super::*;

use crate::rpg_simulator::entities::sword::Entity;

#[allow(dead_code)]
pub struct Day22 {
    boss: Entity
}


#[derive(Clone, PartialEq, Debug)]
pub enum SpellType {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge
}

    
pub struct Spell(SpellType, u32); 

const SPELLS: [SpellType; 5] = [
    SpellType::MagicMissile,
    SpellType::Drain,
    SpellType::Shield,
    SpellType::Poison,
    SpellType::Recharge
];




impl Day22 {
    pub fn new(input: String) -> Self {
        Self { boss: Entity::new_from_input(&input) }
    }


    fn battle(you: Entity, boss: Entity) -> u32 {
        let mut least_mana = u32::MAX; 

        //Queue and insert
        let mut queue = VecDeque::new();
        for spell_idx in 0..SPELLS.len() {
            queue.push_front(
                //Insert data
                (
                    you.clone(),
                    boss.clone(),
                    //500i32, //Mana amount
                    spell_idx,
                    Vec::<Spell>::new(),
                    0
                )
            );
        }
        


        //Main loop that gathers a prev recorded wizard
        while let Some(state) = queue.pop_front() {
            let (mut you, mut enemy, cur_spell, mutspells, mut expended_mana) = state;

            if !you.is_alive() {
                continue;
            }




        }

        least_mana
    }
}

impl Solution for Day22 {
    fn part1(&self) -> String { 
        Self::battle(Entity::new(50, 0, 0), self.boss.clone()).to_string()
    }
    fn part2(&self) -> String { format!("") } 
}

#[cfg(test)]
mod tests {
    //use super::*;
    //#[test] fn test1() {}
    //#[test] fn test2() {}
}