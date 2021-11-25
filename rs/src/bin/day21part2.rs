use aoc;
use std::collections::HashMap;
use std::collections::HashSet;

// const FILE_NAME: &str = "input/day21.test.txt";
const FILE_NAME: &str = "input/day21.txt";

fn main() {
    let input = aoc::read_file(FILE_NAME).expect("cannot read file");

    let foods = parser::parse(&input).expect("cannot parse input");

    // extract list of allergens from foods
    let allergens: HashSet<_> = foods
        .iter()
        .map(|v| match &v.allergens {
            Some(vs) => vs.clone(),
            _ => Vec::new(),
        })
        .flatten()
        .collect();

    // extract list of ingredients from foods
    let ingredients: HashSet<_> = foods
        .iter()
        .map(|v| v.ingredients.clone())
        .flatten()
        .collect();

    // initialize a matrix of the cartesian product of allergens and ingredients
    let mut cp = Vec::new();
    for allergen in &allergens {
        for ingredient in &ingredients {
            cp.push(((*allergen).clone(), (*ingredient).clone()));
        }
    }
    let mut allergen_ingredients_matrix: HashMap<(String, String), usize> =
        cp.into_iter().map(|v| (v, 0)).collect();

    // populate allergens-ingredients matrix by iterating over foods
    for food in &foods {
        if let Some(allergens) = &food.allergens {
            for allergen in allergens {
                for ingredient in &food.ingredients {
                    allergen_ingredients_matrix
                        .entry(((*allergen).clone(), (*ingredient).clone()))
                        .and_modify(|v| *v += 1);
                }
            }
        }
    }

    // initialize ingredient-allergen map
    let mut ingredient_allergen_map: HashMap<String, String> = HashMap::new();

    // repeatedly analyze allergens-ingredients matrix for standout ingredient
    let mut unmapped_allergens = allergens.clone();
    while unmapped_allergens.len() > 0 {
        let mut mapped_allergens: Vec<String> = Vec::new();

        unmapped_allergens.iter().for_each(|a| {
            let oi = analyze_allergen(&allergen_ingredients_matrix, a);

            if let Some(i) = oi {
                ingredient_allergen_map.insert(i.clone(), (*a).clone());

                allergens.iter().for_each(|a| {
                    allergen_ingredients_matrix
                        .entry(((*a).clone(), i.clone()))
                        .and_modify(|v| *v = 0);
                });

                mapped_allergens.push((*a).clone());
            }
        });

        mapped_allergens.into_iter().for_each(|a| {
            unmapped_allergens.remove(&a);
        });
    }

    // create a list of ingredient-allergen tuples sorted by the allergen
    let mut result: Vec<_> = ingredient_allergen_map
        .iter()
        .map(|(k, v)| (k, v))
        .collect();
    result.sort_by_key(|(_, a)| (*a).clone());

    // create a comma-separated string of the "ordered" allergic ingredients
    let result = result
        .into_iter()
        .map(|(i, _)| (*i).clone())
        .collect::<Vec<_>>()
        .join(",");
    println!("result: {}", result);
}

fn analyze_allergen(
    matrix: &HashMap<(String, String), usize>,
    allergen: &String,
) -> Option<String> {
    let mut max_value = 0;
    let mut label: Option<String> = None;

    for ((_, ingredient), &value) in matrix.iter().filter(|((a, _), _)| **a == *allergen) {
        if value > max_value {
            max_value = value;
            label = Some(ingredient.to_string());
        } else if value == max_value {
            if label.is_some() {
                label = None;
            }
        }
    }

    label
}

mod parser {
    use crate::Food;

    pub type ParseErr = peg::error::ParseError<peg::str::LineCol>;

    pub fn parse(s: &str) -> Result<Vec<Food>, ParseErr> {
        parser::parse(s)
    }

    peg::parser! {
        grammar parser() for str {
            pub rule parse() -> Vec<Food>
                = ns:food() ++ eol()
                {
                    ns
                }

            rule food() -> Food
                = ingredients:ingredients() allergens:allergens()?
                {
                    Food{ingredients, allergens}
                }

            rule ingredients() -> Vec<String>
                = ns:identifier() ++ _
                {
                    ns
                }

            rule allergens() -> Vec<String>
                = _ "(" "contains " ns:identifier() ++ ", " ")"
                {
                    ns
                }

            rule identifier() -> String
                = ns:$(['a'..='z']+)
                {
                    String::from(ns)
                }

            rule _()
                = [' ']+

            rule eol()
                = "\n"
                / "\r"
                / "\r" "\n"
        }
    }
}

#[derive(Debug)]
pub struct Food {
    ingredients: Vec<String>,
    allergens: Option<Vec<String>>,
}
