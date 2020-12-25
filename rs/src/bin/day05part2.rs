fn main() {
    let input = include_str!("../../input/day05.txt");
    let seats = input.lines().map(|x| Seat::new(x)).collect::<Vec<_>>();
    println!("{:?} - {:?}\n", seats.len(), seats);

    let seat_ids = seats.iter().map(|x| x.id()).collect::<Vec<_>>();
    println!("{:?} - {:?}\n", seat_ids.len(), seat_ids);

    let empty_seats = (0_i32..(128 * 8))
        .into_iter()
        .map(|id| match seat_ids.iter().any(|n| *n == id) {
            false => Some(id),
            _ => None,
        })
        .flatten()
        .collect::<Vec<_>>();
    println!("{:?} - {:?}\n", empty_seats.len(), empty_seats);

    let isolated_empty_seats = empty_seats
        .windows(3)
        .map(|win| {
            let v1 = win.get(0).unwrap();
            let v2 = win.get(1).unwrap();
            let v3 = win.get(2).unwrap();
            if *v1 + 1 != *v2 && *v2 + 1 != *v3 {
                Some(*v2)
            } else {
                None
            }
        })
        .flatten()
        .collect::<Vec<_>>();
    println!(
        "{:?} - {:?}\n",
        isolated_empty_seats.len(),
        isolated_empty_seats
    );

    // let result = seats.iter().map(|x| x.id()).fold(0, |c, n| cmp::max(c, n));
    // println!("{:?}\n", result);
}

#[derive(Debug)]
struct Seat {
    identifier: String,
    row: i32,
    column: i32,
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

    pub fn id(&self) -> i32 {
        (self.row * 8) + self.column
    }
}

fn decode_seat_identifier(identifier: &str) -> (i32, i32) {
    let mut rows = (0..128).collect::<Vec<i32>>();
    let mut columns = (0..8).collect::<Vec<i32>>();

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
