fn double(n:i32) -> i32{
   n * 2
}

fn double_or_nothing(n:i32) -> i32{
    if n > 0{
        return n * 2;
    }
    0
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
}
