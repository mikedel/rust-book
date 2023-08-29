fn main() {
    fn fib(n: i32) -> i32 {
        if n == 1 || n == 2 {
            return 1;
        }
        return fib(n-1) + fib(n-2);
    }

    println!("1: { }", fib(1) );
    println!("2: { }", fib(2) );
    println!("3: { }", fib(3) );
    println!("4: { }", fib(4) );
    println!("5: { }", fib(5) );
    println!("6: { }", fib(6) );
}
