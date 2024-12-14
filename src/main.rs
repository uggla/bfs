use bfs::{Coord, Grid};
use indoc::indoc;

fn main() {
    // let maze = vec![
    //     vec!['S', '.', '.', '.'],
    //     vec!['#', '#', '.', '#'],
    //     vec!['.', '.', '.', '.'],
    //     vec!['.', '#', '#', 'E'],
    // ];

    let maze = indoc!(
        "
        S...
        ##.#
        ....
        .##E
        "
    );

    let grid = Grid::from(maze);
    grid.print();

    let expected_path: Option<Vec<Coord>> = Some(vec![
        (0, 0).into(),
        (1, 0).into(),
        (2, 0).into(),
        (2, 1).into(),
        (2, 2).into(),
        (3, 2).into(),
        (3, 3).into(),
    ]);

    assert_eq!(grid.bfs(), expected_path);

    println!();
    println!();
    grid.print_path();

    let maze = indoc!(
        "
        S...
        ##.#
        ...#
        .##E
        "
    );

    println!();
    println!();
    let grid = Grid::from(maze);
    grid.print();

    assert_eq!(grid.bfs(), None);

    println!();
    println!();
    grid.print_path();
}
