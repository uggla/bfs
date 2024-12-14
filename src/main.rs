use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve_maze(
    maze: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    // Your code here

    let rows = maze.len(); // Y
    let cols = maze[0].len(); // X

    let is_valid = |row: isize, col: isize| -> bool {
        row >= 0
            && col >= 0
            && (row as usize) < rows
            && (col as usize) < cols
            && "SE.".contains(&maze[row as usize][col as usize].to_string())
    };

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut predecessors: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    // Directions for moving in the maze: up, down, left, right
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    visited.insert(start);
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        // If we reach the end, reconstruct the path
        if current == end {
            let mut path = Vec::new();
            let mut step = end;
            while step != start {
                path.push(step);
                step = predecessors[&step];
            }
            path.push(start); // Add the starting point
            path.reverse(); // Reverse to get the correct order
            return path;
        }

        // Explore neighbors
        for (dr, dc) in directions.iter() {
            let neighbor = (
                (current.0 as isize + dr) as usize,
                (current.1 as isize + dc) as usize,
            );

            if is_valid(neighbor.0 as isize, neighbor.1 as isize) && visited.insert(neighbor) {
                predecessors.insert(neighbor, current);
                queue.push_back(neighbor);
            }
        }
    }

    // If the end is not found
    vec![]
}

fn main() {
    let maze = vec![
        vec!['S', '.', '.', '.'],
        vec!['#', '#', '.', '#'],
        vec!['.', '.', '.', '.'],
        vec!['.', '#', '#', 'E'],
    ];

    let maze2 = maze.clone();

    let start = (0, 0);
    let end = (3, 3);

    let expected_path = vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2), (2, 3), (3, 3)];

    assert_eq!(solve_maze(maze, start, end), expected_path);

    let start = (0, 0);
    let end = (3, 4);

    assert_eq!(solve_maze(maze2, start, end), vec![]);
}
