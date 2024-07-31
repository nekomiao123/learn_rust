use std::thread;

fn my_thread() {
    println!("Hello from a thread named {}",
        thread::current().name().unwrap()
    );
}

fn main() {
    println!("Hello from the main thread");
    let handle = thread::Builder::new()
        .name("ABC thread".to_string())
        .spawn(my_thread)
        .unwrap();

    handle.join().unwrap();
}
