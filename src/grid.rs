#![allow(dead_code)]

use crate::{options::Options, row::Row, HAlign, VAlign};

/// Builder for the [`Grid`] type.
///
/// [`Grid`]: struct.Grid.html
pub struct GridBuilder {
    inner: Grid,
}

/// Data type for crating a [`Grid`].
///
/// [`Grid`]: struct.Grid.html
pub struct Grid {
    /// These options will be used if the equivalent is not provided
    /// by the underlying [`Row`] type.
    ///
    /// [`Row`]: struct.Row.html
    pub default_options: Options,

    /// Width in chars for each column of the [`Grid`].
    ///
    /// [`Grid`]: struct.Grid.html
    pub column_width: Option<usize>,

    /// Number of char spaces for each padding space between grid columns.
    pub padding_size: Option<usize>,

    /// Collection of rows that this [`Grid`] contains.
    ///
    /// [`Grid`]: struct.Grid.html
    pub rows: Vec<Row>,
}

impl Grid {
    /// Creates a [`Grid`] bye specifying its rows.
    ///
    /// [`Grid`]: struct.Grid.html
    pub fn new(rows: Vec<Row>) -> Self {
        let default_options = Options {
            col_span: None,
            h_align: None,
            v_align: None,
            blank_char: None,
        };
        let grid = Self {
            default_options,
            column_width: None,
            padding_size: None,
            rows,
        };
        grid
    }

    /// Creates a [`GridBuilder`] initiated with rows.
    ///
    /// [`GridBuilder`]: struct.GridBuilder.html
    pub fn builder(rows: Vec<Row>) -> GridBuilder {
        GridBuilder {
            inner: Grid::new(rows),
        }
    }

    /// Format the grid into a string.
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            row.render(
                f,
                &self.default_options,
                self.column_width,
                self.padding_size,
            )?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.render(f)
    }
}

impl GridBuilder {
    /// Builds a [`Grid`] from a [`GridBuilder`].
    ///
    /// [`GridBuilder`]: struct.GridBuilder.html
    /// [`Grid`]: struct.Grid.html
    pub fn build(self) -> Grid {
        self.inner
    }

    /// Sets the default column span for all the cells of the grid. If a cell specifies
    /// a column span it will be used instead of the grids default value.
    pub fn default_colspan(mut self, default_colspan: usize) -> Self {
        if default_colspan == 0 {
            panic!("Column span cannot be 0!");
        }
        self.inner.default_options.col_span = Some(default_colspan);
        self
    }

    /// Sets the default horizontal alignment for all the cells of the grid. If a cell specifies
    /// a horizontal alignment it will be used instead of the grids default value.
    pub fn default_h_align(mut self, default_h_align: HAlign) -> Self {
        self.inner.default_options.h_align = Some(default_h_align);
        self
    }

    /// Sets the default vertical alignment for all the cells of the grid. If a cell specifies
    /// a vertical alignment it will be used instead of the grids default value.
    pub fn default_v_align(mut self, default_v_align: VAlign) -> Self {
        self.inner.default_options.v_align = Some(default_v_align);
        self
    }

    /// Sets the default blank char for all the cells of the grid. If a cell specifies
    /// a blank char it will be used instead of the grids default value.
    pub fn default_blank_char(mut self, default_blank_char: char) -> Self {
        self.inner.default_options.blank_char = Some(default_blank_char);
        self
    }

    /// Width of each column in the grid in number of chars.
    pub fn column_width(mut self, column_width: usize) -> Self {
        self.inner.column_width = Some(column_width);
        self
    }

    /// Width of each padding space in the grid in number of chars.
    pub fn padding_size(mut self, padding_size: usize) -> Self {
        self.inner.padding_size = Some(padding_size);
        self
    }
}
