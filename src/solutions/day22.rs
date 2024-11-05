use super::*;
use crate::rpg_simulator::{
    items::{Spell, SpellType},
    entity::Entity    
};
use std::collections::VecDeque;

#[allow(dead_code)]
pub struct Day22 {
    boss: Entity
}

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

    fn battle(you: Entity, boss: Entity, hard_mode: bool) -> u32 {
        let mut least_mana = u32::MAX; 

        //Queue and insert
        let mut queue = VecDeque::new();
        for spell_idx in 0..SPELLS.len() {
            queue.push_back(
                (
                    you.clone(),
                    boss.clone(),
                    Some(spell_idx),
                    Vec::<Spell>::new(),
                    0
                )
            );
        }
    
        //Main loop that gathers a prev recorded wizard
        while let Some(state) = queue.pop_front() {
            let (mut you, mut boss, cur_spell, mut spells, mut expended_mana) = state;

            if hard_mode {
                you.life_points -= 1;
                if !you.is_alive() {
                    continue;
                }
            }

            if cur_spell.is_none() && spells.iter().any(|spell| spell.is_active()) { continue; }

            for spell in spells.iter_mut() {
                spell.passives(&mut you, &mut boss);
            }
            spells.retain(|spell| spell.is_active());

            if !boss.is_alive() {
                least_mana = least_mana.min(expended_mana);
                continue;
            }

            if let Some(current_spell) = cur_spell {
                let spell_type = &SPELLS[current_spell];

                if spells.iter().any(|spell| spell.0 == *spell_type) { continue }

                expended_mana += match you.cast(spell_type, &mut boss) {
                    Some((Some(spell), cost)) => {
                        spells.push(spell);
                        cost
                    },
                    Some((None, cost)) => cost,
                    None => continue
                };

                if expended_mana >= least_mana { continue }

                for spell in spells.iter_mut() {
                    spell.passives(&mut you, &mut boss);
                }
                spells.retain(|spell| spell.is_active());


                if !boss.is_alive() {
                    least_mana = least_mana.min(expended_mana);
                    continue;
                }

            }
            boss.attack(&mut you);

            if !you.is_alive() { continue }

           
            for i in 0..SPELLS.len() {
                queue.push_back((you.clone(), boss.clone(), Some(i), spells.clone(), expended_mana));
            }
            queue.push_back((you.clone(), boss.clone(), None, spells.clone(), expended_mana));
        }

        least_mana
    }
}

impl Solution for Day22 {
    fn part1(&self) -> String { 
        Self::battle(Entity::new_with_mana(50, 500), self.boss.clone(), false).to_string()
    }
    fn part2(&self) -> String { 
        Self::battle(Entity::new_with_mana(50, 500), self.boss.clone(), true).to_string() 
    } 
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] 
    fn test1() {
        let boss = Entity::new(13, 8, 0);
        let wizard = Entity::new_with_mana(10, 250);
        let mana_expendature = Day22::battle(wizard, boss, false);
        assert_eq!(226, mana_expendature);
    }
    #[test] fn test2() {
        let boss = Entity::new(14, 8, 0);
        let wizard = Entity::new_with_mana(10, 250);
        let mana_expendature = Day22::battle(wizard, boss, false);
        assert_eq!(641, mana_expendature);
    }
}