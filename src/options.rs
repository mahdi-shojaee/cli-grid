use crate::{HAlign, VAlign};

/// Options for the grid system.
pub struct Options {
    /// Default column span for all the cells of the grid. If a cell specifies
    /// a column span it will be used instead of the grids default value.
    pub col_span: Option<usize>,

    /// Default horizontal alignment for all the cells of the grid. If a cell specifies
    /// a horizontal alignment it will be used instead of the grids default value.
    pub h_align: Option<HAlign>,

    /// Default vertical alignment for all the cells of the grid. If a cell specifies
    /// a vertical alignment it will be used instead of the grids default value.
    pub v_align: Option<VAlign>,

    /// Default blank char for all the cells of the grid. If a cell specifies
    /// a blank char it will be used instead of the grids default value.
    pub blank_char: Option<char>,
}
