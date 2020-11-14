//! A small and dependency free crate for formatting the terminal outputs
//! in a column based grid style.
//! ```text
//! [---------------------]     [---------------------]     [---------------------]
//! [---------------------]     [---------------------]     [---------------------]
//! [---------------------]     [---------------------]     [---------------------]
//! <----------1---------->                            <-2->
//!
//! 1: Grid column
//! 2: Grid padding
//! ```
//! Each `Cell` of the grid can span into 1 or more columns.
//! ```text
//! [ Cell with colspan 1 ]     [---------------------]     [---------------------]
//! [-------------- Cell with colspan 2 --------------]     [---------------------]
//! [---------------------------- Cell with colspan 3 ----------------------------]
//! ```
//!
//! Horizontal alignments of cells are [`HAlign::Left`], [`HAlign::Center`],
//! [`HAlign::Right`] and [`HAlign::Fill`],
//!
//! [`HAlign::Left`]: enum.HAlign.html#variant.Left
//! [`HAlign::Center`]: enum.HAlign.html#variant.Center
//! [`HAlign::Right`]: enum.HAlign.html#variant.Right
//! [`HAlign::Fill`]: enum.HAlign.html#variant.Fill
//!
//! Vertical alignments of cells are [`VAlign::Top`], [`VAlign::Middle`] and
//! [`VAlign::Bottom`].
//!
//! [`VAlign::Top`]: enum.VAlign.html#variant.Top
//! [`VAlign::Middle`]: enum.VAlign.html#variant.Middle
//! [`VAlign::Bottom`]: enum.VAlign.html#variant.Bottom
//!
//! # Examples:
//! ```rust
//! use cli_grid::*;
//!
//! let grid = Grid::builder(vec![
//!     Row::new(vec![
//!         Cell::new("1".into(), 1),
//!         Cell::new("1".into(), 1),
//!         Cell::new("1".into(), 1),
//!     ]),
//!     Row::new(vec![
//!         Cell::new("2".into(), 2),
//!         Cell::new("1".into(), 1),
//!     ]),
//!     Row::new(vec![
//!         Cell::new("3".into(), 3),
//!     ]),
//! ])
//! .default_blank_char('.')
//! .column_width(15)
//! .build();
//!
//! let expected = format!(
//!     "{}\n{}\n{}\n",
//!     "1.............. 1.............. 1..............",
//!     "2.............................. 1..............",
//!     "3..............................................",
//! );
//!
//! assert_eq!(grid.to_string(), expected);
//! ```
//! Multi line `Cell`s also is supported:
//! ```rust
//! use cli_grid::*;
//!
//! let grid = Grid::builder(vec![
//!     Row::new(vec![
//!         Cell::new("1".into(), 1),
//!         Cell::new("1\n1\n1".into(), 1),
//!         Cell::new("1".into(), 1),
//!     ]),
//!     Row::new(vec![
//!         Cell::new("2".into(), 2),
//!         Cell::new("1".into(), 1),
//!     ]),
//!     Row::new(vec![
//!         Cell::new("3".into(), 3),
//!     ]),
//! ])
//! .default_blank_char('.')
//! .column_width(15)
//! .build();
//!
//! let expected = format!(
//!     "{}\n{}\n{}\n{}\n{}\n",
//!     "1.............. 1.............. 1..............",
//!     "............... 1.............. ...............",
//!     "............... 1.............. ...............",
//!     "2.............................. 1..............",
//!     "3..............................................",
//! );
//!
//! assert_eq!(grid.to_string(), expected);
//! ```
//! So nested grids also is supported:
//! ```rust
//! use cli_grid::*;
//!
//! let nested_grid = Grid::builder(vec![
//!     Row::new(vec![
//!         Cell::new("1".into(), 1),
//!         Cell::new("1".into(), 1),
//!     ]),
//!     Row::new(vec![
//!         Cell::new("1".into(), 1),
//!         Cell::new("1".into(), 1),
//!     ]),
//!     Row::new(vec![
//!         Cell::new("1".into(), 1),
//!         Cell::new("1".into(), 1),
//!     ]),
//! ])
//! .default_h_align(HAlign::Center)
//! .default_blank_char('-')
//! .column_width(5)
//! .build();
//!
//! let grid = Grid::builder(vec![
//!     Row::new(vec![
//!         Cell::new("2".into(), 2),
//!         Cell::new("1".into(), 1),
//!     ]),
//!     Row::new(vec![
//!         Cell::new("1".into(), 1),
//!         Cell::new(nested_grid.to_string(), 1),
//!         Cell::new("1".into(), 1),
//!     ]),
//!     Row::new(vec![
//!         Cell::new("3".into(), 3),
//!     ]),
//! ])
//! .default_h_align(HAlign::Center)
//! .default_v_align(VAlign::Middle)
//! .default_blank_char('.')
//! .column_width(15)
//! .build();
//!
//! let expected = format!(
//!     "{}\n{}\n{}\n{}\n{}\n",
//!     "...............2............... .......1.......",
//!     "............... ..--1-- --1--.. ...............",
//!     ".......1....... ..--1-- --1--.. .......1.......",
//!     "............... ..--1-- --1--.. ...............",
//!     ".......................3.......................",
//! );
//!
//! assert_eq!(grid.to_string(), expected);
//! ```
//! 
//! Empty cells and rows can be created by [`Cell::new_empty`] and [`Row::new_empty`] methods.
//!
//! [`Cell::new_empty`]: struct.Cell.html#method.new_empty
//! [`Row::new_empty`]: struct.Row.html#method.new_empty
//!
//! Filled cells and rows can be created by [`Cell::new_fill`] and [`Row::new_fill`] methods.
//!
//! [`Cell::new_fill`]: struct.Cell.html#method.new_fill
//! [`Row::new_fill`]: struct.Row.html#method.new_fill
//!

mod grid;
mod row;
mod cell;
mod options;

pub use cell::{Cell, CellBuilder, HAlign, VAlign};
pub use grid::{Grid, GridBuilder};
pub use row::{Row, RowBuilder};
pub use options::Options;
