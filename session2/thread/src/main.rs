
fn hello_from_thread(){
    println!("Hello from a thread!");
}

fn main() {
    println!("Hello, world!");
    let thread_handle = std::thread::spawn(hello_from_thread);
    thread_handle.join().unwrap();
}

