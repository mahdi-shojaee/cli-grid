use crate::{
    cell::{Cell, HAlign, VAlign, DEFAULT_BLANK_CHAR, DEFAULT_H_ALIGN, DEFAULT_V_ALIGN},
    options::Options,
};

use std::borrow::Cow;

/// Data type for creating a [`Row`] for the grid.
///
/// [`Row`]: struct.Row.html
pub struct Row {
    /// These options will be used if the equivalent is not provided
    /// by the underlying [`Cell`] type.
    ///
    /// [`Cell`]: struct.Cell.html
    pub default_options: Options,

    /// Width in chars for each column of the [`Row`].
    ///
    /// [`Row`]: struct.Row.html
    pub column_width: Option<usize>,

    /// Number of char spaces for each padding space between row columns.
    pub padding_size: Option<usize>,

    /// Collection of cells that this [`Row`] contains.
    ///
    /// [`Row`]: struct.Row.html
    pub cells: Vec<Cell>,
}

impl Row {
    /// Creates a new [`Row`] by its cells.
    ///
    /// [`Row`]: struct.Row.html
    pub fn new(cells: Vec<Cell>) -> Self {
        Self {
            default_options: Options {
                col_span: None,
                h_align: None,
                v_align: None,
                blank_char: None,
            },
            column_width: None,
            padding_size: None,
            cells,
        }
    }

    /// Create a new empty row with a specific column span.
    pub fn new_empty(col_span: usize) -> Self {
        Row::new(vec![Cell::new_empty(col_span)])
    }

    /// Create a new row with a specified column span filled by the repeated content.
    pub fn new_fill(content: String, col_span: usize) -> Self {
        Row::new(vec![Cell::new_fill(content, col_span)])
    }

    /// Creates a [`RowBuilder`] initiated with cells.
    /// To build the final [`RowBuilder`] call the [`build`] method.
    ///
    /// [`RowBuilder`]: struct.RowBuilder.html
    /// [`build`]: struct.RowBuilder.html#method.build
    pub fn builder(cells: Vec<Cell>) -> RowBuilder {
        RowBuilder {
            inner: Self::new(cells),
        }
    }

