use std::collections::HashSet;

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
    let mut answers = HashSet::<u8>::new();

    for ms in s.split('\n') {
        parse_member_answers(&mut answers, ms.as_bytes());
    }

    answers.len() as u16
}

fn parse_member_answers(map: &mut HashSet<u8>, answers: &[u8]) {
    for a in answers {
        map.insert(*a);
    }
}
