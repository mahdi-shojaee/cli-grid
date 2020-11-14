use cli_grid::{Cell, Grid, HAlign, Row, VAlign};

#[test]
fn test_grid_1x1() {
    let grid = Grid::builder(vec![Row::new(vec![Cell::new("1".into(), 1)])])
        .default_h_align(HAlign::Left)
        .default_v_align(VAlign::Top)
        .default_blank_char('.')
        .column_width(3)
        .build();

    let result = grid.to_string();

    #[rustfmt::skip]
    let expected = format!(
        "{}\n",
        "1.."
    );

    assert_eq!(result, expected);
}

#[test]
fn test_grid_2x2_with_padding_0() {
    let grid = Grid::builder(vec![
        Row::new(vec![Cell::new("1".into(), 1), Cell::new("1".into(), 1)]),
        Row::new(vec![Cell::new("1".into(), 1), Cell::new("1".into(), 1)]),
    ])
    .default_h_align(HAlign::Left)
    .default_v_align(VAlign::Top)
    .default_blank_char('.')
    .column_width(3)
    .padding_size(0)
    .build();

    let result = grid.to_string();

    #[rustfmt::skip]
    let expected = format!(
        "{}\n{}\n",
        "1..1..",
        "1..1.."
    );

    assert_eq!(result, expected);
}

#[test]
fn test_grid_2x2_with_padding_1() {
    let grid = Grid::builder(vec![
        Row::new(vec![Cell::new("1".into(), 1), Cell::new("1".into(), 1)]),
        Row::new(vec![Cell::new("1".into(), 1), Cell::new("1".into(), 1)]),
    ])
    .default_h_align(HAlign::Left)
    .default_v_align(VAlign::Top)
    .default_blank_char('.')
    .column_width(3)
    .build();

    let result = grid.to_string();

    #[rustfmt::skip]
    let expected = format!(
        "{}\n{}\n",
        "1.. 1..",
        "1.. 1.."
    );

    assert_eq!(result, expected);
}

#[test]
fn test_grid_3x3_center_top_with_empty_cell() {
    let grid = Grid::builder(vec![
        Row::new(vec![
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
        ]),
        Row::new(vec![
            Cell::new("1".into(), 1),
            Cell::new_empty(1),
            Cell::new("1".into(), 1),
        ]),
        Row::new(vec![
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
        ]),
    ])
    .default_h_align(HAlign::Center)
    .default_v_align(VAlign::Top)
    .default_blank_char('.')
    .column_width(6)
    .build();

    let result = grid.to_string();

    #[rustfmt::skip]
    let expected = format!(
        "{}\n{}\n{}\n",
        "..1... ..1... ..1...",
        "..1... ...... ..1...",
        "..1... ..1... ..1...",
    );

    assert_eq!(result, expected);
}

#[test]
fn test_grid_3x3_multi_line_center_top() {
    let grid = Grid::builder(vec![
        Row::new(vec![
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
        ]),
        Row::new(vec![
            Cell::new("1".into(), 1),
            Cell::new("1\n111\n1".into(), 1),
            Cell::new("1".into(), 1),
        ]),
        Row::new(vec![
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
        ]),
    ])
    .default_h_align(HAlign::Center)
    .default_v_align(VAlign::Top)
    .default_blank_char('.')
    .column_width(6)
    .build();

    let result = grid.to_string();

    #[rustfmt::skip]
    let expected = format!(
        "{}\n{}\n{}\n{}\n{}\n",
        "..1... ..1... ..1...",
        "..1... ..1... ..1...",
        "...... .111.. ......",
        "...... ..1... ......",
        "..1... ..1... ..1...",
    );

    assert_eq!(result, expected);
}

#[test]
fn test_grid_3x3_multi_line_fill_top() {
    let grid = Grid::builder(vec![
        Row::new(vec![
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
        ]),
        Row::new(vec![
            Cell::new("1".into(), 1),
            Cell::builder("1\nabc\n1".into(), 1)
                .h_align(HAlign::Fill)
                .build(),
            Cell::new("1".into(), 1),
        ]),
        Row::new(vec![
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
        ]),
    ])
    .default_h_align(HAlign::Center)
    .default_v_align(VAlign::Top)
    .default_blank_char('.')
    .column_width(6)
    .build();

    let result = grid.to_string();

    #[rustfmt::skip]
    let expected = format!(
        "{}\n{}\n{}\n{}\n{}\n",
        "..1... ..1... ..1...",
        "..1... 111111 ..1...",
        "...... abcabc ......",
        "...... 111111 ......",
        "..1... ..1... ..1...",
    );

    assert_eq!(result, expected);
}

