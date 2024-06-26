pub fn add(a: usize, b: usize) -> usize {
    b + a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn it_doesnt_work() {
        let result = add(3, 1);
        assert_ne!(result, 4);
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("falla")
    }

    #[test]
    fn results() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

}
