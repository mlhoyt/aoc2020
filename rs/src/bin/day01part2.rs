fn main() {
    let input = include_str!("../../input/day01.txt");
    let numbers: Vec<i32> = input.lines().map(|line| line.parse().unwrap()).collect();
    let groups = generate_groups(numbers.as_slice());
    let answers: Vec<_> = groups
        .into_iter()
        .filter(|x| x.0 + x.1 + x.2 == 2020)
        .collect();

    for answer in answers {
        println!(
            "{} * {} * {} = {}",
            answer.0,
            answer.1,
            answer.2,
            (answer.0 * answer.1 * answer.2)
        );
    }
}

fn generate_groups(xs: &[i32]) -> Vec<(i32, i32, i32)> {
    let mut groups = Vec::new();

    for (xi, x) in xs.iter().enumerate() {
        let ys = &xs[xi + 1..];
        for (yi, y) in ys.iter().enumerate() {
            let zs = &ys[yi + 1..];
            for z in zs {
                groups.push((*x, *y, *z));
            }
        }
    }

    return groups;
}

#[test]
fn test_generate_groups() {
    let values: Vec<i32> = vec![0, 1, 2, 3];
    let groups = generate_groups(values.as_slice());

    let expected: Vec<_> = vec![(0, 1, 2), (0, 1, 3), (0, 2, 3), (1, 2, 3)];
    assert_eq!(groups, expected);
}