    /// Formats the [`Row`] into a string.
    ///
    /// [`Row`]: struct.Row.html
    pub fn render(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        default_options: &Options,
        column_width: Option<usize>,
        padding_size: Option<usize>,
    ) -> std::fmt::Result {
        let column_width = column_width.or(self.column_width).unwrap_or(1);
        let padding_size = padding_size.or(self.padding_size).unwrap_or(1);
        let mut cols_lines = self
            .cells
            .iter()
            .map(|c| {
                let mut lines = c.content.lines().map(|l| l.to_owned()).collect::<Vec<_>>();
                if lines.is_empty() {
                    lines.push(String::new());
                }
                lines
            })
            .collect::<Vec<_>>();
        let max_lines = cols_lines
            .iter()
            .map(|col_lines| col_lines.len())
            .max()
            .unwrap_or(0);
        for line_index in 0..max_lines {
            for (col_index, col) in self.cells.iter().enumerate() {
                let col_lines = &mut cols_lines[col_index];
                let col_span = col
                    .col_span
                    .or(self.default_options.col_span)
                    .or(default_options.col_span)
                    .unwrap_or(1);
                let col_width = col_span * column_width + padding_size * (col_span - 1);
                let h_align = col
                    .h_align
                    .or(self.default_options.h_align)
                    .or(default_options.h_align)
                    .unwrap_or(DEFAULT_H_ALIGN);
                let v_align = col
                    .v_align
                    .or(self.default_options.v_align)
                    .or(default_options.v_align)
                    .unwrap_or(DEFAULT_V_ALIGN);
                let blank_char = col
                    .blank_char
                    .or(self.default_options.blank_char)
                    .or(default_options.blank_char)
                    .unwrap_or(DEFAULT_BLANK_CHAR);
                if col_index != 0 {
                    write!(f, "{s:<0$}", padding_size, s = "")?;
                }
                write!(
                    f,
                    "{}",
                    col_line(
                        h_align, v_align, col_width, col_lines, max_lines, line_index, blank_char,
                    )
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.render(
            f,
            &self.default_options,
            self.column_width,
            self.padding_size,
        )
    }
}

fn col_line(
    h_align: HAlign,
    v_align: VAlign,
    col_width: usize,
    col_lines: &mut [String],
    max_lines: usize,
    line_index: usize,
    blank_char: char,
) -> Cow<str> {
    let index = match v_align {
        VAlign::Top => {
            let start_line_index = 0;
            let end_line_index = col_lines.len() - 1;
            if line_index >= start_line_index && line_index <= end_line_index {
                Some(line_index)
            } else {
                None
            }
        }
        VAlign::Bottom => {
            let start_line_index = max_lines - col_lines.len();
            let end_line_index = max_lines - 1;
            if line_index >= start_line_index && line_index <= end_line_index {
                Some(line_index - start_line_index)
            } else {
                None
            }
        }
        VAlign::Middle => {
            let start_line_index = (max_lines - col_lines.len()) / 2;
            let end_line_index = start_line_index + col_lines.len() - 1;
            if line_index >= start_line_index && line_index <= end_line_index {
                Some(line_index - start_line_index)
            } else {
                None
            }
        }
    };
    if let Some(i) = index {
        return pad(h_align, &mut col_lines[i], col_width, blank_char);
    }
    Cow::Owned(blank_char.to_string().repeat(col_width))
}

fn pad(h_align: HAlign, s: &mut String, width: usize, blank_char: char) -> Cow<str> {
    let s_chars_len = s.chars().count();
    if s_chars_len >= width {
        let bytes_index = byte_index(s, width);
        return s[..bytes_index].into();
    }
    let blanks = width - s_chars_len;
    match h_align {
        HAlign::Left => {
            s.extend(std::iter::repeat(blank_char).take(blanks));
            s.as_str().into()
        }
        HAlign::Right => {
            let mut new_str = std::iter::repeat(blank_char)
                .take(blanks)
                .collect::<String>();
            new_str.push_str(s);
            new_str.into()
        }
        HAlign::Center => {
            let left_blanks = blanks / 2;
            let right_blanks = blanks - left_blanks;
            let mut new_str = std::iter::repeat(blank_char)
                .take(left_blanks)
                .collect::<String>();
            new_str.push_str(s);
            new_str.extend(std::iter::repeat(blank_char).take(right_blanks));
            new_str.into()
        }
        HAlign::Fill => {
            let repeats = width / s_chars_len + 1;
            let s = s.repeat(repeats);
            let bytes_index = byte_index(&s, width);
            s[..bytes_index].to_owned().into()
        }
    }
}

fn byte_index(s: &str, char_index: usize) -> usize {
    s.char_indices()
        .take(char_index)
        .last()
        .map_or(0, |(i, ch)| i + ch.len_utf8())
}

/// Builder for the [`Row`] type.
///
/// [`Row`]: struct.Row.html
pub struct RowBuilder {
    inner: Row,
}

impl RowBuilder {
    /// Builds a [`Row`] from a [`RowBuilder`].
    ///
    /// [`Row`]: struct.Row.html
    /// [`RowBuilder`]: struct.RowBuilder.html
    pub fn build(self) -> Row {
        self.inner
    }

    /// Sets the default column span for all the cells of the grid. If a cell specifies
    /// a column span it will be used instead of the grids default value.
    pub fn default_colspan(mut self, default_col_span: usize) -> Self {
        self.inner.default_options.col_span = Some(default_col_span);
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

    /// Sets the width of each column in the [`Row`].
    ///
    /// [`Row`]: struct.Row.html
    pub fn column_width(mut self, column_width: usize) -> Self {
        self.inner.column_width = Some(column_width);
        self
    }

    /// Sets the padding size between each column in the [`Row`].
    ///
    /// [`Row`]: struct.Row.html
    pub fn padding_size(mut self, padding_size: usize) -> Self {
        self.inner.padding_size = Some(padding_size);
        self
    }

    /// Sets the cells collection of the [`Row`].
    ///
    /// [`Row`]: struct.Row.html
    pub fn cells(mut self, cells: Vec<Cell>) -> Self {
        self.inner.cells = cells;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_index_ascii() {
        let s = "abc";
        let result = byte_index(s, 2);
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_byte_index_unicode1() {
        let s = "aµc";
        let result = byte_index(s, 2);
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_byte_index_unicode2() {
        let s = "µ∆c";
        let result = byte_index(s, 2);
        let expected = 5;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_pad_left_empty() {
        let s = &mut "".into();
        let result = pad(HAlign::Right, s, 3, '.');
        let expected = String::from("...");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_pad_right_empty() {
        let s = &mut "".into();
        let result = pad(HAlign::Left, s, 3, '.');
        let expected = String::from("...");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_pad_left() {
        let s = &mut "a".into();
        let result = pad(HAlign::Right, s, 3, '.');
        let expected = String::from("..a");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_pad_right() {
        let s = &mut "a".into();
        let result = pad(HAlign::Left, s, 3, '.');
        let expected = String::from("a..");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_pad_center() {
        let s = &mut "a".into();
        let result = pad(HAlign::Center, s, 3, '.');
        let expected = String::from(".a.");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_pad_fill1() {
        let s = &mut "a".into();
        let result = pad(HAlign::Fill, s, 3, '.');
        let expected = String::from("aaa");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_pad_fill2() {
        let s = &mut "ab".into();
        let result = pad(HAlign::Fill, s, 3, '.');
        let expected = String::from("aba");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_pad_left_unicode() {
        let s = &mut "∆".into();
        let result = pad(HAlign::Right, s, 3, '.');
        let expected = String::from("..∆");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_pad_right_unicode() {
        let s = &mut "∆".into();
        let result = pad(HAlign::Left, s, 3, '.');
        let expected = String::from("∆..");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_pad_center_unicode() {
        let s = &mut "∆".into();
        let result = pad(HAlign::Center, s, 3, '.');
        let expected = String::from(".∆.");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_pad_fill_unicode() {
        let s = &mut "∆".into();
        let result = pad(HAlign::Fill, s, 3, '.');
        let expected = String::from("∆∆∆");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_col_line_for_a_content_with_1_row_in_a_3_lines_column_left_top() {
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let col_width = 3;
        let col_lines = &mut [String::from("a")];
        let max_lines = 3;
        let blank_char = '.';

        let line_index = 0;
        let result = col_line(
            h_align, v_align, col_width, col_lines, max_lines, line_index, blank_char,
        );
        let expected = "a..";
        assert_eq!(result, expected);

        let line_index = 1;
        let result = col_line(
            h_align, v_align, col_width, col_lines, max_lines, line_index, blank_char,
        );
        let expected = "...";
        assert_eq!(result, expected);

        let line_index = 2;
        let result = col_line(
            h_align, v_align, col_width, col_lines, max_lines, line_index, blank_char,
        );
        let expected = "...";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_col_line_for_a_content_with_1_row_in_a_3_lines_column_left_middle() {
        let h_align = HAlign::Left;
        let v_align = VAlign::Middle;
        let col_width = 3;
        let col_lines = &mut [String::from("a")];
        let max_lines = 3;
        let blank_char = '.';

        let line_index = 0;
        let result = col_line(
            h_align, v_align, col_width, col_lines, max_lines, line_index, blank_char,
        );
        let expected = "...";
        assert_eq!(result, expected);

        let line_index = 1;
        let result = col_line(
            h_align, v_align, col_width, col_lines, max_lines, line_index, blank_char,
        );
        let expected = "a..";
        assert_eq!(result, expected);

        let line_index = 2;
        let result = col_line(
            h_align, v_align, col_width, col_lines, max_lines, line_index, blank_char,
        );
        let expected = "...";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_col_line_for_a_content_with_1_row_in_a_3_lines_column_left_bottom() {
        let h_align = HAlign::Left;
        let v_align = VAlign::Bottom;
        let col_width = 3;
        let col_lines = &mut [String::from("a")];
        let max_lines = 3;
        let blank_char = '.';

        let line_index = 0;
        let result = col_line(
            h_align, v_align, col_width, col_lines, max_lines, line_index, blank_char,
        );
        let expected = "...";
        assert_eq!(result, expected);

        let line_index = 1;
        let result = col_line(
            h_align, v_align, col_width, col_lines, max_lines, line_index, blank_char,
        );
        let expected = "...";
        assert_eq!(result, expected);

        let line_index = 2;
        let result = col_line(
            h_align, v_align, col_width, col_lines, max_lines, line_index, blank_char,
        );
        let expected = "a..";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_col_line_for_a_content_with_2_row_in_a_4_lines_column_left_top() {
        let h_align = HAlign::Left;
        let v_align = VAlign::Top;
        let col_width = 3;
        let col_lines = &mut [String::from("a"), String::from("b")];
        let max_lines = 4;
        let blank_char = '.';

        let line_index = 0;
        let result = col_line(
            h_align, v_align, col_width, col_lines, max_lines, line_index, blank_char,
        );
        let expected = "a..";
        assert_eq!(result, expected);

        let line_index = 1;
        let result = col_line(
            h_align, v_align, col_width, col_lines, max_lines, line_index, blank_char,
        );
        let expected = "b..";
        assert_eq!(result, expected);

        let line_index = 2;
        let result = col_line(
            h_align, v_align, col_width, col_lines, max_lines, line_index, blank_char,
        );
        let expected = "...";
        assert_eq!(result, expected);

        let line_index = 3;
        let result = col_line(
            h_align, v_align, col_width, col_lines, max_lines, line_index, blank_char,
        );
        let expected = "...";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_col_line_for_a_content_with_2_row_in_a_4_lines_column_left_middle() {
        let h_align = HAlign::Left;
        let v_align = VAlign::Middle;
        let col_width = 3;
        let col_lines = &mut [String::from("a"), String::from("b")];
        let max_lines = 4;
        let blank_char = '.';

        let line_index = 0;
        let result = col_line(
            h_align, v_align, col_width, col_lines, max_lines, line_index, blank_char,
        );
        let expected = "...";
        assert_eq!(result, expected);

        let line_index = 1;
        let result = col_line(
            h_align, v_align, col_width, col_lines, max_lines, line_index, blank_char,
        );
        let expected = "a..";
        assert_eq!(result, expected);

        let line_index = 2;
        let result = col_line(
            h_align, v_align, col_width, col_lines, max_lines, line_index, blank_char,
        );
        let expected = "b..";
        assert_eq!(result, expected);

        let line_index = 3;
        let result = col_line(
            h_align, v_align, col_width, col_lines, max_lines, line_index, blank_char,
        );
        let expected = "...";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_col_line_for_a_content_with_2_row_in_a_4_lines_column_left_bottom() {
        let h_align = HAlign::Left;
        let v_align = VAlign::Bottom;
        let col_width = 3;
        let col_lines = &mut [String::from("a"), String::from("b")];
        let max_lines = 4;
        let blank_char = '.';

        let line_index = 0;
        let result = col_line(
            h_align, v_align, col_width, col_lines, max_lines, line_index, blank_char,
        );
        let expected = "...";
        assert_eq!(result, expected);

        let line_index = 1;
        let result = col_line(
            h_align, v_align, col_width, col_lines, max_lines, line_index, blank_char,
        );
        let expected = "...";
        assert_eq!(result, expected);

        let line_index = 2;
        let result = col_line(
            h_align, v_align, col_width, col_lines, max_lines, line_index, blank_char,
        );
        let expected = "a..";
        assert_eq!(result, expected);

        let line_index = 3;
        let result = col_line(
            h_align, v_align, col_width, col_lines, max_lines, line_index, blank_char,
        );
        let expected = "b..";
        assert_eq!(result, expected);
    }
}
