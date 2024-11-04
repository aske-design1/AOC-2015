use super::*;

    pub struct Day21 {
        opponent: Entity
    }

    struct Object<'a> {
        name: &'a str,
        cost: u32,
        dmg: u32, 
        armor: u32,
    }

    #[derive(Clone)]
    struct Entity {
        hit_points: u32,
        dmg: u32, 
        armor: u32,
    }

    impl<'a> Object<'a> {
        fn from(name: &'a str, cost: u32, dmg: u32, armor: u32) -> Self {
            Self { name, cost, dmg, armor }
        }
        fn get_inventory() -> Vec<Vec<Self>> {
            vec![
                vec![
                    Object::from("Dagger", 8, 4, 0), 
                    Object::from("Shortsword", 10, 5, 0), 
                    Object::from("Warhammer", 25, 6, 0),
                    Object::from("Longsword", 40, 7, 0),
                    Object::from("Greataxe", 74, 8, 0)
                ],
                vec![
                    Object::from("Nothing", 0, 0, 0),
                    Object::from("Leather", 13, 0, 1),
                    Object::from("Chainmail", 31, 0, 2),
                    Object::from("Splintmail", 53, 0, 3),
                    Object::from("Bandedmail", 75, 0, 4),
                    Object::from("Platemail", 102, 0, 5),
                ],
                vec![
                    Object::from("Nothing", 0, 0, 0),
                    Object::from("Damage +1", 25, 1, 0),
                    Object::from("Damage +2", 50, 2, 0),
                    Object::from("Damage +3", 100, 3, 0),
                    Object::from("Defense +1", 20, 0, 1),
                    Object::from("Defense +2", 40, 0, 2),
                    Object::from("Defense +3", 80, 0, 3),
                ]
            ]
        }

    }

    impl Entity {
        fn from_input(input: &str) -> Self {
            let splitter = if input.contains("\r\n") { "\r\n" } else { "\n" };
            let split_input: Vec<&str> = input.split(splitter).collect();
            
            let hit_points = split_input[0].split_once(": ").unwrap().1.parse().unwrap();
            let dmg = split_input[1].split_once(": ").unwrap().1.parse().unwrap(); 
            let armor = split_input[2].split_once(": ").unwrap().1.parse().unwrap();

            Entity { hit_points, dmg, armor }
        }
        fn new(hit_points: u32) -> Self {
            Self { 
                hit_points,
                dmg: 0,
                armor: 0
            }
        }
        fn battle(&self, other: &Entity) -> bool {
            //true if you win 
            let your_dmg = 1.max(self.dmg as i32 - other.armor as i32) as u32;
            let opp_dmg = 1.max(other.dmg as i32 - self.armor as i32) as u32;

            let your_cycles = self.hit_points / opp_dmg;
            let opp_cycle = other.hit_points / your_dmg;

            //println!("your cycles: {your_cycles}, opp cycles: {opp_cycle}");
            your_cycles >= opp_cycle
        }
        fn add(&mut self, item: &Object) {
            self.dmg += item.dmg;
            self.armor += item.armor;
        }


    }


    impl Day21 {
        pub fn new(input: String) -> Self {
            Self { opponent: Entity::from_input(&input) }
        }

        fn lowest_cost(
            &self, 
            inventory: &Vec<Vec<Object>>, you: Entity, logic: fn(u32, u32, &Entity, &Entity) -> u32,
            default_value: u32)
            -> u32 {
            let mut lowest_cost = default_value;
            
            for i in inventory[0].iter() {
                for j in inventory[1].iter() {
                    for k in inventory[2].iter() {
                        for l in inventory[2].iter() {
                            if k.name == l.name && k.name != "Nothing" { continue; }
                            let mut you_case = you.clone(); 
                            you_case.add(i);
                            you_case.add(j);
                            you_case.add(k);
                            you_case.add(l);

                            let cost = i.cost + j.cost + k.cost + l.cost;
                            lowest_cost = logic(lowest_cost, cost, &you_case, &self.opponent);

                            
                        } 
                    }

                }
            }

            lowest_cost
        }
    }

    impl Solution for Day21 {
        fn part1(&self) -> String {
            let logic: fn(u32, u32, &Entity, &Entity) -> u32 = |lowest_cost, cost, you, opp| {
                if you.battle(opp) {
                    return lowest_cost.min(cost); 
                }
                lowest_cost
            };


            let inventory: Vec<Vec<Object>> = Object::get_inventory();
            let you = Entity::new(100); 
            self.lowest_cost(&inventory, you, logic, u32::MAX).to_string()
        }
        fn part2(&self) -> String { 
            let logic: fn(u32, u32, &Entity, &Entity) -> u32 = |lowest_cost, cost, you, opp| {
                if !you.battle(opp) {
                    return lowest_cost.max(cost); 
                }
                lowest_cost
            };


            let inventory: Vec<Vec<Object>> = Object::get_inventory();
            let you = Entity::new(100); 
            self.lowest_cost(&inventory, you, logic, 0).to_string() 
        } 
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test] 
        fn test1() {
            let mut you = Entity::new(8);
            you.dmg = 5;
            you.armor = 5; 
            let mut opp = Entity::new(12);
            opp.dmg = 7; 
            opp.armor = 2;

            assert!(you.battle(&opp))
        }
        //#[test] fn test2() {}
    }