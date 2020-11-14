#![allow(dead_code)]

pub const DEFAULT_COLSPAN: usize = 1;
pub const DEFAULT_H_ALIGN: HAlign = HAlign::Left;
pub const DEFAULT_V_ALIGN: VAlign = VAlign::Top;
pub const DEFAULT_BLANK_CHAR: char = '\x20';

/// Horizontal alignments for a cell.
#[derive(Clone, Copy)]
pub enum HAlign {
    /// Left align contents of the cell. (default)
    Left,

    /// Right align contents of the cell
    Right,

    /// Center align contents of the cell
    Center,

    /// Fills the entire width of the cell with repeating the content.
    Fill,
}

/// Vertical alignments for a cell.
#[derive(Clone, Copy)]
pub enum VAlign {
    /// Top align contents of the cell. (default)
    Top,

    /// Bottom align contents of the cell
    Bottom,

    /// Middle align contents of the cell
    Middle,
}

/// Data type that represents options for a cell and its content.
pub struct Cell {
    /// The content of the cell. It can be a multi line string or even a nested [`Grid`].
    ///
    /// [`Grid`]: struct.Grid.html
    pub content: String,

    /// Number of columns that this cell will be spreads out into.
    /// If `None` specified, the value [`col_span`] of the grid will
    /// be used. If [`col_span`] of the grid also is `None`, value 1
    /// will be used.
    ///
    /// [`col_span`]: struct.Options.html#structfield.col_span
    ///
    /// # Panics
    ///
    /// Panics if `0` is specified.
    pub col_span: Option<usize>,

    /// Align content of the cell horizontally. If `None` specified,
    /// the value [`h_align`] of the grid will be used. If [`h_align`]
    /// of the grid also is `None`, [`HAlign::Left`] will be used.
    ///
    /// [`h_align`]: struct.Options.html#structfield.h_align
    /// [`HAlign::Left`]: enum.HAlign.html#variant.Left
    pub h_align: Option<HAlign>,

    /// Align content of the cell vertically. If `None` specified,
    /// the value [`v_align`] of the grid will be used. If [`v_align`]
    /// of the grid also is `None`, [`VAlign::Top`] will be used.
    ///
    /// [`v_align`]: struct.Options.html#structfield.v_align
    /// [`VAlign::Top`]: enum.VAlign.html#variant.Top
    pub v_align: Option<VAlign>,

    /// Fill cell padding spaces by this char value. If `None` specified,
    /// the value [`blank_char`] of the grid will be used. If [`blank_char`]
    /// of the grid also is `None`, white space char (`'\x20'`) will be used.
    ///
    /// [`blank_char`]: struct.Options.html#structfield.blank_char
    pub blank_char: Option<char>,
}

impl Cell {
    /// Create a new [`Cell`] by its `content` and [`col_span`] properties.
    /// To specify other properties, use the [`builder`] method instead.
    ///
    /// [`Cell`]: struct.Cell.html
    /// [`col_span`]: struct.Cell.html#structfield.col_span
    /// [`builder`]: struct.Cell.html#method.builder
    pub fn new(content: String, col_span: usize) -> Self {
        if col_span == 0 {
            panic!("Column span cannot be 0");
        }
        Self {
            content,
            col_span: Some(col_span),
            h_align: None,
            v_align: None,
            blank_char: None,
        }
    }

    /// Creates a [`Cell`] with the specified [`col_span`].
    /// The entire width of the cell will be filled by repeating the content.
    ///
    /// [`Cell`]: struct.Cell.html
    /// [`col_span`]: struct.Cell.html#structfield.col_span
    pub fn new_fill(content: String, col_span: usize) -> Self {
        Cell::builder(content, col_span).h_align(HAlign::Fill).build()
    }

    /// Creates an empty [`Cell`] with the specified [`col_span`].
    /// To specify other properties, use the [`builder`] method instead.
    ///
    /// [`Cell`]: struct.Cell.html
    /// [`col_span`]: struct.Cell.html#structfield.col_span
    /// [`builder`]: struct.Cell.html#method.builder
    pub fn new_empty(col_span: usize) -> Self {
        Cell::new("".into(), col_span)
    }

    /// Creates a [`CellBuilder`] initiated with `content` and [`col_span`] properties.
    /// To build the final [`CellBuilder`] call the [`build`] method.
    ///
    /// [`CellBuilder`]: struct.CellBuilder.html
    /// [`col_span`]: struct.Cell.html#structfield.col_span
    /// [`build`]: struct.CellBuilder.html#method.build
    pub fn builder(content: String, col_span: usize) -> CellBuilder {
        CellBuilder {
            inner: Cell::new(content, col_span),
        }
    }
}

/// Builder for the [`Cell`] type.
///
/// [`Cell`]: struct.CEll.html
pub struct CellBuilder {
    inner: Cell,
}

impl CellBuilder {
    /// Builds a [`Cell`] from a [`CellBuilder`].
    ///
    /// [`Cell`]: struct.Cell.html
    /// [`CellBuilder`]: struct.CellBuilder.html
    pub fn build(self) -> Cell {
        self.inner
    }

    /// Sets the content of the cell. It can be a multi line string or even a nested [`Grid`].
    /// To build the final [`Cell`] type, [`build`] method must be called.
    ///
    /// [`Grid`]: struct.Grid.html
    /// [`Cell`]: struct.Cell.html
    /// [`build`]: struct.CellBuilder.html#method.build
    pub fn content(mut self, content: String) -> Self {
        self.inner.content = content;
        self
    }

    /// Sets the column span of the cell
    /// To build the final [`Cell`] type, [`build`] method must be called.
    ///
    /// [`Cell`]: struct.Cell.html
    /// [`build`]: struct.CellBuilder.html#method.build
    pub fn col_span(mut self, col_span: usize) -> Self {
        self.inner.col_span = Some(col_span);
        self
    }

    /// Sets the horizontal alignment of the cell.
    /// To build the final [`Cell`] type, [`build`] method must be called.
    ///
    /// [`Cell`]: struct.Cell.html
    /// [`build`]: struct.CellBuilder.html#method.build
    pub fn h_align(mut self, h_align: HAlign) -> Self {
        self.inner.h_align = Some(h_align);
        self
    }

    /// Sets the vertical alignment of the cell.
    /// To build the final [`Cell`] type, [`build`] method must be called.
    ///
    /// [`Cell`]: struct.Cell.html
    /// [`build`]: struct.CellBuilder.html#method.build
    pub fn v_align(mut self, v_align: VAlign) -> Self {
        self.inner.v_align = Some(v_align);
        self
    }

    /// Sets the blank char as the cell padding space char.
    /// To build the final [`Cell`] type, [`build`] method must be called.
    ///
    /// [`Cell`]: struct.Cell.html
    /// [`build`]: struct.CellBuilder.html#method.build
    pub fn blank_char(mut self, blank_char: char) -> Self {
        self.inner.blank_char = Some(blank_char);
        self
    }
}
