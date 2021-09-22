use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::iter;

// const FILE_NAME: &str = "input/day15.test1.txt";
// const FILE_NAME: &str = "input/day15.test2.txt";
// const FILE_NAME: &str = "input/day15.test3.txt";
const FILE_NAME: &str = "input/day15.txt";

const NUM_ROUNDS: usize = 2020;

pub fn main() {
    let file = fs::File::open(FILE_NAME).expect("cannot open file");
    let file = BufReader::new(file);

    let seed: Vec<usize> = file
        .lines()
        .map(|l| l.expect("cannot read line"))
        .map(|l| {
            l.split(',')
                .map(|v| v.parse().expect("cannot parse unsigned integer"))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();
    println!("seed: {:?}", seed);

    let mut history = History::new();

    let result = seed
        .iter()
        .map(|&v| Some(v))
        .chain(iter::repeat(None).take(NUM_ROUNDS - seed.len()))
        .enumerate()
        .fold(0, |prev, (n, curr)| {
            let next = match curr {
                Some(next) => next,
                None => {
                    match history.get(prev) {
                        (None, None) => 0,
                        (Some(_), None) => 0,
                        (Some(a), Some(b)) => a - b,
                        _ => prev, // Unreachable.  Should it panic?
                    }
                }
            };

            history.set(next, n);
            next
        });
    println!("{}", result);
}

#[derive(Debug)]
struct History {
    map: HashMap<usize, (Option<usize>, Option<usize>)>,
}

impl History {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn get(&mut self, key: usize) -> (Option<usize>, Option<usize>) {
        let entry = self.map.entry(key).or_insert((None, None));
        *entry
    }

    fn set(&mut self, key: usize, value: usize) {
        self.map
            .entry(key)
            .and_modify(|v| {
                let (v1, _) = v;
                *v = (Some(value), *v1);
            })
            .or_insert((Some(value), None));
    }
}
