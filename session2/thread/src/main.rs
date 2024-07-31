
fn hello_from_thread(n: u32){
    println!("Hello from a thread {n}");
}

fn do_math(i: u32) -> u32 {
    let mut n = i+1;
    for _ in 0..5{
        n *= 2;
    }
    n
}


fn main() {
    println!("Hello, world!");

    let mut thread_handles = Vec::new();

    for i in 0..5 {
        let thread_handle = std::thread::spawn(move || do_math(i));
        thread_handles.push(thread_handle);
    }

    thread_handles.into_iter().for_each(|handle| {
        println!("{}",handle.join().unwrap());
    });
}

 