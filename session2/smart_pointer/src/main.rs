use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

struct Point{
    x: i32,
    y: i32
}

fn new_thread(){
    let p = Arc::new(Point {x:1, y:1});

    let p1 = Arc::clone(&p);
    let _h1 =thread::spawn(move || {
        println!("{}, {}", p1.x, p1.y);
    }).join();
    
    let p2 = Arc::clone(&p);
    let _h2 = thread::spawn(move || {
        println!("{}, {}", p2.x, p2.y);
    }).join();

}

fn arc_mut(){
    let p = Arc::new(Mutex::new(Point {x:1, y:1}));

    let p1 = Arc::clone(&p);
    let _h1 = thread::spawn(move || {
        let mut p1_t = p1.lock().unwrap();
        p1_t.x += 1; 
        println!("{}, {}", p1_t.x, p1_t.y);
    }).join();
    
    let p2 = Arc::clone(&p);
    let _h2 = thread::spawn(move || {
        let mut p2_t = p2.lock().unwrap();
        p2_t.x += 1;
        println!("{}, {}", p2_t.x, p2_t.y);
    }).join();
}


fn main() {
    // Box smart point(move data to heap)
    let b = Box::new(Point {x:0, y:0});
    println!("Box: {}, {}", b.x, b.y);

    // Reference count, read only, single-threaded
    let p = Rc::new(Point {x:1, y:1});
    let p1 = Rc::clone(&p);
    let p2 = Rc::clone(&p);

    println!("Rc: {}, {}, {}", p.x, p1.x, p2.x);

    println!("Now is Arc");
    new_thread();

    // Mutex, 
    let p_mutex = Mutex::new(Point {x:2, y:2});
    let mut p_new = p_mutex.lock().unwrap();
    p_new.x += 1;
    
    println!("Mutex: {}, {}", p_new.x, p_new.y);

    println!("Now is Arc add Mut");
    arc_mut();

}
