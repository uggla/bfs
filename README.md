# bfs

A simple module to parse a grid with the following functionalities:

- Find the shortest path between two points using a breadth first search algorithm.
- Display the grid.
- Display the path between start(S) and end(E) points.

```rust
let grid = indoc!(
    "
    S...
    ##.#
    ....
    .##E
    "
);
```

- `S` is the start point.
- `E` is the end point.
- `#` is a wall.
- `.` is an empty space.

Display the grid:

```
S...
##.#
....
.##E
```

Display the path:

```
S**.
##*#
..**
.##E
```

- `*` is the path between start and end points.
