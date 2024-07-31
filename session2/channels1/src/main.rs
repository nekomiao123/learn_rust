use std::sync::mpsc;

enum Command {
    SayHello,
    Quit
}

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();

    let handle = std::thread::spawn(move || {
        while let Ok(c) = rx.recv() {
            match c {
                Command::SayHello => println!("Hello!"),
                Command::Quit => {
                    println!("Quitting!");
                    break;
                }
            }
        }
    });

    for _ in 0..5 {
        tx.send(Command::SayHello).unwrap();
    }
    println!("Sending quit command");
    tx.send(Command::Quit).unwrap();
    handle.join().unwrap();
    
}
