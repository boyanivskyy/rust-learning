use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io, path::Path};

pub fn hash_password(password: &str) -> String {
    use sha2::Digest;

    let mut hasher = sha2::Sha256::new();
    hasher.update(password);

    format!("{:X}", hasher.finalize())
}

pub fn greet_user(name: &str) -> String {
    format!("Welcome {name}")
}

#[derive(PartialEq, Debug)]
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
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        Self {
            username: username.to_lowercase(),
            password: hash_password(password),
            role,
        }
    }
}

pub fn get_default_users() -> HashMap<String, User> {
    let mut users_hashmap = HashMap::new();

    users_hashmap.insert(
        "admin".to_string(),
        User::new("admin", "password", LoginRole::Admin),
    );
    users_hashmap.insert(
        "boyanivskyy".to_string(),
        User::new("boyanivskyy", "password", LoginRole::User),
    );

    users_hashmap
}

pub fn get_users() -> HashMap<String, User> {
    let users_path = Path::new("users.json");
    if users_path.exists() {
        // Load the file!
        let users_json = fs::read_to_string(users_path).unwrap();
        let users: HashMap<String, User> = serde_json::from_str(&users_json).unwrap();
        users
    } else {
        let users = get_default_users();
        let users_json = serde_json::to_string(&users).unwrap();
        fs::write(users_path, users_json).unwrap();
        users
    }
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();
    let password_hash = hash_password(password);

    // NOTE: approach with vector
    // let users = get_users();
    // if let Some(user) = users.iter().find(|user| user.username == username) {

    // NOTE: approach with hash map
    let users = get_users();
    if let Some(user) = users.get(&username) {
        if user.password == password_hash {
            return Some(LoginAction::Granted(user.role.clone()));
        } else {
            return Some(LoginAction::Denied);
        }
    }

    return None;
}

pub fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Std in is not working");

    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    // NOTE: super is the module name and ::* to include everything from the code above(some scope)
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!("Welcome Vitaliy", greet_user("Vitaliy"))
    }

    #[test]
    fn test_login() {
        assert_eq!(
            login("admin", "password"),
            Some(LoginAction::Granted(LoginRole::Admin))
        );
        assert_eq!(
            login("boyanivskyy", "password"),
            Some(LoginAction::Granted(LoginRole::User))
        );
        assert_eq!(login("random", "credentials"), None);
    }
}
