use authentication::{greet_user, login, read_line, LoginAction, LoginRole};

fn main() {
    let mut tries = 0;
    loop {

        println!("Enter your username:");
        let username = read_line();

        println!("Enter your password:");
        let password = read_line();

        match login(&username, &password) {
            Some(LoginAction::Granted(role)) => {
                match role {
                    LoginRole::Admin => println!("Admin!"),
                    LoginRole::User => println!("User!")
                }
                
                greet_user(&username);

                break;
            }
            Some(LoginAction::Denied) => { println!("Access Denid"); }
            None => { println!("None access"); }
        }

        tries += 1;
        if tries >= 3 {
            println!("Too many failed attempts");
            break;
        }
    }
}
