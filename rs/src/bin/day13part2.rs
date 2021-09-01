fn main() -> Result<(), parser::ParseErr> {
    // let input = include_str!("../../input/day13.test.txt");
    let input = include_str!("../../input/day13.txt");

    let busses = parser::parse(input)?;

    // let (offset, max_bus_id) = busses
    //     .iter()
    //     .max_by(|a, b| (a.1).partial_cmp(&b.1).unwrap())
    //     .unwrap();
    // println!("offset={} max_bus_id={}", offset, max_bus_id);

    // let mut time = max_bus_id - offset;
    // loop {
    //     if check_time(time, &busses) {
    //         break;
    //     }
    //     time += max_bus_id;
    // }

    let time = number_theory::crt(&busses);
    println!("time={}", time);

    Ok(())
}

mod parser {
    pub fn parse(s: &str) -> Result<Vec<(usize, usize)>, ParseErr> {
        parser::parse(s)
    }

    pub type ParseErr = peg::error::ParseError<peg::str::LineCol>;

    peg::parser! {
        grammar parser() for str {
            pub rule parse() -> (Vec<(usize,usize)>)
                = _:number() eol() busses:number_csv()
                {
                    busses
                }

            rule number() -> usize
                = n:$(['0'..='9']+)
                  {
                      n.parse().unwrap()
                  }

            rule number_csv() -> Vec<(usize,usize)>
                = ns:number_or_x() ++ ","
                {
                    ns.into_iter().enumerate().filter(|&(i, v)| v.is_some()).map(|(i, v)| (i, v.unwrap())).collect()
                }

            rule number_or_x() -> Option<usize>
                = n:number() { Some(n) }
                / "x" { None }

            rule eol()
                = "\n"
                / "\r"
                / "\r" "\n"
        }
    }
}

fn check_time(time: usize, busses: &Vec<(usize, usize)>) -> bool {
    !busses
        .iter()
        .any(|(offset, bus_id)| (time + offset) % bus_id != 0)
}

mod number_theory {
    // Copied from https://gist.github.com/miseran/abf1629c6498538a0175ff7548635317
    pub fn crt(vs: &[(usize, usize)]) -> usize {
        let prod = vs.iter().map(|(_, v)| *v as i64).product();
        vs.iter()
            .map(|&(i, v)| -(i as i64) * (prod / v as i64) * inv_mod(prod / v as i64, v as i64))
            .sum::<i64>()
            .rem_euclid(prod) as usize
    }

    fn inv_mod(x: i64, p: i64) -> i64 {
        // p must be prime
        (0..p - 2).fold(1, |o, _| (o * x) % p)
    }
}
