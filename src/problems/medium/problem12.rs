pub fn int_to_roman(num: i32) -> String {
    let values = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let symbols = vec![
        "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
    ];

    let mut input = num;
    let mut result = String::new();

    for (index, value) in values.iter().enumerate() {
        while input > 0 && input >= *value {
            input = input - value;
            result.push_str(symbols.get(index).unwrap())
        }
    }

    result
}

#[cfg(test)]
mod problem12 {
    use crate::problems::medium::problem12::int_to_roman;

    #[test]
    fn it_should_return_i() {
        assert_eq!(int_to_roman(1), String::from("I"))
    }

    #[test]
    fn it_should_return_v() {
        assert_eq!(int_to_roman(5), String::from("V"))
    }

    #[test]
    fn it_should_return_x() {
        assert_eq!(int_to_roman(10), String::from("X"))
    }

    #[test]
    fn it_should_return_iii() {
        assert_eq!(int_to_roman(3), String::from("III"))
    }

    #[test]
    fn it_should_returns_mcmxciv() {
        assert_eq!(int_to_roman(1994), String::from("MCMXCIV"))
    }
}
