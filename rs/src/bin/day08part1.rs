use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let input = include_str!("../../input/day08.txt");

    let instrs: Vec<Instr> = input
        .lines()
        .map(|v| v.parse())
        .filter(|v| v.is_ok())
        .map(|v| v.unwrap())
        .collect();

    if let ProgramExitStatus::InfiniteLoop(v) = Program::new(instrs).execute() {
        println!("{}", v);
    }
}

#[derive(Debug, Clone)]
struct Instr {
    op: Op,
    arg1: i32,
}

impl FromStr for Instr {
    type Err = InstrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split(' ').collect();

        if tokens.len() != 2 {
            return Err(InstrParseError::new("insufficient number of fields"));
        }

        match tokens[0] {
            "acc" => Ok(Instr {
                op: Op::Acc,
                arg1: tokens[1].parse::<i32>()?,
            }),
            "jmp" => Ok(Instr {
                op: Op::Jmp,
                arg1: tokens[1].parse::<i32>()?,
            }),
            "nop" => Ok(Instr {
                op: Op::Nop,
                arg1: tokens[1].parse::<i32>()?,
            }),
            _ => Err(InstrParseError::new(
                format!("unrecognized operation {}", tokens[0]).as_str(),
            )),
        }
    }
}

impl Instr {
    fn execute(&self, ctx: &ExecutionContext) -> ExecutionContext {
        match self.op {
            Op::Acc => ExecutionContext::new(ctx.pc() + 1, ctx.acc() + self.arg1),
            Op::Jmp => ExecutionContext::new(ctx.pc() + self.arg1, ctx.acc()),
            Op::Nop => ExecutionContext::new(ctx.pc() + 1, ctx.acc()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct InstrParseError {
    details: String,
}

impl InstrParseError {
    fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for InstrParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.details)
    }
}

impl Error for InstrParseError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<ParseIntError> for InstrParseError {
    fn from(err: ParseIntError) -> Self {
        #[allow(deprecated)]
        InstrParseError::new(err.description())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    Acc,
    Jmp,
    Nop,
}

impl fmt::Display for Op {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Acc => {
                write!(fmt, "acc")
            }
            Self::Jmp => {
                write!(fmt, "jmp")
            }
            Self::Nop => {
                write!(fmt, "nop")
            }
        }
    }
}

struct Program {
    instrs: Vec<Instr>,
}

impl Program {
    fn new(instrs: Vec<Instr>) -> Program {
        Program { instrs }
    }

    fn execute(&self) -> ProgramExitStatus {
        let mut cache = HashSet::<i32>::new();
        let mut ctx: ExecutionContext = Default::default();
        loop {
            match cache.get(&ctx.pc()) {
                Some(_) => return ProgramExitStatus::InfiniteLoop(ctx.acc()),
                None => {
                    cache.insert(ctx.pc());

                    let instr = match self.instrs.get(ctx.pc() as usize) {
                        Some(v) => v,
                        None => {
                            if ctx.pc() == self.instrs.len() as i32 {
                                return ProgramExitStatus::Completed(ctx.acc());
                            } else {
                                return ProgramExitStatus::InstrAddrOutOfRange(ctx.pc());
                            }
                        }
                    };

                    ctx = instr.execute(&ctx);
                }
            };
        }
    }
}

enum ProgramExitStatus {
    InfiniteLoop(i32),
    InstrAddrOutOfRange(i32),
    Completed(i32),
}

#[derive(Default, Clone)]
struct ExecutionContext {
    pc: i32,
    acc: i32,
}

impl ExecutionContext {
    pub fn new(pc: i32, acc: i32) -> Self {
        Self { pc, acc }
    }

    pub fn pc(&self) -> i32 {
        self.pc
    }

    pub fn acc(&self) -> i32 {
        self.acc
    }
}
