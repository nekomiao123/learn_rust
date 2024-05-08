fn double(n:i32) -> i32{
   n * 2
}

fn double_or_nothing(n:i32) -> i32{
    if n > 0{
        return n * 2;
    }
    0
}

fn greet(s: String){
    println!("Hello {}", s);
}

fn greet_borrow(s: &String){
    println!("Hello {}", s);
}

fn greet_borrow_mut(s: &mut String){
    // s.push_str(" World");
    *s = format!("{} World", s);
}

fn main() {
    let n:i32 = 5;
    let mut m = 2;
    m += 1;
    {
        let n = 1;
        println!("{}", n);
    }
    println!("{}", n);
    println!("{}", m);

    println!("{}", double(5));
    println!("{}", double_or_nothing(5));
    println!("{}", double_or_nothing(-5));

    let a = if n > 0 {
        n * 2
    } else {
        0
    };
    println!("a is {}", a);

    let name = "Hello".to_string();
    greet(name.clone());
    // greet(name);
    greet_borrow(&name);
    greet_borrow(&name);

    let mut name2 = "Hello".to_string();
    greet_borrow_mut(&mut name2);
    println!("{}", name2);
}
