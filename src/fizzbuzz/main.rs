pub fn main() {
    for i in 1..101 {
        let mut done = false;
        if i % 3 == 0 {
            print!("Fizz");
            done = true;
        }
        if i % 5 == 0 {
            print!("Buzz ");
            continue;
        }
        if !done {
            print!("{}", i);
        }
        print!(" ");
    }
    print!("\n");
}
