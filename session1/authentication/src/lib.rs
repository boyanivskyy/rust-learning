pub fn greet_user(name: &str) -> String {
    format!("Welcome {name}")
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginRole {
    Admin,
    User
}

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
            role
        }
    }
}

pub fn get_users() -> [User; 2] {
    [
        User::new("admin", "password", LoginRole::Admin),
        User::new("boyanivskyy", "password", LoginRole::User)
    ]
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();
    let users = get_users();

    if let Some(user) = users.iter().find(|user| user.username == username) {
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
    std::io::stdin().read_line(&mut input).expect("Std in is not working");

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
        assert_eq!(login("admin", "password"), Some(LoginAction::Granted(LoginRole::Admin)));
        assert_eq!(login("boyanivskyy", "password"), Some(LoginAction::Granted(LoginRole::User)));
        assert_eq!(login("random", "credentials"), None);
    }

}
