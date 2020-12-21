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
        .split(',')
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
    fn part1(food_list: &[Food]) -> usize {
        let possible_allergens = find_possible_allergens(food_list);
        count_ingredients_without_allergens(&food_list, &possible_allergens)
    }

    fn part2(food_list: &[Food]) -> String {
        let mut found_allergens: HashMap<String, String> = HashMap::new();

        let mut possible_allergens = find_possible_allergens(food_list);

        while found_allergens.len() != possible_allergens.len() {
            for (allergen, ingredients) in possible_allergens.iter_mut() {
                if found_allergens.contains_key(allergen) {
                    continue;
                }
                if ingredients.len() == 1 {
                    found_allergens
                        .insert(allergen.clone(), ingredients.iter().next().unwrap().clone());
                    continue;
                }
                for found in found_allergens.values() {
                    ingredients.remove(found);
                }
            }
        }

        let mut allergen_list: Vec<String> = found_allergens.keys().cloned().collect();
        allergen_list.sort_unstable();

        let mut cdil = "".to_owned();
        for (i, allergen) in allergen_list.iter().enumerate() {
            cdil += found_allergens.get(allergen).unwrap();
            if i < allergen_list.len() - 1 {
                cdil += ",";
            }
        }

        cdil
    }
}

fn main() {
    let input = fs::read_to_string("./input/day21.txt").expect("File not found!");
    let food_list: Vec<Food> = input.lines().map(parse_food).collect();

    let timer = Instant::now();
    println!(
        "p1: {} (runtime: {:?})",
        Solution::part1(&food_list),
        timer.elapsed()
    );

    let timer = Instant::now();
    println!(
        "p2: {} (runtime: {:?})",
        Solution::part2(&food_list),
        timer.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day21() {
        let input = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        let food_list: Vec<Food> = input.lines().map(parse_food).collect();
        assert_eq!(Solution::part1(&food_list), 5);
        assert_eq!(Solution::part2(&food_list), "mxmxvkd,sqjhc,fvjkl");
    }
}
