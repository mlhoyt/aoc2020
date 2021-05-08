fn main() {
    let input = include_str!("../../input/day03.txt");
    let topo_map = new_topo_map(input);
    let result = traverse(&topo_map, 3, 1);
    println!("{:?}", result);
}

fn new_topo_map(input: &str) -> TopoMap {
    parser::Parse(input).unwrap()
}

type TopoMap = Vec<Vec<TopoItem>>;

#[derive(PartialEq)]
pub enum TopoItem {
    TopoTree,
    TopoSpace,
}

peg::parser! {
    grammar parser() for str {
        pub rule Parse() -> TopoMap
            = rows:row() ++ eol()
              {
                 rows
              }

        rule row() -> Vec<TopoItem>
            = items:( topoTree() / topoSpace() )+
              {
                  items
              }

        rule topoTree() -> TopoItem
            = item:$(['#'])
              {
                  TopoItem::TopoTree
              }

        rule topoSpace() -> TopoItem
            = item:$(['.'])
              {
                  TopoItem::TopoSpace
              }

        rule eol()
            = "\n"
            / "\r"
            / "\r" "\n"
    }
}

struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn is_on_map(&self, m: &TopoMap) -> bool {
        self.y < m.len()
    }

    fn is_hit(&self, m: &TopoMap) -> bool {
        let nx = self.x % m[0].len();
        let ny = self.y;

        m[ny][nx] == TopoItem::TopoTree
    }

    fn step(&self, x: usize, y: usize) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

fn traverse(m: &TopoMap, x: usize, y: usize) -> usize {
    let mut hits = 0;

    let mut curr = Position::new();
    while curr.is_on_map(m) {
        if curr.is_hit(m) {
            hits += 1;
        }

        curr = curr.step(x, y);
    }

    return hits;
}
