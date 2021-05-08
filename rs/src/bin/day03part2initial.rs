fn main() {
    let input = include_str!("../../input/day03.txt");
    let topo_map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let slopes: Vec<(usize, usize)> = vec![
        (1, 1), // Right 1, down 1.
        (3, 1), // Right 3, down 1. (This is the slope you already checked.)
        (5, 1), // Right 5, down 1.
        (7, 1), // Right 7, down 1.
        (1, 2), // Right 1, down 2.
    ];

    let result = slopes
        .iter()
        .map(|x| traverse(&topo_map, x.0, x.1))
        .fold(1, |accum, x| accum * x);
    println!("{:?}", result);
}

fn traverse(tm: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    traverse_step(tm, x, y, x, y)
}

fn traverse_step(tm: &Vec<Vec<char>>, x: usize, y: usize, cx: usize, cy: usize) -> usize {
    if cy >= tm.len() {
        return 0;
    }

    let cx = cx % tm[0].len();

    let hit = if tm[cy][cx] == '#' { 1 } else { 0 };

    hit as usize + traverse_step(tm, x, y, cx + x, cy + y)
}

#[test]
fn test_traverse() {
    let input = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
    let topo_map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    assert_eq!(traverse(&topo_map, 1, 1), 2);
    assert_eq!(traverse(&topo_map, 3, 1), 7);
    assert_eq!(traverse(&topo_map, 5, 1), 3);
    assert_eq!(traverse(&topo_map, 7, 1), 4);
    assert_eq!(traverse(&topo_map, 1, 2), 2);
}
