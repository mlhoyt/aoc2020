use std::cmp;

fn main() {
    let input = include_str!("../../input/day05.txt");
    let seats = input.lines().map(|x| Seat::new(x)).collect::<Vec<_>>();
    println!("{:?} - {:?}\n", seats.len(), seats);

    let result = seats.iter().map(|x| x.id()).fold(0, |c, n| cmp::max(c, n));
    println!("{:?}\n", result);
}

#[derive(Debug)]
struct Seat {
    identifier: String,
    row: usize,
    column: usize,
}

impl Seat {
    fn new(identifier: &str) -> Seat {
        let (row, column) = decode_seat_identifier(identifier);
        Seat {
            identifier: identifier.to_string(),
            row,
            column,
        }
    }

    pub fn id(&self) -> usize {
        (self.row * 8) + self.column
    }
}

fn decode_seat_identifier(identifier: &str) -> (usize, usize) {
    let mut rows = (0..128).collect::<Vec<usize>>();
    let mut columns = (0..8).collect::<Vec<usize>>();

    for x in identifier.chars() {
        match x {
            'F' => {
                let (f, _) = rows.split_at(rows.len() / 2);
                rows = f.to_vec();
            }
            'B' => {
                let (_, b) = rows.split_at(rows.len() / 2);
                rows = b.to_vec();
            }
            'L' => {
                let (l, _) = columns.split_at(columns.len() / 2);
                columns = l.to_vec();
            }
            'R' => {
                let (_, r) = columns.split_at(columns.len() / 2);
                columns = r.to_vec();
            }
            _ => {}
        };
    }

    return (rows[0], columns[0]);
}
