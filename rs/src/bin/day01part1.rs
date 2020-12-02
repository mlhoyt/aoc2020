fn main() {
    let input = include_str!("../../input/day01.txt");
    let values = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let result = generate_pairs(values)
        .iter()
        .filter(|x| x.0 + x.1 == 2020)
        .map(|x| x.0 * x.1)
        .collect::<Vec<_>>();
    println!("{:?}", result);
}

fn generate_pairs(xs: Vec<i32>) -> Vec<(i32, i32)> {
    let mut ps = Vec::<(i32, i32)>::new();

    for (xi, x) in xs.iter().enumerate() {
        let ys = &xs[xi + 1..];
        for y in ys.iter() {
            ps.push((*x, *y));
        }
    }

    return ps;
}

#[test]
fn test_generate_pairs() {
    let values: Vec<i32> = vec![0, 1, 2];
    let pairs = generate_pairs(values);

    let expected: Vec<(i32, i32)> = vec![(0, 1), (0, 2), (1, 2)];
    assert_eq!(pairs, expected);
}
