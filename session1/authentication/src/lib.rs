pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greet_user(name: &str) -> String{
    format!("Hello, {}!", name)
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
    fn test_greet_user() {
        let result = greet_user("Alice");
        assert_eq!(result, "Hello, Alice!");
    }
}

