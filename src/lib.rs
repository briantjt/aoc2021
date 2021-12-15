pub fn is_out_of_bounds(
    row: usize,
    col: usize,
    row_d: usize,
    col_d: usize,
    row_length: usize,
    col_length: usize,
) -> bool {
    let row_out_of_bounds =
        (row == 0 && row_d == row_length - 1) || (row == row_length - 1 && row_d == 1);
    let col_out_of_bounds =
        (col == 0 && col_d == col_length - 1) || (col == col_length - 1 && col_d == 1);
    row_out_of_bounds || col_out_of_bounds
}
