use super::entities::{Boss, Stats, Wizard};

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


