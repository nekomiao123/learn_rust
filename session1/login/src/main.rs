use authentication::{login, read_line, LoginAction, LoginRole};


fn main() {
    let mut tries = 0;
    loop{
        println!("Enter your username:");
        let username = read_line();
        println!("Enter your password:");
        let password = read_line();

        match login(&username, &password) {
            Some(LoginAction::Granted(role)) => {
                match role {
                    LoginRole::Admin => println!("Welcome Admin"),
                    LoginRole::User => println!("Welcome User"),
                }
                break;
            }
            Some(LoginAction::Denied) =>{
                println!("Wrong password\n");
            }
            None => {
                println!("It seems we have new user")
            }
        }
        tries += 1;
        if tries >= 3 {
            println!("Too many tries!\n");
            break;
        }

        // if login(&username, &password) {
        //     println!("Welcome!");
        //     break;
        // } else {
        //     println!("Wrong username or password\n");
        //     tries += 1;
        //     if tries >= 3 {
        //         println!("Too many tries!\n");
        //         break;
        //     }
        // }
        
    }
}


