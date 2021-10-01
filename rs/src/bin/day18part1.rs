use aoc;

// const FILE_NAME: &str = "input/day18.test.txt";
const FILE_NAME: &str = "input/day18.txt";

fn main() {
    let input = aoc::read_file(FILE_NAME).expect("cannot read file");
    // println!("{}", input);

    let exprs = parser::parse(&input).expect("cannot parse input expressions");
    // println!("{:?}", exprs);

    let result: usize = exprs.iter().sum();
    println!("{}", result);
}

mod parser {
    pub type ParseErr = peg::error::ParseError<peg::str::LineCol>;

    pub fn parse(s: &str) -> Result<Vec<usize>, ParseErr> {
        parser::parse(s)
    }

    // Almost verbatim from https://docs.rs/peg/0.7.0/peg/#precedence-climbing
    peg::parser! {
        grammar parser() for str {
            pub rule parse() -> Vec<usize>
                = es:expr() ++ eol()

            rule expr() -> usize = precedence!{
                x:(@) _ "+" _ y:@ { x + y }
                x:(@) _ "*" _ y:@ { x * y }
                --
                n:number() { n }
                "(" _ e:expr() _ ")" { e }
            }

            rule number() -> usize
                = n:$(['0'..='9']+)
                {
                    n.parse().unwrap()
                }

            rule _()
                = [' ']*

            rule eol()
                = "\n"
                / "\r"
                / "\r" "\n"
        }
    }
}
