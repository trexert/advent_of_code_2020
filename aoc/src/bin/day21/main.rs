use std::collections::{HashMap, HashSet};

fn main() {
    let recipes: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| parse_line(line))
        .collect();

    let all_ingredients: HashSet<_> = recipes
        .iter()
        .flat_map(|(ingredients, _allergens)| ingredients.clone())
        .collect();
    let mut all_allergens: HashMap<_, _> = recipes
        .iter()
        .flat_map(|(_ingredients, allergens)| {
            allergens
                .iter()
                .map(|allergen| (*allergen, all_ingredients.clone()))
        })
        .collect();

    recipes.iter().for_each(|(ingredients, allergens)| {
        allergens.iter().for_each(|allergen| {
            all_allergens.insert(allergen, &all_allergens[allergen] & ingredients);
        });
    });

    println!("{:#?}", all_allergens);

    println!(
        "part1: {}",
        part1(&all_ingredients, &all_allergens, &recipes)
    );
    println!("part2: {}", part2(&all_allergens));
}

fn parse_line(line: &str) -> (HashSet<&str>, HashSet<&str>) {
    let mut ingr_allerg = line
        .trim_end_matches(")")
        .split("(contains")
        .map(|s| s.trim());
    let ingredients = ingr_allerg.next().unwrap().split_whitespace().collect();
    let allergens = ingr_allerg.next().unwrap().split(", ").collect();
    (ingredients, allergens)
}

fn part1(
    all_ingredients: &HashSet<&str>,
    all_allergens: &HashMap<&str, HashSet<&str>>,
    recipes: &Vec<(HashSet<&str>, HashSet<&str>)>,
) -> usize {
    let possible_allergen_ingredients: HashSet<_> = all_allergens
        .iter()
        .flat_map(|(_allergen, ingredients)| ingredients.clone())
        .collect();
    let non_allergens = all_ingredients - &possible_allergen_ingredients;
    recipes
        .iter()
        .map(|(ingredients, _allergens)| (ingredients & &non_allergens).len())
        .sum()
}

fn part2(all_allergens: &HashMap<&str, HashSet<&str>>) -> String {
    let mut iter_allergens = all_allergens.clone();
    let mut matched_ingredients: HashSet<&str> = HashSet::with_capacity(all_allergens.len());
    let mut final_allergens: Vec<(&str, &str)> = vec![];
    while matched_ingredients.len() < all_allergens.len() {
        iter_allergens = iter_allergens
            .into_iter()
            .filter(|(allergen, ingredients)| {
                let possible_ingredients = ingredients - &matched_ingredients;
                if possible_ingredients.len() == 1 {
                    let ingredient = possible_ingredients.iter().next().unwrap();
                    matched_ingredients.insert(ingredient);
                    final_allergens.push((allergen, ingredient));
                    false
                } else {
                    true
                }
            })
            .collect();
    }
    final_allergens.sort_unstable();
    final_allergens
        .iter()
        .map(|(_allergen, ingredient)| *ingredient)
        .collect::<Vec<&str>>()
        .join(",")
}
