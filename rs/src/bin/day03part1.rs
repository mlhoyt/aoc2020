fn main() {
    let input = include_str!("../../input/day03.txt");
    let topo_map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let result = traverse(&topo_map, 3, 1);
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
