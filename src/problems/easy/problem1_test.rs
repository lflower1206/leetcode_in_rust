#[cfg(test)]
mod problem1 {
    use crate::problems::easy::problem1::two_sum;

    #[test]
    fn it_works() {
        assert_eq!(two_sum(vec![3,2,4], 6), vec![1, 2]);
    }
}