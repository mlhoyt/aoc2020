fn main() {
    let input = include_str!("../../input/day01.txt");
    let values = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let result = generate_triples(values)
        .iter()
        .filter(|x| x.0 + x.1 + x.2 == 2020)
        .map(|x| x.0 * x.1 * x.2)
        .collect::<Vec<_>>();
    println!("{:?}", result);
}

fn generate_triples(xs: Vec<i32>) -> Vec<(i32, i32, i32)> {
    let mut ps = Vec::<(i32, i32, i32)>::new();

    for (xi, x) in xs.iter().enumerate() {
        let ys = &xs[xi + 1..];
        for (yi, y) in ys.iter().enumerate() {
            let zs = &ys[yi + 1..];
            for z in zs.iter() {
                ps.push((*x, *y, *z));
            }
        }
    }

    return ps;
}

#[test]
fn test_generate_triples() {
    let values: Vec<i32> = vec![0, 1, 2, 3];
    let groups = generate_triples(values);

    let expected: Vec<_> = vec![(0, 1, 2), (0, 1, 3), (0, 2, 3), (1, 2, 3)];
    assert_eq!(groups, expected);
}
