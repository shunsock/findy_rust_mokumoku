fn main() {
    fizzbuzz();
}

fn fizzbuzz() {
    for i in 0..30 {
        if i == 0 {
            println!("{}", i)
        } else if i % 3 == 0 && i % 5 == 0  {
            println!("FizzBuzz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{}", i)
        }
    }
}

