use std::str::FromStr;

fn main() {
    // let input = include_str!("../../input/day11.test.txt");
    let input = include_str!("../../input/day11.txt");

    let mut layout = input.parse::<Layout>().unwrap_or_else(|_| panic!());

    layout.simulate();
    println!("{}", layout.count_seats_with_state(SeatState::Occupied));
}

#[derive(Debug)]
pub struct Layout {
    grid: Vec<SeatState>,
    width: usize,
    height: usize,
}

impl FromStr for Layout {
    type Err = peg::error::ParseError<peg::str::LineCol>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::Parse(s)
    }
}

impl Layout {
    fn simulate(&mut self) {
        while self.step() {}
    }

    fn step(&mut self) -> bool {
        let mut next_grid = self.grid.clone();

        let mut nr_changes = 0;
        for i in 0..self.grid.len() {
            let number_of_adjacent_occupied_seats = self
                .get_adjacent_indeces(i)
                .iter()
                .map(|v| self.grid[*v].clone())
                .filter(|v| *v == SeatState::Occupied)
                .count();

            match self.grid[i] {
                SeatState::Available => {
                    if number_of_adjacent_occupied_seats == 0 {
                        next_grid[i] = SeatState::Occupied;
                        nr_changes += 1;
                    }
                }
                SeatState::Occupied => {
                    if number_of_adjacent_occupied_seats >= 5 {
                        next_grid[i] = SeatState::Available;
                        nr_changes += 1;
                    }
                }
                _ => (),
            };
        }

        self.grid = next_grid;

        nr_changes > 0
    }

    fn get_adjacent_indeces(&self, n: usize) -> Vec<usize> {
        vec![
            self.find_seat(n, (-1, -1)), // NW
            self.find_seat(n, (-1, 0)),  // N
            self.find_seat(n, (-1, 1)),  // NE
            self.find_seat(n, (0, 1)),   // E
            self.find_seat(n, (1, 1)),   // SE
            self.find_seat(n, (1, 0)),   // S
            self.find_seat(n, (1, -1)),  // SW
            self.find_seat(n, (0, -1)),  // W
        ]
        .iter()
        .filter_map(|v| *v) // remove None, unwrap Some
        .collect()
    }

    fn find_seat(&self, n: usize, dir: (i32, i32)) -> Option<usize> {
        let (mut row, mut col) = self.index_to_rowcol(n);

        row += dir.0;
        col += dir.1;
        loop {
            match self.rowcol_to_index((row, col)) {
                Some(n) => {
                    if self.grid[n] == SeatState::Floor {
                        row += dir.0;
                        col += dir.1;
                    } else {
                        return Some(n);
                    }
                }
                None => return None,
            }
        }
    }

    fn index_to_rowcol(&self, i: usize) -> (i32, i32) {
        let row = (i / self.width) as i32;
        let col = (i % self.width) as i32;

        (row, col)
    }

    fn rowcol_to_index(&self, pos: (i32, i32)) -> Option<usize> {
        if pos.0 >= 0 && pos.0 < (self.height as i32) && pos.1 >= 0 && pos.1 < (self.width as i32) {
            let n = ((pos.0 as usize) * self.width) + (pos.1 as usize);
            Some(n)
        } else {
            None
        }
    }

    pub fn count_seats_with_state(&self, state: SeatState) -> usize {
        self.grid.iter().filter(|&v| *v == state).count()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SeatState {
    Floor,
    Available,
    Occupied,
}

peg::parser! {
    grammar parser() for str {
        pub rule Parse() -> Layout
            = rows:row() ++ eol()
            {
                let mut grid = Vec::new();
                let width = rows[0].len();
                let height= rows.len();

                for row in rows {
                    grid.append(&mut row.clone());
                }

                Layout{ grid, width, height }
            }

        rule row() -> Vec<SeatState>
            = items:( seat_floor() / seat_available() / seat_occupied() )+
            {
                items
            }

        rule seat_floor() -> SeatState
            = $(['.']) { SeatState::Floor }

        rule seat_available() -> SeatState
            = $(['L']) { SeatState::Available }

        rule seat_occupied() -> SeatState
            = $(['#']) { SeatState::Occupied }

        rule eol()
            = "\n"
            / "\r"
            / "\r" "\n"
    }
}
