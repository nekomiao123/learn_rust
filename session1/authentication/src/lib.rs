pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greet_user(name: &str) -> String{
    format!("Hello, {}!", name)
}


#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug)]
pub enum LoginRole {
    Admin,
    User,
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();

    if username != "admin" && username != "alice" {
        return None;
    }

    if username == "admin" && password == "password" {
        Some(LoginAction::Granted(LoginRole::Admin))
    } else if username == "alice" && password == "password" {
        Some(LoginAction::Granted(LoginRole::User))
    } else {
        Some(LoginAction::Denied)
    }
}

pub fn read_line() -> String{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
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
    #[test]
    fn test_login() {
        assert_eq!(login("admin", "password"), Some(LoginAction::Granted(LoginRole::Admin)));
        assert_eq!(login("alice", "password"), Some(LoginAction::Granted(LoginRole::User)));
        assert_eq!(login("Alice", "wrong"), Some(LoginAction::Denied));
    }
}

