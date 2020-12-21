use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

#[derive(Debug)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

fn parse_food(food_str: &str) -> Food {
    let token = food_str.split('(').collect::<Vec<_>>();
    let ingredients: HashSet<String> = token[0].split_whitespace().map(str::to_owned).collect();
    let allergens: HashSet<String> = token[1]
        .replace(')', "")
        .replace("contains ", "")
        .split(",")
        .map(|s| s.trim().to_owned())
        .collect();

    Food {
        ingredients,
        allergens,
    }
}

fn find_possible_allergens(food_list: &[Food]) -> HashMap<String, HashSet<String>> {
    let mut possible_allergens: HashMap<String, HashSet<String>> = HashMap::new();

    for food in food_list.iter() {
        for allergen in food.allergens.iter() {
            match possible_allergens.get(allergen) {
                Some(ingredients_set) => {
                    let new_ingredients = ingredients_set
                        .intersection(&food.ingredients)
                        .into_iter()
                        .cloned()
                        .collect();
                    possible_allergens.insert(allergen.clone(), new_ingredients);
                }
                None => {
                    possible_allergens.insert(allergen.clone(), food.ingredients.clone());
                }
            }
        }
    }

    possible_allergens
}

fn count_ingredients_without_allergens(
    food_list: &[Food],
    possible_allergens: &HashMap<String, HashSet<String>>,
) -> usize {
    let mut sum = 0;
    let ingredients_with_allergens: HashSet<String> =
        possible_allergens.values().cloned().flatten().collect();
    for food in food_list.iter() {
        sum += food
            .ingredients
            .iter()
            .filter(|ingredient| !ingredients_with_allergens.contains(*ingredient))
            .count();
    }

    sum
}

struct Solution;

impl Solution {
    fn part1(input: &str) -> usize {
        let food_list: Vec<Food> = input.lines().map(parse_food).collect();
        let possible_allergens = find_possible_allergens(&food_list);
        count_ingredients_without_allergens(&food_list, &possible_allergens)
    }

    // fn part2(input: &str) -> usize {
    // unimplemented!()
    // }
}

fn main() {
    let input = fs::read_to_string("./input/day21.txt").expect("File not found!");

    let timer = Instant::now();
    println!(
        "p1: {} (runtime: {:?})",
        Solution::part1(&input),
        timer.elapsed()
    );

    // let timer = Instant::now();
    // println!(
    // "p2: {} (runtime: {:?})",
    // Solution::part2(&input),
    // timer.elapsed()
    // );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day21_part1() {
        let input = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        assert_eq!(Solution::part1(&input), 5);
    }
}
