use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let mut symbol_map = HashMap::with_capacity(7);
    symbol_map.insert('I', 1);
    symbol_map.insert('V', 5);
    symbol_map.insert('X', 10);
    symbol_map.insert('L', 50);
    symbol_map.insert('C', 100);
    symbol_map.insert('D', 500);
    symbol_map.insert('M', 1000);

    let chars = s.chars();
    let mut result = 0;
    let mut prev: &i32 = &1000;

    for (_index, c) in chars.enumerate() {
        let current = symbol_map.get(&c).unwrap();

        if current > prev {
            result = result + current - 2 * prev;
        } else {
            result = result + current;
        }

        prev = current;
    }

    result
}

#[cfg(test)]
mod problem13 {
    use crate::problems::easy::problem13::roman_to_int;

    #[test]
    fn it_should_return_1() {
        assert_eq!(roman_to_int(String::from("I")), 1)
    }

    #[test]
    fn it_should_return_5() {
        assert_eq!(roman_to_int(String::from("V")), 5)
    }

    #[test]
    fn it_should_return_2() {
        assert_eq!(roman_to_int(String::from("II")), 2)
    }

    #[test]
    fn it_should_return_4() {
        assert_eq!(roman_to_int(String::from("IV")), 4)
    }

    #[test]
    fn it_should_return_1994() {
        assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994)
    }
}
