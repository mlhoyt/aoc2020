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

    // initialize a matrix (2d) of the cartesian product of allergens and ingredients
    // The matrix is represented as a HashMap where the key is an allergen and ingredient tuple and
    // the value is the number of times that allergen and ingredient pair are both listed in a
    // food.
    let mut cp = Vec::new();
    for allergen in &allergens {
        for ingredient in &ingredients {
            cp.push(((*allergen).clone(), (*ingredient).clone()));
        }
    }
    let mut allergen_ingredients_matrix: HashMap<(String, String), usize> =
        cp.into_iter().map(|v| (v, 0)).collect();

    // populate the allergens-ingredients matrix by counting the allergen and ingredient pair
    // occurrences in the foods
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

    // repeatedly analyze the allergens-ingredients matrix until all allergens have been mapped to
    // an ingredient
    let mut unmapped_allergens = allergens.clone();
    while unmapped_allergens.len() > 0 {
        // initialize a list of allergens associated to an ingredient in this iteration
        let mut mapped_allergens: Vec<String> = Vec::new();

        unmapped_allergens.iter().for_each(|a| {
            // if one ingredient occurred more than any others for the allergen then we create an
            // association and clear all values for that ingredient in the matrix.
            if let Some(i) = analyze_allergen(&allergen_ingredients_matrix, a) {
                // create an ingredient to allergen association
                ingredient_allergen_map.insert(i.clone(), (*a).clone());

                // clear all matrix values for the ingredient
                allergens.iter().for_each(|a| {
                    allergen_ingredients_matrix
                        .entry(((*a).clone(), i.clone()))
                        .and_modify(|v| *v = 0);
                });

                // store the associated allergen for later removal from the unmapped list
                mapped_allergens.push((*a).clone());
            }
        });

        // remove allergens associated to an ingredient in this iteration from the unmapped list
        mapped_allergens.into_iter().for_each(|a| {
            unmapped_allergens.remove(&a);
        });
    }

    // create a set of non-allergen ingredients
    let non_allergen_ingredients: HashSet<_> = ingredients
        .iter()
        .filter(|v| !ingredient_allergen_map.contains_key(*v))
        .collect();

    // count how many times any of the non-allergen ingredients occur in all the foods
    let mut result = 0;
    for food in &foods {
        for ingredient in &food.ingredients {
            if non_allergen_ingredients.contains(ingredient) {
                result += 1;
            }
        }
    }
    println!("result: {}", result);
}

// analyze_allergen examines all the paired ingrediants in the matrix for one that occurs more than
// any other.  If one such ingredient exists then it is returned, otherwise a None is returned.
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
