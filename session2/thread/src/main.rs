
fn hello_from_thread(n: u32){
    println!("Hello from a thread {n}");
}

fn main() {
    println!("Hello, world!");

    let mut thread_handles = Vec::new();

    for i in 0..5 {
        let thread_handle = std::thread::spawn(move || hello_from_thread(i));
        thread_handles.push(thread_handle);
    }
    thread_handles.into_iter().for_each(|handle| handle.join().unwrap());
}

