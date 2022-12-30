fn main() {
    println!("Hello, world!");
    let mut i = 0;
    while i < 10000 {
        println!("fibonacci({}) = {}", i, fibonacci(i));
        i += 1;
    }
}


fn fibonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}