pub fn convert(s: String, num_rows: i32) -> String {
    if s.len() <= 1 {
        return s;
    }

    let input = s.into_bytes();
    let len = input.len();
    let num_of_rows = num_rows as usize;
    let offset: usize = 2 * num_of_rows - 2;

    let mut result: Vec<char> = Vec::with_capacity(len);
    let mut index_row: usize = 0;

    while index_row < num_of_rows {
        let mut index_column: usize = index_row;

        while index_column < len {
            let index_next_column = index_column + offset;
            result.push(input[index_column] as char);

            if index_row != 0 && index_row != num_of_rows - 1 {
                let index_between_columns = index_next_column - 2 * index_row;

                if let Some(char) = input.get(index_between_columns) {
                    result.push(*char as char);
                }
            }

            if offset == 0 {
                index_column = index_column + 1;
            } else {
                index_column = index_next_column;
            }
        }
        index_row = index_row + 1
    }
    result.iter().collect::<String>()
}
