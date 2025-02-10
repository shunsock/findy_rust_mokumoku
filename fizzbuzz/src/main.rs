fn main() {
    fizzbuzz_runtime();
}

fn fizzbuzz_runtime() {
    for i in 1..30 {
        println!("{}", fizzbuzz_generator(i));
    }
}

fn fizzbuzz_generator(x: u8) -> String {
    let mod5 = x % 5 == 0;
    let mod3 = x % 3 == 0;
    match (mod5, mod3) {
        (true, true) => "FizzBuzz".to_string(),
        (true, false) => "Buzz".to_string(),
        (false, true) => "Fizz".to_string(),
        (false, false) => x.to_string(),
    }
}

