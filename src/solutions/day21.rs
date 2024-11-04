use crate::rpg_simulator::{entities::{Battle, Boss, SwordPlayer}, object::Item};
use super::*;


pub struct Day21 {
    opponent: Boss
}

impl Day21 {
    pub fn new(input: String) -> Self {
        Self { opponent: Boss::new(&input) }
    }

    fn find_cost(
        &self, 
        inventory: &Vec<Vec<Item>>, 
        you: SwordPlayer, 
        cost_cmp_logic: fn(u32, u32, &SwordPlayer, &Boss) -> u32,
        mut cost_to_cmp: u32
    ) -> u32 {
        //Iterate through every possible combination
        for weapon in inventory[0].iter() {
            for armor in inventory[1].iter() {
                for ring1 in inventory[2].iter() {
                    for ring2 in inventory[2].iter() {
                        if ring1.get_name() == ring2.get_name() && ring1.get_name() != "Nothing" { continue; }
                        let mut you_case = you.clone();
                        you_case.add_stats(weapon);
                        you_case.add_stats(armor);
                        you_case.add_stats(ring1);
                        you_case.add_stats(ring2);

                        let cost = weapon.get_cost() + armor.get_cost() + ring1.get_cost() + ring2.get_cost();
                        cost_to_cmp = cost_cmp_logic(cost_to_cmp, cost, &you_case, &self.opponent);
                    } 
                }

            }
        }
        cost_to_cmp
    }
}

impl Solution for Day21 {
    fn part1(&self) -> String {
        let lowest_cost_and_win: fn(u32, u32, &SwordPlayer, &Boss) -> u32 = 
        |lowest_cost, cost, you, opp| {
            if you.battle(opp) { lowest_cost.min(cost) } 
            else { lowest_cost }
        };
        let inventory: Vec<Vec<Item>> = Item::get_inventory();
        let you = SwordPlayer::new(100,0,0); 

        self.find_cost(&inventory, you, lowest_cost_and_win, u32::MAX).to_string()
    }
    fn part2(&self) -> String { 
        let highest_cost_and_lose: fn(u32, u32, &SwordPlayer, &Boss) -> u32 = 
        |lowest_cost, cost, you, opp| {
            if !you.battle(opp) { lowest_cost.max(cost) } 
            else { lowest_cost }
        };
        let inventory: Vec<Vec<Item>> = Item::get_inventory();
        let you = SwordPlayer::new(100,0,0); 

        self.find_cost(&inventory, you, highest_cost_and_lose, 0).to_string() 
    } 
}