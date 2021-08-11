fn main() {
    // let input = include_str!("../../input/day10.test.txt");
    // let input = include_str!("../../input/day10.test2.txt");
    let input = include_str!("../../input/day10.txt");

    let mut adapters: Vec<u64> = input.lines().filter_map(|v| v.parse().ok()).collect();
    adapters.sort();

    // extend the adapters to add the outlet and the device
    let mut chain: Vec<u64> = vec![0]; // outlet
    chain.extend_from_slice(adapters.as_slice());
    chain.push(adapters[adapters.len() - 1] + 3); // device

    let result: u64 = adapters_to_sections(chain)
        .into_iter()
        .map(section_to_arrangements)
        .product();

    println!("{}", result);
}

// Divide the input vector into sub-vectors (sections) at deltas of 3.
fn adapters_to_sections(xs: Vec<u64>) -> Vec<Vec<u64>> {
    let mut sections = Vec::new();

    let mut xb = 0;
    for xn in 1..xs.len() {
        if xs[xn] - xs[xn - 1] == 3 {
            sections.push(xs[xb..xn].to_vec());

            xb = xn;
        }
    }

    sections.push(xs[xs.len() - 1..].to_vec());

    println!("adapters_to_sections: output={:?}", sections);
    sections
}

// Calculate the number of arrangements for the section knowing the first and
// last elements cannot be removed (because section boundaries were at deltas of
// 3 that cannot be optimized across).
fn section_to_arrangements(xs: Vec<u64>) -> u64 {
    println!("section_to_arrangements: input={:?}", xs);
    if xs.len() < 3 {
        println!("section_to_arrangements: output={}", 1);
        return 1;
    }

    // Analysis of groups-of-3:
    // [0, 1, 2] => d=(1,1) td=2 => 2  (td <  3) = 1c1 + 1c0
    // [0, 1, 3] => d=(1,2) td=3 => 2* (td == 3) = 1c1 + 1c0
    // [0, 2, 3] => d=(2,1) td=3 => 2* (td == 3) = 1c1 + 1c0
    // [0, 2, 4] => d=(2,2) td=4 => 1  (td >  3) = 1c1

    let mut arrangements = Vec::new();

    for xn in 0..xs.len() - 2 {
        if xs[xn + 2] - xs[xn] > 3 {
            arrangements.push(1);
        } else {
            arrangements.push(2);
        }
    }

    let result = filter_arrangements(arrangements).into_iter().product();
    println!("section_to_arrangements: output={}", result);
    result
}

// When groups-of-3 arrangements yield a sequence of three 2's then a correction
// factor (of minus 1) must be applied.
fn filter_arrangements(xs: Vec<u64>) -> Vec<u64> {
    println!("filter_arrangements: input={:?}", xs);
    if xs.len() < 3 {
        println!("filter_arrangements: output={:?}", xs);
        return xs;
    }

    let mut filtered = Vec::new();

    let mut xn = 0;
    while xn < (xs.len() - 2) {
        if xs[xn] == 2 && xs[xn + 1] == 2 && xs[xn + 2] == 2 {
            filtered.push(7);
            xn += 3;
        } else {
            filtered.push(xs[xn]);
            xn += 1;
        }
    }

    while xn < xs.len() {
        filtered.push(xs[xn]);
        xn += 1;
    }

    println!("filter_arrangements: output={:?}", filtered);
    filtered
}

// Analysis of groups-of-4:
// [0, 1, 2, 3] => d=(1,1,1) td=3 => (2,  2)  => 4 = 2c2 + 2c1 + 2c0
// [0, 1, 2, 4] => d=(1,1,2) td=4 => (2,  2*) => 3 = 2c2 + 2c1
// [0, 1, 3, 4] => d=(1,2,1) td=4 => (2*, 2*) => 3 = 2c2 + 2c1
// [0, 1, 3, 5] => d=(1,2,2) td=5 => (2*, 1)  => 2 = 2c2 + (2c1 - 1)
// [0, 2, 3, 4] => d=(2,1,1) td=4 => (2*, 2)  => 3 = 2c2 + 2c1
// [0, 2, 3, 5] => d=(2,1,2) td=5 => (2*, 2*) => 3 = 2c2 + 2c1
// [0, 2, 4, 5] => d=(2,2,1) td=5 => (1,  2*) => 2 = 2c2 + (2c1 - 1)
// [0, 2, 4, 6] => d=(2,2,2) td=6 => (1,  1)  => 1 = 2c2

// Analysis of groups-of-5:
// [0, 1, 2, 3, 4] => d=(1,1,1,1) => (4, 4) => 7 = 3c3 + 3c2 + 3c1
// [0, 1, 2, 3, 5] => d=(1,1,1,2) => (4, 3) => 6 = 3c3 + 3c2 + (3c1 - 1)
// [0, 1, 2, 4, 5] => d=(1,1,2,1) => (3, 3) =>
// [0, 1, 2, 4, 6] => d=(1,1,2,2) => (3, 2) =>
// [0, 1, 3, 4, 5] => d=(1,2,1,1) => (3, 3) =>
// [0, 1, 3, 4, 6] => d=(1,2,1,2) => (3, 3) =>
// [0, 1, 3, 5, 6] => d=(1,2,2,1) => (2, 2) =>
// [0, 1, 3, 5, 7] => d=(1,2,2,2) => (2, 1) =>
// [0, 2, 3, 4, 5] => d=(2,1,1,1) => (3, 4) =>
// [0, 2, 3, 4, 6] => d=(2,1,1,2) => (3, 3) =>
// [0, 2, 3, 5, 6] => d=(2,1,2,1) => (3, 3) =>
// [0, 2, 3, 5, 7] => d=(2,1,2,2) => (3, 2) =>
// [0, 2, 4, 5, 6] => d=(2,2,1,1) => (2, 3) =>
// [0, 2, 4, 5, 7] => d=(2,2,1,2) => (2, 3) =>
// [0, 2, 4, 6, 7] => d=(2,2,2,1) => (1, 2) =>
// [0, 2, 4, 6, 8] => d=(2,2,2,2) => (1, 1) =>
