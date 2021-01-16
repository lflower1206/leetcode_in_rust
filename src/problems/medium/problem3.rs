use std::cmp;
use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max_langth = 0;
    let mut map_char_index: HashMap<&u8, usize> = HashMap::new();
    let mut index_start: usize = 0;
    let chars = s.into_bytes();

    for (index_end, char) in chars.iter().enumerate() {
        if let Some(index_prev) = map_char_index.get(char) {
            index_start = cmp::max(index_start, *index_prev);
        }

        max_langth = cmp::max(max_langth, index_end - index_start + 1);
        map_char_index.insert(char, index_end + 1);
    }

    max_langth as i32
}
