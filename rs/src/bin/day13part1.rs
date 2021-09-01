fn main() {
    // let input = include_str!("../../input/day13.test.txt");
    let input = include_str!("../../input/day13.txt");

    let (time, busses) = parse_input(input).unwrap();
    println!("time={} busses={:?}", time, busses);

    let mut waits: Vec<_> = busses
        .iter()
        .map(|bus_id| {
            let diff = time % bus_id;
            if diff > 0 {
                (bus_id, (bus_id - diff))
            } else {
                (bus_id, 0)
            }
        })
        .collect();
    waits.sort_by(|a, b| (a.1).partial_cmp(&b.1).unwrap());

    println!("waits={:?}", waits);
    println!(
        "{} * {} = {}",
        waits[0].0,
        waits[0].1,
        (waits[0].0 * waits[0].1)
    );
}

fn parse_input(s: &str) -> Result<(u32, Vec<u32>), peg::error::ParseError<peg::str::LineCol>> {
    parser::parse(s)
}

peg::parser! {
    grammar parser() for str {
        pub rule parse() -> (u32, Vec<u32>)
            = time:number() eol() busses:number_csv()
            {
                (time, busses)
            }

        rule number() -> u32
            = n:$(['0'..='9']+)
              {
                  n.parse().unwrap()
              }

        rule number_csv() -> Vec<u32>
            = ns:number_or_x() ++ ","
            {
                ns.iter().filter_map(|&v| v).collect()
            }

        rule number_or_x() -> Option<u32>
            = n:number() { Some(n) }
            / "x" { None }

        rule eol()
            = "\n"
            / "\r"
            / "\r" "\n"
    }
}
