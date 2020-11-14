A small and dependency free crate for formatting the terminal outputs
in a column based grid style.

```text
[---------------------]     [---------------------]     [---------------------]
[---------------------]     [---------------------]     [---------------------]
[---------------------]     [---------------------]     [---------------------]
<----------1---------->                            <-2->
1: Grid column
2: Grid padding

```
Each `Cell` of the grid can span into 1 or more columns.
```text
[ Cell with colspan 1 ]     [---------------------]     [---------------------]
[-------------- Cell with colspan 2 --------------]     [---------------------]
[---------------------------- Cell with colspan 3 ----------------------------]
```

Horizontal alignments of cells are `HAlign::Left`, `HAlign::Center`, `HAlign::Right` and `HAlign::Fill`,

Vertical alignments of cells are `VAlign::Top`, `VAlign::Middle` and `VAlign::Bottom`.

# Examples:

```rust
use cli_grid::*;
let grid = Grid::builder(vec![
    Row::new(vec![
        Cell::new("1".into(), 1),
        Cell::new("1".into(), 1),
        Cell::new("1".into(), 1),
    ]),
    Row::new(vec![
        Cell::new("2".into(), 2),
        Cell::new("1".into(), 1),
    ]),
    Row::new(vec![
        Cell::new("3".into(), 3),
    ]),
])
.default_blank_char('.')
.column_width(15)
.build();
let expected = format!(
    "{}\n{}\n{}\n",
    "1.............. 1.............. 1..............",
    "2.............................. 1..............",
    "3..............................................",
);
assert_eq!(grid.to_string(), expected);
```

Multi line `Cell`s also is supported:
```rust
use cli_grid::*;
let grid = Grid::builder(vec![
    Row::new(vec![
        Cell::new("1".into(), 1),
        Cell::new("1\n1\n1".into(), 1),
        Cell::new("1".into(), 1),
    ]),
    Row::new(vec![
        Cell::new("2".into(), 2),
        Cell::new("1".into(), 1),
    ]),
    Row::new(vec![
        Cell::new("3".into(), 3),
    ]),
])
.default_blank_char('.')
.column_width(15)
.build();
let expected = format!(
    "{}\n{}\n{}\n{}\n{}\n",
    "1.............. 1.............. 1..............",
    "............... 1.............. ...............",
    "............... 1.............. ...............",
    "2.............................. 1..............",
    "3..............................................",
);
assert_eq!(grid.to_string(), expected);
```

So nested grids also is supported:
```rust
use cli_grid::*;
let nested_grid = Grid::builder(vec![
    Row::new(vec![
        Cell::new("1".into(), 1),
        Cell::new("1".into(), 1),
    ]),
    Row::new(vec![
        Cell::new("1".into(), 1),
        Cell::new("1".into(), 1),
    ]),
    Row::new(vec![
        Cell::new("1".into(), 1),
        Cell::new("1".into(), 1),
    ]),
])
.default_h_align(HAlign::Center)
.default_blank_char('-')
.column_width(5)
.build();
let grid = Grid::builder(vec![
    Row::new(vec![
        Cell::new("2".into(), 2),
        Cell::new("1".into(), 1),
    ]),
    Row::new(vec![
        Cell::new("1".into(), 1),
        Cell::new(nested_grid.to_string(), 1),
        Cell::new("1".into(), 1),
    ]),
    Row::new(vec![
        Cell::new("3".into(), 3),
    ]),
])
.default_h_align(HAlign::Center)
.default_v_align(VAlign::Middle)
.default_blank_char('.')
.column_width(15)
.build();
let expected = format!(
    "{}\n{}\n{}\n{}\n{}\n",
    "...............2............... .......1.......",
    "............... ..--1-- --1--.. ...............",
    ".......1....... ..--1-- --1--.. .......1.......",
    "............... ..--1-- --1--.. ...............",
    ".......................3.......................",
);
assert_eq!(grid.to_string(), expected);
```

Empty cells and rows can be created by `Cell::new_empty()` and `Row::new_empty()` methods.

Filled cells and rows can be created by `Cell::new_fill()` and `Row::new_fill()` methods.
