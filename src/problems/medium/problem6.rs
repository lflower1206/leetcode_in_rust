pub fn convert(s: String, num_rows: i32) -> String {
    if s.len() <= 2 {
        return s;
    }

    let input = s.into_bytes();
    let len = input.len();
    let num_of_rows = num_rows as usize;
    let offset: usize = if num_of_rows == 1 {
        1
    } else {
        2 * num_of_rows - 2
    };

    let mut result: Vec<char> = Vec::with_capacity(len);

    for index_row in 0usize..num_of_rows {
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

            index_column = index_next_column;
        }
    }
    result.iter().collect::<String>()
}
