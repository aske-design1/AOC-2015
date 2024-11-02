use super::*;

    #[allow(dead_code)]
    pub struct Day15 {
        input: Vec<Ingredient>
    }

    struct Ingredient {
        name: String,
        capacity: i32,
        durability: i32, 
        flavor: i32,
        texture: i32,
        calories: i32
    }
    impl Ingredient {
        fn new(name: String, 
            capacity: i32, 
            durability: i32, 
            flavor: i32, 
            texture: i32, 
            calories: i32) -> Self {
            Self { name, capacity, durability, flavor, texture, calories }
        }
        #[allow(dead_code)]
        fn print(&self) {
            println!("Ingredient {{ Name: {}, Capacity: {}, Durability {}, Flavor: {}, Texture: {}, Calories: {} }}",
                self.name,
                self.capacity,
                self.durability,
                self.flavor,
                self.texture,
                self.calories
            )
        }
    }

    #[derive(Clone)]
    struct Categories {
        capacity: i64,
        durability: i64,
        flavor: i64,
        texture: i64
    }
    impl Categories {
        fn calculate_total(&self) -> i64 {
            self.capacity * self.durability * self.flavor * self.texture
        } 
        fn from(&mut self, other: &Categories) {
            self.capacity = other.capacity;
            self.durability = other.durability;
            self.flavor = other.flavor; 
            self.texture = other.texture;
        }

        fn add_teaspoons(&mut self, teaspoons: i64, ingredient: &Ingredient) {
            self.capacity += teaspoons * (ingredient.capacity as i64);
            self.durability += teaspoons * (ingredient.durability as i64);
            self.flavor += teaspoons * (ingredient.flavor as i64);
            self.texture += teaspoons * (ingredient.flavor as i64); 
        }

        #[allow(dead_code)]
        fn print(&self) {
            println!("Categories {{ Capacity: {}, Durability: {}, Flavor: {}, Texture: {} }}", 
                self.capacity,
                self.durability,
                self.flavor,
                self.texture
            )
        } 
    }

    impl Day15 {
        pub fn new(input: String) -> Self {
            let splitter = if input.contains("\r\n") { "\r\n" } else { "\n" };
            let input = 
            input
            .split(splitter)
            .map(|line| Self::parse_line(line))
            .collect::<Vec<Ingredient>>();

            Self { input }
        }

        fn parse_line(line: &str) -> Ingredient {
            let first_split = line.split(": ").collect::<Vec<&str>>();
            let name = first_split[0].to_string();
            let ingredients = first_split[1].split(", ").collect::<Vec<&str>>();

            let mut processed_ingredients: Vec<i32> = Vec::new();
            for ingredient in ingredients.into_iter() {
                processed_ingredients.push(
                    ingredient
                    .split(" ")
                    .collect::<Vec<&str>>()[1]
                    .parse::<i32>()
                    .unwrap()
                );
            }
            Ingredient::new(
                name, 
                processed_ingredients[0], 
                processed_ingredients[1], 
                processed_ingredients[2],
                processed_ingredients[3],
                processed_ingredients[4]
            )
        }

        fn calculate_best_cookie(&self) -> i64 {
            let mut best: i64 = 0; 
            for teaspoons in 1..100 {
                let categories = Categories { capacity: 0, durability: 0, flavor: 0, texture: 0 };
                let categories = self.calculate_best_cookie_helper(teaspoons, 0, categories);
                best = best.max(categories.calculate_total())
            }
            best
        }
        fn calculate_best_cookie_helper(&self, teaspoons: i64, ingredient_idx: usize, mut total: Categories) -> Categories {
            if ingredient_idx+1 == self.input.len() {
                total.add_teaspoons(100 - teaspoons, &self.input[ingredient_idx]);
                return total
            }
        
            let mut best_categories = total.clone();
            best_categories.add_teaspoons(teaspoons, &self.input[ingredient_idx]);

            let mut best = total.calculate_total(); 
            for teaspoons_to_used in 1..100 {
                let current = self.calculate_best_cookie_helper(
                    teaspoons_to_used, ingredient_idx + 1, total.clone()
                );
                let current_sum = current.calculate_total();
                if best < current_sum {
                    best = current_sum;
                    best_categories.from(&current);


                    //cur_total.print();
                    //println!("Teaspoons: {}, Best: {}", teaspoons, best );
                }
            }

            best_categories

        }

    }

    impl<'a> Solution for Day15 {
        fn part1(&self) -> String { self.calculate_best_cookie().to_string() }
        fn part2(&self) -> String { format!("") } 
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test] fn test1() {
            //"-1".parse::<i32>().unwrap();

            let input = 
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\nCinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
            let day = Day15::new(input.to_string());
            assert_eq!(day.part1(), 62842880.to_string())
        }
        //#[test] fn test2() {}
    }