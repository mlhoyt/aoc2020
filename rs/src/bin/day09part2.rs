fn main() {
    let input = include_str!("../../input/day09.txt");
    let preamble_length = 25;

    let code: Vec<u32> = input.lines().filter_map(|v| v.parse().ok()).collect();

    match xmas_validate(code.as_slice(), preamble_length) {
        Some(v) => match xmas_find_weakness(code.as_slice(), v) {
            Some((v_min, v_max)) => {
                println!("xmas encoding weakness = {}", (v_min + v_max));
            }
            None => {
                println!("xmas encoding weakness NOT found");
            }
        },
        None => {
            println!("xmas encoding is valid");
        }
    }
}

fn xmas_validate(input: &[u32], n: usize) -> Option<u32> {
    for m in (n + 1)..input.len() {
        let window = &input[m - n - 1..m];
        let next_val = input[m];
        if !xmas_check_window(window, next_val) {
            return Some(next_val);
        }
    }

    None
}

fn xmas_check_window(input: &[u32], value: u32) -> bool {
    generate_pairs(input)
        .iter()
        .any(|v| (v.0 + v.1 == value) && (v.0 != v.1))
}

// This is leveraged from day01part1
fn generate_pairs(xs: &[u32]) -> Vec<(u32, u32)> {
    let mut pairs = Vec::new();

    for (xi, x) in xs.iter().enumerate() {
        let ys = &xs[xi + 1..];
        for y in ys {
            pairs.push((*x, *y));
        }
    }

    pairs
}

fn xmas_find_weakness(input: &[u32], value: u32) -> Option<(u32, u32)> {
    for vi in 0..input.len() {
        let mut vn = vi;
        let mut sum = 0;
        while vn < input.len() && sum < value {
            sum += input[vn];
            vn += 1;
        }

        if sum == value {
            let window = &input[vi..vn];
            let v_min = window.iter().min();
            let v_max = window.iter().max();

            if v_min.is_some() && v_max.is_some() {
                return Some((*v_min.unwrap(), *v_max.unwrap()));
            }
        }
    }

    None
}
