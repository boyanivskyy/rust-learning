use std::collections::HashMap;

pub fn greet_user(name: &str) -> String {
    format!("Welcome {name}")
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        Self {
            username: username.to_lowercase(),
            password: password.to_string(),
            role,
        }
    }
}

pub fn get_users() -> Vec<User> {
    // let mut users = vec![];
    // users.push(User::new("admin", "password", LoginRole::Admin));

    vec![
        User::new("admin", "password", LoginRole::Admin),
        User::new("boyanivskyy", "password", LoginRole::User),
    ]
}

fn get_admin_usernames() -> Vec<String> {
    let users: Vec<String> = get_users()
        .into_iter()
        .filter(|u| u.role == LoginRole::Admin)
        .map(|u| u.username)
        .collect();

    users
}

fn get_users_hashmap() -> HashMap<String, User> {
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

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();

    // NOTE: approach with vector
    // let users = get_users();
    // if let Some(user) = users.iter().find(|user| user.username == username) {

    // NOTE: approach with hash map
    let users = get_users_hashmap();
    if let Some(user) = users.get(&username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        } else {
            return Some(LoginAction::Denied);
        }
    }

    return None;
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
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
