use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/day06.txt");
    let answers = input
        .split("\n\n")
        .map(|s| parse_group_answers(s))
        .collect::<Vec<_>>();

    println!("{:?}", answers);
    println!("{:?}", answers.into_iter().sum::<u16>());
}

fn parse_group_answers(s: &str) -> u16 {
    let mut answers = HashMap::<u8, u16>::new();

    let mut group_size: u16 = 0;
    for ms in s.split('\n') {
        parse_member_answers(&mut answers, ms.as_bytes());
        group_size += 1;
    }

    let mut complete_answers = 0;
    for v in answers.values() {
        if *v == group_size {
            complete_answers += 1;
        }
    }

    complete_answers
}

fn parse_member_answers(map: &mut HashMap<u8, u16>, answers: &[u8]) {
    for a in answers {
        let count = map.entry(*a).or_insert(0);
        *count += 1;
    }
}
