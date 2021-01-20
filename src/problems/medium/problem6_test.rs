#[cfg(test)]
mod problem6 {
    use crate::problems::medium::problem6::convert;

    #[test]
    fn it_should_return_itself() {
        assert_eq!(convert(String::from("a"), 3), String::from("a"))
    }

    #[test]
    fn it_should_return_pahnaplsiigyir() {
        // PAHNAPLSIIGYIR
        assert_eq!(
            convert(String::from("paypalishiring"), 3),
            String::from("pahnaplsiigyir")
        )
    }

    #[test]
    fn it_should_return_ab() {
        assert_eq!(convert(String::from("ab"), 1), String::from("ab"))
    }
}
