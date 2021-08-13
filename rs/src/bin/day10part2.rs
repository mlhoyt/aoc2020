fn main() {
    // let input = include_str!("../../input/day10.test.txt");
    // let input = include_str!("../../input/day10.test2.txt");
    let input = include_str!("../../input/day10.txt");

    let outlet = 0;
    let mut adapters: Vec<u64> = input.lines().filter_map(|v| v.parse().ok()).collect();
    adapters.sort();
    let device = adapters[adapters.len() - 1] + 3;

    // extend the adapters to add the outlet and the device
    let xs: Vec<u64> = vec![vec![outlet], adapters, vec![device]].concat();

    let result: u64 = adapters_to_sections(xs)
        .iter()
        .map(|x| x.as_slice())
        .map(count_arrangements)
        .product();

    println!("{}", result);
}

// Divide the input vector into sub-vectors (sections) at deltas of 3.
// Ideally this would be replaced by '.group_by(|a,b| b-a >= 3)' but that is
// currently unstable.
fn adapters_to_sections(xs: Vec<u64>) -> Vec<Vec<u64>> {
    let mut sections = Vec::new();

    let mut xb = 0;
    for xn in 1..xs.len() {
        if xs[xn] - xs[xn - 1] >= 3 {
            sections.push(xs[xb..xn].to_vec());

            xb = xn;
        }
    }

    sections.push(xs[xs.len() - 1..].to_vec());

    sections
}

fn count_arrangements(xs: &[u64]) -> u64 {
    if xs.len() < 3 {
        return 1;
    }

    count_arrangements(&xs[1..])
        + if xs[2] - xs[0] <= 3 {
            let xs_p = [xs[..1].to_vec(), xs[2..].to_vec()].concat();

            count_arrangements(&xs_p)
        } else {
            0
        }
}