#[test]
fn test_grid_3x3_multi_line_center_middle() {
    let grid = Grid::builder(vec![
        Row::new(vec![
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
        ]),
        Row::new(vec![
            Cell::new("1".into(), 1),
            Cell::new("1\n111\n1".into(), 1),
            Cell::new("1".into(), 1),
        ]),
        Row::new(vec![
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
        ]),
    ])
    .default_h_align(HAlign::Center)
    .default_v_align(VAlign::Middle)
    .default_blank_char('.')
    .column_width(6)
    .build();

    let result = grid.to_string();

    #[rustfmt::skip]
    let expected = format!(
        "{}\n{}\n{}\n{}\n{}\n",
        "..1... ..1... ..1...",
        "...... ..1... ......",
        "..1... .111.. ..1...",
        "...... ..1... ......",
        "..1... ..1... ..1...",
    );

    assert_eq!(result, expected);
}

#[test]
fn test_grid_3x3_multi_line_center_bottom() {
    let grid = Grid::builder(vec![
        Row::new(vec![
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
        ]),
        Row::new(vec![
            Cell::new("1".into(), 1),
            Cell::new("1\n111\n1".into(), 1),
            Cell::new("1".into(), 1),
        ]),
        Row::new(vec![
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
        ]),
    ])
    .default_h_align(HAlign::Center)
    .default_v_align(VAlign::Bottom)
    .default_blank_char('.')
    .column_width(6)
    .build();

    let result = grid.to_string();

    #[rustfmt::skip]
    let expected = format!(
        "{}\n{}\n{}\n{}\n{}\n",
        "..1... ..1... ..1...",
        "...... ..1... ......",
        "...... .111.. ......",
        "..1... ..1... ..1...",
        "..1... ..1... ..1...",
    );

    assert_eq!(result, expected);
}

#[test]
fn test_grid_3x3_center_top_with_different_col_spans1() {
    let grid = Grid::builder(vec![
        Row::new(vec![
            Cell::new("1".into(), 1),
            Cell::new("6".into(), 6),
            Cell::new("1".into(), 1),
        ]),
        Row::new(vec![
            Cell::new("2".into(), 2),
            Cell::new("4".into(), 4),
            Cell::new("2".into(), 2),
        ]),
        Row::new(vec![
            Cell::new("3".into(), 3),
            Cell::new("2".into(), 2),
            Cell::new("3".into(), 3),
        ]),
    ])
    .default_h_align(HAlign::Center)
    .default_v_align(VAlign::Top)
    .default_blank_char('.')
    .column_width(3)
    .build();

    let result = grid.to_string();

    #[rustfmt::skip]
    let expected = format!(
        "{}\n{}\n{}\n",
        ".1. ...........6........... .1.",
        "...2... .......4....... ...2...",
        ".....3..... ...2... .....3.....",
    );

    assert_eq!(result, expected);
}

#[test]
fn test_grid_3x3_center_top_with_different_col_spans2() {
    let grid = Grid::builder(vec![
        Row::new(vec![Cell::new("3".into(), 3)]),
        Row::new(vec![Cell::new("1".into(), 1), Cell::new("2".into(), 2)]),
        Row::new(vec![
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
        ]),
    ])
    .default_h_align(HAlign::Center)
    .default_v_align(VAlign::Top)
    .default_blank_char('.')
    .column_width(3)
    .build();

    let result = grid.to_string();

    #[rustfmt::skip]
    let expected = format!(
        "{}\n{}\n{}\n",
        ".....3.....",
        ".1. ...2...",
        ".1. .1. .1.",
    );

    assert_eq!(result, expected);
}

#[test]
fn test_nested_grids_3x3_multi_line_center_middle() {
    let inner_grid = Grid::builder(vec![
        Row::new(vec![Cell::new("1".into(), 1), Cell::new("1".into(), 1)]),
        Row::new(vec![Cell::new("1".into(), 1), Cell::new("1".into(), 1)]),
        Row::new(vec![Cell::new("1".into(), 1), Cell::new("1".into(), 1)]),
    ])
    .default_h_align(HAlign::Center)
    .default_blank_char('-')
    .column_width(3)
    .build();
    let inner_grid_str = inner_grid.to_string();
    let grid = Grid::builder(vec![
        Row::new(vec![
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
        ]),
        Row::new(vec![
            Cell::new("1".into(), 1),
            Cell::new(inner_grid_str.to_string().into(), 1),
            Cell::new("1".into(), 1),
        ]),
        Row::new(vec![
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
            Cell::new("1".into(), 1),
        ]),
    ])
    .default_h_align(HAlign::Center)
    .default_v_align(VAlign::Middle)
    .default_blank_char('.')
    .column_width(13)
    .build();

    let result = grid.to_string();

    #[rustfmt::skip]
    let expected = format!(
        "{}\n{}\n{}\n{}\n{}\n",
        "......1...... ......1...... ......1......",
        "............. ...-1- -1-... .............",
        "......1...... ...-1- -1-... ......1......",
        "............. ...-1- -1-... .............",
        "......1...... ......1...... ......1......",
    );

    assert_eq!(result, expected);
}
