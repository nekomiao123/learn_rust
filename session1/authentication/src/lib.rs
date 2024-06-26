use std::{collections::HashMap, fs::read_to_string};
use std::path::Path;
use serde::{Serialize, Deserialize};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greet_user(name: &str) -> String{
    format!("Hello, {}!", name)
}


#[derive(PartialEq, Debug, Clone)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User{
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User{
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        User { 
            username: username.to_lowercase(),
            password: hash_password(password), 
            role
        }
    }
}

// pub fn get_users() -> [User; 2] {
//     [
//         User::new("admin", "password", LoginRole::Admin),
//         User::new("alice", "password", LoginRole::User),
//     ]
// }

// pub fn get_users() -> Vec<User> {
//     vec![
//         User::new("admin", "password", LoginRole::Admin),
//         User::new("alice", "password", LoginRole::User),
//     ]
// }

pub fn hash_password(password: &str) -> String{
    use sha2::{Digest, Sha256};
    let mut hahser = Sha256::new();
    hahser.update(password);
    format!("{:X}", hahser.finalize())

}

pub fn get_default_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert("admin".to_string(), User::new("admin", "password", LoginRole::Admin));
    users.insert("alice".to_string(), User::new("alice", "password", LoginRole::User));
    users
}

pub fn get_users() -> HashMap<String, User> {
    let users_path = Path::new("users.json");
    if users_path.exists() {
        // Load the file
        let users_json = read_to_string(users_path).unwrap();
        let users: HashMap<String, User> = serde_json::from_str(&users_json).unwrap();
        users
    } else {
        // Create a file and return it 
        let users = get_default_users();
        let users_json = serde_json::to_string(&users).unwrap();
        std::fs::write(users_path, users_json).unwrap();
        users
    }
}

// fn test_admin_vec() {
//     let users: Vec<String> = get_users()
//     .into_iter()
//     .filter(|u| u.role == LoginRole::Admin)
//     .map(|u| u.username)
//     .collect();
// }

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();
    let password = hash_password(password);
    let users = get_users();

    if let Some(user) = users.get(&username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        } else {
            return Some(LoginAction::Denied);
        }
    }
    None

    // if let Some(user) = users.iter().find(|user| user.username == username) {
    //     if user.password == password {
    //         return Some(LoginAction::Granted(user.role.clone()));
    //     } else {
    //         return Some(LoginAction::Denied);
    //     }
    // } 
    // None

    // let username = username.to_lowercase();
    // if username != "admin" && username != "alice" {
    //     return None;
    // }

    // if username == "admin" && password == "password" {
    //     Some(LoginAction::Granted(LoginRole::Admin))
    // } else if username == "alice" && password == "password" {
    //     Some(LoginAction::Granted(LoginRole::User))
    // } else {
    //     Some(LoginAction::Denied)
    // }
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

