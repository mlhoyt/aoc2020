use std::collections::HashMap;

fn main() {
    // let input = include_str!("../../input/day10.test.txt");
    // let input = include_str!("../../input/day10.test2.txt");
    let input = include_str!("../../input/day10.txt");

    let mut adapters: Vec<u32> = input.lines().filter_map(|v| v.parse().ok()).collect();
    adapters.sort();

    // extend the adapters to add the outlet and the device
    let mut chain: Vec<u32> = vec![0]; // outlet
    chain.extend_from_slice(adapters.as_slice());
    chain.push(adapters[adapters.len() - 1] + 3); // device

    let deltas = analyze_deltas(chain.as_slice());
    let d1s = deltas.get(&1).unwrap_or(&0);
    let d3s = deltas.get(&3).unwrap_or(&0);

    println!("1s:{} 3s:{} prod:{}", d1s, d3s, (d1s * d3s));
}

fn analyze_deltas(xs: &[u32]) -> HashMap<u32, u32> {
    let mut ds = HashMap::<u32, u32>::new();

    for i in 1..xs.len() {
        let d = xs[i] - xs[i - 1];

        ds.entry(d).and_modify(|v| *v += 1).or_insert(1);
    }

    ds
}
