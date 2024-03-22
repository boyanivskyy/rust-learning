pub fn greet_user(name: &str) -> String {
    format!("Welcome {name}")
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied
}

#[derive(PartialEq, Debug)]
pub enum LoginRole {
    Admin,
    User
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();

    if username != "admin" && username != "boyanivskyy" {
        return None;
    }

    if username == "admin" && password == "password" {
        Some(LoginAction::Granted(LoginRole::Admin))
    } else if username == "boyanivskyy" && password == "password" {
        Some(LoginAction::Granted(LoginRole::User))
    } else {
        Some(LoginAction::Denied)
    }
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
