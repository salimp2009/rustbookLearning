pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn get() -> usize {
    100 // arbitrary number for testing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
