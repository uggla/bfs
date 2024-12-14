use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Add,
};

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy, Default)]
pub struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Coord {
        Coord { x, y }
    }
}

impl From<(isize, isize)> for Coord {
    fn from(value: (isize, isize)) -> Self {
        Coord::new(value.0, value.1)
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord::new(self.x + other.x, self.y + other.y)
    }
}

#[derive(Debug)]
pub struct Grid {
    data: HashMap<Coord, char>,
    rows: usize,
    cols: usize,
    start: Coord,
    end: Coord,
}

impl Grid {
    fn in_grid(&self, coord: &Coord) -> Option<&char> {
        self.data.get(coord)
    }

    fn is_valid(&self, coord: &Coord) -> bool {
        self.in_grid(coord).map_or(false, |c| "SE.".contains(*c))
    }

    fn neighbors(&self, coord: &Coord) -> Vec<Coord> {
        let mut neighbors = Vec::new();
        let directions: [Coord; 4] = [(1, 0).into(), (-1, 0).into(), (0, 1).into(), (0, -1).into()];
        for direction in directions.iter() {
            let neighbor = *coord + *direction;
            if self.is_valid(&neighbor) {
                neighbors.push(neighbor);
            }
        }
        neighbors
    }

    pub fn print(&self) {
        for y in 0..self.rows {
            for x in 0..self.cols {
                print!("{}", self.data[&Coord::new(x as isize, y as isize)]);
            }
            println!();
        }
    }

    pub fn print_path(&self) {
        if let Some(path) = self.bfs() {
            for y in 0..self.rows {
                for x in 0..self.cols {
                    let value = self.data[&Coord::new(x as isize, y as isize)];
                    if "SE".contains(&value.to_string()) {
                        print!("{}", value);
                    } else if path.contains(&Coord::new(x as isize, y as isize)) {
                        print!("*");
                    } else {
                        print!("{}", value);
                    }
                }
                println!();
            }
        } else {
            println!("No path found");
        }
    }

    pub fn bfs(&self) -> Option<Vec<Coord>> {
        let mut visited: HashSet<Coord> = HashSet::new();
        let mut queue: VecDeque<Coord> = VecDeque::new();
        let mut predecessors: HashMap<Coord, Coord> = HashMap::new();

        visited.insert(self.start);
        queue.push_back(self.start);

        while let Some(current) = queue.pop_front() {
            // If we reach the self.end, reconstruct the path
            if current == self.end {
                let mut path = Vec::new();
                let mut step = self.end;
                while step != self.start {
                    path.push(step);
                    step = predecessors[&step];
                }
                path.push(self.start); // Add the start point
                path.reverse(); // Reverse to get the correct order
                return Some(path);
            }

            // Explore neighbors
            for neighbor in &self.neighbors(&current) {
                if self.is_valid(neighbor) && visited.insert(*neighbor) {
                    predecessors.insert(*neighbor, current);
                    queue.push_back(*neighbor);
                }
            }
        }

        // If the self.end is not found
        None
    }
}

impl From<Vec<Vec<char>>> for Grid {
    fn from(value: Vec<Vec<char>>) -> Self {
        from_vec_of_vec(value)
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        from_vec_of_vec(value.lines().map(|l| l.chars().collect()).collect())
    }
}

fn from_vec_of_vec(value: Vec<Vec<char>>) -> Grid {
    let mut data = HashMap::new();
    let mut start = Coord::default();
    let mut end = Coord::default();
    // Y
    let rows = value.len();
    // X
    let cols = value[0].len();

    for (y, row) in value.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            data.insert(Coord::new(x as isize, y as isize), *c);
            if *c == 'S' {
                start = Coord::new(x as isize, y as isize);
            } else if *c == 'E' {
                end = Coord::new(x as isize, y as isize);
            }
        }
    }

    Grid {
        data,
        rows,
        cols,
        start,
        end,
    }
}
